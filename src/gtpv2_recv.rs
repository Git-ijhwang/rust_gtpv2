use std::f32::consts::E;
use std::hash::Hash;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket, SocketAddr};
use std::ptr::slice_from_raw_parts;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use core::result::Result;
use std::{clone, thread};
use std::collections::HashMap;
use log::{debug, error, info, trace, warn};

use crate::gtp_dictionary::{self, *};
use crate::{find_pkt_in_queue, session, validate_gtpv2::*};
use crate::gtpv2_type::*;
use crate::gtp_msg::*;
use std::io::Error;
use crate::peers::*;
use crate::session::*;
use crate::ippool::*;

const BUFSIZ: usize = 8192;

#[derive(Debug)]
struct Gtpv2Peer {
    ipaddr: u32,
    status: u8,
}


#[derive(Debug, Clone)]
pub struct BearerQos {
    flag:   u8,
    qci:    u8,
    mbr_ul: u32,
    mbr_dl: u32,
    gbr_ul: u32,
    gbr_dl: u32,
}

impl BearerQos {
    pub fn new() -> Self {
        BearerQos {
            flag:   0,
            qci:    0,
            mbr_ul: 0,
            mbr_dl: 0,
            gbr_ul: 0,
            gbr_dl: 0,
        }
    }
}

// const INSTANCE_FOR_S1U_ENB_FTEID: u8 =		0;
// const INSTANCE_FOR_S4U_SGSN_FTEID: u8 =		1;
// const INSTANCE_FOR_S5S8U_SGW_FTEID: u8 =	2;
// const INSTANCE_FOR_S5S8U_PGW_FTEID: u8 =	3;
// const INSTANCE_FOR_S12_RNC_FTEID: u8 =		4;
// const INSTANCE_FOR_S2B_EPDG_FTEID: u8 =		5;
// const INSTANCE_FOR_S2A_TWAN_FTEID: u8 =		6;
// const INSTANCE_FOR_S11U_MME_FTEID: u8 =		7;

#[derive(Debug, Clone, Copy)]
pub struct BearerTFT {

}
#[derive(Debug, Clone, Copy)]
pub struct FTeid {
	pub used: bool,
	pub iface_type: u8,
	pub teid: u32,
	pub addr: Ipv4Addr,
}
impl FTeid {
	fn new() -> Self {
		FTeid {
			used:	false,
			iface_type: 0,
			teid: 0,
			addr: Ipv4Addr::new(0,0,0,0),
		}
	}
}

#[derive(Debug, Clone)]
pub struct BearerCnxt {
	pub ebi: u8,
	pub iface_info: [FTeid;8],
	pub bearer_qos: BearerQos,
}

impl BearerCnxt {
	pub fn new() -> Self {
		BearerCnxt {
			ebi: 0,
			iface_info: [FTeid::new();8],
			bearer_qos: BearerQos::new(),
		}
	}
}


// #[derive(Debug, Clone, Copy)]
struct Gtpv2IeDetailInfo {
    ie_type:        u8,
    ie_min_length:  u8,
    ie_max_length:  u8,
    reserved:       u8,
    ie_instance:    u8,
    ie_presence:    u8,
    group_ie_info:  Option<Box<Gtpv2IeInfo>>, // Pointer to group IE info, optional to handle null pointers
}

#[derive(Debug, Clone, Copy)]
struct Gtpv2IeInfo {
    msg_type:       u8,     // Message Type
    mie_count:      usize,  // Mandatory IE Count
    // ie_parse_info: Vec<Vec<IeParseDetails>>,
    // ie_detail_info: HashMap<(u8, u8), Gtpv2IeDetailInfo>,
}

impl Gtpv2IeInfo {
    fn new (msg_type: u8) -> Self {
        Self {
            msg_type,
            mie_count:      0,
            // ie_detail_info: HashMap::new(),
        }
    }

    fn add_ie_parse_info(&mut self, msg_type: u8,
        min_len: u8, max_len: u8, received: u8,
        ie_instance: u8, ie_presence: u8,
        // gie_info: &[Gtpv2IeInfo]
    ) {
            // if gie.ie_type == 0 {
            //     break; // End of array
            // }

            // let key = (gie.ie_type, gie.ie_instance);
            // let parse_info = Gtpv2IeParseInfo {
            //     ie_min_length: gie.ie_min_length,
            //     ie_max_length: gie.ie_max_length,
            //     ie_presence: gie.ie_presence,
            //     reserved: gie.reserved,
            //     group_ie_info: None,
            // };

            // self.ie_parse_info.insert(key, parse_info);

            // if gie.ie_presence == GTPV2C_IE_PRESENCE_MANDATORY {
            //     self.mie_count += 1;
            // }
        }
}


#[derive(Debug)]
struct IeParseDetails { 
    ie_min_length: u8,
    ie_max_length: u8,
    ie_presence: u8,
    reserved: u8,
    group_ie_info: Option<Box<Gtpv2GroupIeParseInfo>>,
}

#[derive(Debug)]
struct IeParseGroupDetails { //XXX
    ie_min_length: u8,
    ie_max_length: u8,
    ie_presence: u8,
    reserved: u8,
}


#[derive(Debug)]
struct Gtpv2GroupIeParseInfo {//gtpv2_group_ie_parse_info_t
    group_ie_type: u8,
    mie_count: usize,

    ie_parse_info: Vec<Vec<IeParseDetails>>,//[typemax][instancemax]
}


#[derive(Debug)]
struct Gtpv2IeParseInfo { //gtpv2_ie_parse_info_t
    group_ie_type: u8,
    mie_count: usize,

    ie_parse_info: Vec<Vec<IeParseDetails>>,//[typemax][instancemax]
}


const GTPV2_CONTROL_INTERFACE: u8 = 0;
const GTPV2_USER_INTERFACE: u8 = 1;

pub fn interface_type_map(interface_type: u8) -> u8 {
    match interface_type {
        0..=5 | 15 | 16 | 19..=23 | 29 | 31 | 33 | 34 | 37..=39 | 41 => { GTPV2_CONTROL_INTERFACE } //GTP-C
        6..=14 | 17 | 18 | 24..=28 | 30 | 32 | 35 | 36 | 40 => { GTPV2_USER_INTERFACE } //GTP-U
        _ => {
            println!("Unknown interface type");
            255 // Unknown type
        }
    }
}


fn encode_tbcd(input: &[u8]) -> Result<Vec<u8>, String> {
    let n = input.len();
    if n < 2 || n > 32 {
        return Err("Input length must be between 2 and 32".to_string());
    }

    let mut encoded = Vec::new();
    let mut i = 0;

    if n % 2 != 0 {
        // Odd length
        while i < n - 1 {
            let high = input[i + 1] & 0x0F;
            let low = input[i] & 0x0F;
            encoded.push((high << 4) | low);
            i += 2;
        }
        let last = input[n - 1] & 0x0F;
        encoded.push((0xF0) | last);
    } else {
        // Even length
        while i < n {
            let high = input[i + 1] & 0x0F;
            let low = input[i] & 0x0F;
            encoded.push((high << 4) | low);
            i += 2;
        }
    }

    Ok(encoded)
}



fn decode_tbcd(input: &[u8]) -> Result<Vec<u8>, String> {
    let n = input.len();
    if n < 6 || n > 9 {
        return Err("Input length must be between 6 and 9".to_string());
    }

    let mut decoded = Vec::new();

    for i in 0..n - 1 {
        let low = (input[i] & 0x0F) + b'0';
        let high = (input[i] >> 4) + b'0';
        decoded.push(low);
        decoded.push(high);
    }

    let last = input[n - 1];
    decoded.push((last & 0x0F) + b'0');
    if (last & 0xF0) != 0xF0 {
        decoded.push((last >> 4) + b'0');
    }

    Ok(decoded)
}

fn send_gtpv2_version_not_supported( _peer: Arc<Mutex<Gtpv2Peer>>,
    _peerip: u32, _peerport: u16, _seqnum: u32) {
    // Example function to send version not supported message
    info!("Sent version not supported message.");
}


fn is_group_ie(ie_type: u8, dictionary: &GtpMessage) -> bool {
    dictionary.ie_list
        .iter()
        .find(|info| info.ie_type == ie_type )
        .map_or(false, |info| info.group_ie_info.is_some())
}


pub async fn find_dictionary(msg_type: u8) -> Result<GtpMessage, String> {
    let dictionary = GTP_DICTIONARY.read().await;

    let result = dictionary 
            .iter()
            .find(|msg| msg.msg_type == msg_type )
            .ok_or(format!("Unknown message type: 0x{:x}", msg_type));
    
    match result {
        Ok(data) =>  Ok(data.clone()), 
        Err(err) =>  Err("Unknow Message type".to_string()),
    }
}

fn extract_nested_ies(raw_data: &[u8], dictionary: &GtpMessage) -> Vec<(u8, usize) > {
    let mut ies = Vec::new();
    let mut index = 0;

    while index + 4 <= raw_data.len() {
        let ie_type = raw_data[index];
        let ie_length = u16::from_be_bytes([raw_data[index + 1], raw_data[index + 2]]) as usize;

        if index + 4 + ie_length > raw_data.len() {
            break;
        }

        ies.push((ie_type, ie_length ));
        index += 4 + ie_length;
    }

    ies
}


fn check_ie_length(raw_data: &[u8], dictionary: &GtpMessage)
    -> Vec<(u8, usize, Vec<(u8, usize)>) > {
    let mut ies = Vec::new();
    let mut index = 0;

    while index + 4 <= raw_data.len() {
        let ie_type = raw_data[index];
        let ie_length = u16::from_be_bytes([raw_data[index + 1], raw_data[index + 2]]) as usize;

        if index + 4 + ie_length > raw_data.len() {
            break;
        }

        let ie_value = &raw_data[index+4..index+4+ie_length];
        let nested_ies = if is_group_ie(ie_type, &dictionary) {
            extract_nested_ies(ie_value, &dictionary)
        }
        else {
            Vec::new()
        };

        ies.push((ie_type, ie_length, nested_ies));
        index += 4 + ie_length;
    }

    ies
}

#[derive(Debug, Clone)]
enum LVValue {
    Single(Vec<u8>),
    Nested(Vec<TLV>),
    None,
}

#[derive(Debug, Clone)]
struct TLV {
    t:  u8,    //length
    l:  u16,    //length
    i:  u8,     //instance
    v:  Vec<u8>,//value
}
impl TLV {
    fn new() -> Self {
        TLV{
			t : 0u8, //type
            l : 0u16, //length
            i : 0u8,  //instance
            v : Vec::new(),
        }
    }
}

fn get_single_value(ies: IeMessage, id: u8)
-> Result<Vec<u8>, String> {
    if let Some(lv) = ies.get_ie(id) {
		if let LVValue::Single(ref v) = lv[0].v {
			Ok(v.clone())
		} else {
			Err(format!("Fail to get IE{id} value").to_string())
		}
	}
	else {
		Err(format!("IE{id} not found").to_string())
	}
}


#[derive(Debug, Clone)]
struct LV {
    l:  u16,    //length
    i:  u8,     //instance
    v:  LVValue,
}
impl LV {
    fn new_single() -> Self {
        LV{
            l : 0u16, //length
            i : 0u8,  //instance
            v : LVValue::Single(Vec::new()),
        }
    }

    fn new_nested() -> Self {
        LV{
            l : 0u16, //length
            i : 0u8,  //instance
            v : LVValue::Nested(Vec::new()),
        }
    }
}


#[derive(Debug, Clone)]
struct IeMessage {
    tlvs: HashMap<u8, Vec<LV>>,
}

impl IeMessage {
    fn new() -> Self {
        IeMessage {
            tlvs: HashMap::new()
        }
    }

    fn add_ie(&mut self, id: u8, lv: LV) {
        // self.tlvs.entry(id).or_insert_with(Vec::new).push(lv);
		if let Some(vec) = self.tlvs.get_mut(&id) {
            // If a entry is exiest already, push lv to Vec.
            vec.push(lv);
        } else {
            // If a entry is not exist, create a new Vec and insert lv.
            self.tlvs.insert(id, vec![lv]);
        }
    }

    fn get_ie(&self, id: u8) -> Option<&Vec<LV>> {
        self.tlvs.get(&id)
    }
}


pub fn get_ie_value(raw_data: &[u8])
    -> IeMessage
{
    let mut index = 0;
    let mut table = IeMessage::new();
    let mut len: usize;

    while index + 4 <= raw_data.len() {

        let t = raw_data[index];
        if t == GTPV2C_IE_BEARER_CONTEXT {// for Group IEs
            
            let mut glv = LV::new_nested();
            let mut gpos = 0;
            let glen = u16::from_be_bytes([raw_data[index + 1], raw_data[index + 2]]);

            glv.l = glen;
            glv.i = raw_data[index + 3] & 0x0f;

            index += 4;

            while gpos <= glen as usize {

                let mut lv = TLV::new();
                lv.t = raw_data[index + gpos + 0];
                lv.l = u16::from_be_bytes([raw_data[index + gpos + 1], raw_data[index + gpos + 2]]);
                lv.i = raw_data[index + gpos + 3] & 0x0f;
                len = lv.l as usize;
                lv.v.copy_from_slice(&raw_data[index + gpos + 4..index + gpos + 4 + len]);

                gpos += (4 + len);
                if let LVValue::Nested(ref mut nested_vec) = glv.v { nested_vec.push(lv);}
                else { panic!("Expected LVValue::Nested for glv.v"); }
            }

            table.add_ie( t, glv);
            continue;
        }

        let mut lv = LV::new_single();

        lv.l = u16::from_be_bytes([raw_data[index + 1], raw_data[index + 2]]);
        lv.i = raw_data[index + 3] & 0x0f;
        len = lv.l as usize;

        if let LVValue::Single(ref mut sigle_vec) = lv.v {
            sigle_vec.copy_from_slice(&raw_data[index + 4..index + 4 + len]);
        }

        table.add_ie( t, lv);
        index += 4 + len;
    }

    table
}


async fn recv_delete_session_req( peer: & mut Peer, ies: IeMessage, teid: u32,
    teid_list: &TeidList, sess_list: &SessionList)
-> Result < (), String>
{
    let mut imsi: String;
    let mut ebi = 0;
    let mut pdn_index:u32 = 0;
    let mut address = [Ipv4Addr::new(0,0,0,0); 48];

    trace!("GET EBI IE");
    // if let Some(lv ) = ies.get_ie(GTPV2C_IE_EBI) {

    //     if let LVValue::Single(ref v) = lv[0].v { ebi = v[0];}
    //     else {return Err("Single value is error".to_string());}
    // }
	if let Result::Ok(ret) = get_single_value(ies, GTPV2C_IE_EBI) {
		ebi = ret[0];
	}
	else {
		return Err("Fail to get IE EBI".to_string());
	}

	let mut session: Arc<Mutex<Session>> = if let Some(result) = teid_list.get(&teid) {
		result
	}
	else {
		return Err(format!("Fail to get Session by teid{}", teid).to_string());
	};
	let imsi = session.lock().unwrap().imsi.clone();

    delete_pdn_and_bearer(&mut session, ebi);

    gtp_send_delete_session_response(peer.clone(), &imsi.clone(), pdn_index as usize).await;

    Ok(())
}

async fn recv_modify_bearer_req( peer: & mut Peer, ies: IeMessage, teid: u32,
    teid_list: &TeidList, sess_list: &SessionList)
-> Result < (), String>
{
	let mut session: Arc<Mutex<Session>>;
    let mut ebi = 0;
    let mut pdn_index:u32 = 0;
    let mut address = [Ipv4Addr::new(0,0,0,0); 48];

    trace!("GET EBI IE");
    // if let Some(lv) = ies.get_ie(GTPV2C_IE_EBI) {
    //     if let LVValue::Single(ref v) = lv[0].v { ebi = v[0];}
    //     else {return Err("Single value is error".to_string());}
    // }
	if let Result::Ok(ret) = get_single_value(ies.clone(), GTPV2C_IE_EBI) {
		ebi = ret[0];
	}
	else {
		return Err("Fail to get IE EBI".to_string());
	}

    // trace!("GET F-TEID IE");
    // let mut gtpc_interfaces: Vec<control_info> = vec![control_info::new()];
    // let mut gtpu_interfaces: Vec<control_info> = vec![control_info::new()];

    // if let Some(lv) = ies.get_ie(GTPV2C_IE_FTEID) {
    //     for item in lv {
    //         let mut info  = control_info::new();

    //     	if let LVValue::Single(ref v) = item.v {
    //         	info.interface_type = v[0] & 0x3f;
    //         	info.teid = u32::from_be_bytes( v[1..5].try_into().expect("convert to teid problem") );
    //         	info.addr = Ipv4Addr::new( v[5], v[6], v[7], v[8]);
    //         	info!("Interface type: {}", info.interface_type);
    //         	info!("Interface teid: {}", info.teid);
    //         	info!("Interface addr: {}", info.addr);
    //     	}
    //     	else {return Err("Single value is error".to_string());}


    //         if interface_type_map(info.interface_type) == GTPV2_CONTROL_INTERFACE {
    //             gtpc_interfaces.push(info);
    //         }
    //         else if interface_type_map(info.interface_type) == GTPV2_USER_INTERFACE {
    //             gtpu_interfaces.push(info);
    //         }
    //     }

    // }
	trace!("Get Bearer Context IE"); //Group IE
	let mut bearers: Vec<BearerCnxt> = vec![BearerCnxt::new()];
	if let Some(lv) = ies.get_ie(GTPV2C_IE_BEARER_CONTEXT) {

		if let LVValue::Nested( ref group_ies ) = lv[0].v {
			let mut fteid_index = 0;
			for tlv in group_ies {
				let mut bearer_info: BearerCnxt = BearerCnxt::new();

				match tlv.t {
					GTPV2C_IE_EBI => {
    					trace!("\tGET EBI IE");
						bearer_info.ebi = tlv.v[0];
					}

					GTPV2C_IE_FTEID => {
    					trace!("\tGET F_TEID IE");
            			// let mut info  = control_info::new();
						bearer_info.iface_info[fteid_index].used = true;
						bearer_info.iface_info[fteid_index].iface_type = tlv.v[0] & 0x3f;
						bearer_info.iface_info[fteid_index].teid = u32::from_be_bytes( tlv.v[1..5].try_into().expect("convert to teid problem") );
						bearer_info.iface_info[fteid_index].addr = Ipv4Addr::new( tlv.v[5], tlv.v[6], tlv.v[7], tlv.v[8]);
						fteid_index += 1;
					}

					GTPV2C_IE_BEARER_QOS => {
    					trace!("\tGET Bearer QoS IE");
						bearer_info.bearer_qos.flag   = tlv.v[0];
						bearer_info.bearer_qos.qci    = tlv.v[1];
						bearer_info.bearer_qos.mbr_ul = u32::from_be_bytes( tlv.v[2..6].try_into().expect("convert to mbr_ul problem"));
						bearer_info.bearer_qos.mbr_dl = u32::from_be_bytes( tlv.v[6..10].try_into().expect("convert to mbr_dl problem"));
						bearer_info.bearer_qos.gbr_ul = u32::from_be_bytes( tlv.v[10..14].try_into().expect("convert to gbr_ul problem"));
						bearer_info.bearer_qos.gbr_dl = u32::from_be_bytes( tlv.v[14..18].try_into().expect("convert to gbr_dl problem"));
					}

					_ => {
						error!("Unknown IE type: 0x{:x}", tlv.t);
					}
				}

				bearers.push(bearer_info);
			}
		}
	}
	info!("Done to get Info from IEs");

	info!("Get Session by teid");
	session = if let Some(result) = teid_list.get(&teid) {
		result
	}
	else {
		return Err(format!("Fail to get Session by teid{}", teid).to_string());
	};

	trace!("Set Bearer");
    {
		for bearer_info in bearers {
        	if let Some(index) = find_bearer(session.clone(), bearer_info.ebi) {
        		let session = &mut session.lock().unwrap();
				let mut bearer = &mut session.bearer[index];
				modify_bearer(&mut bearer, bearer_info);
			}
		}
		// if 
    }
    // gtp_send_modify_bearer_response ();
    Ok(())
}


async fn recv_crte_sess_req( peer: & mut Peer, ies: IeMessage, teid: u32,
    teid_list: &TeidList, sess_list: &SessionList)
-> Result < (), String>
{
    let imsi;
    let msisdn;
    let mut ebi = 0;
    let mut pdn_index = 0;
    let mut address = [Ipv4Addr::new(0,0,0,0); 48];

    trace!("GET IE IMSI");
    if let Some(lv) = ies.get_ie(GTPV2C_IE_IMSI) {
        if let LVValue::Single(ref data) = lv[0].v {
			match decode_tbcd(&data) {
				Ok(decoded) => {
					info!("TBCD Convert for IMSI success");
					imsi = String::from_utf8_lossy(&decoded).into_owned();
				}
				Err(_) => {
					error!("Failed to decode IMSI");
					return Err("Failed to decode IMSI.".to_string());
				}
			}
		}
        else { return Err("Error to get IMSI from LVValue::Single".to_string()); }
    } else {
        error!("IMSI IE not found");
        return Err("IMSI IE not found.".to_string());
    }

    trace!("GET IE MSISDN");
    if let Some(lv) = ies.get_ie(GTPV2C_IE_MSISDN) {
    
        if let LVValue::Single(ref data) = lv[0].v {
			// match decode_tbcd(&data) {
			match decode_tbcd(data) {
				Ok(decoded) => {
					info!("TBCD Convert for MSISDN success");
					msisdn = String::from_utf8_lossy(&decoded).into_owned();
				}
				Err(_) => {
					error!("Failed to decode MSISDN");
					return Err("Failed to decode MSISDN.".to_string());
				}
			}
		}
        else { return Err("Error to get MSISDN from LVValue::Single".to_string()); }
    } else {
        error!("MSISDN IE not found");
        return Err("MSISDN IE not found.".to_string());
    }
    
    trace!("GET EBI IE");
    if let Some(lv) = ies.get_ie(GTPV2C_IE_EBI) {
        if let LVValue::Single(ref v) = lv[0].v { ebi = v[0];}
        else {return Err("Single value is error".to_string());}
    }

    trace!("GET FTEID IE");
    let mut gtpc_interfaces: Vec<control_info> = vec![control_info::new()];
    let mut gtpu_interfaces: Vec<control_info> = vec![control_info::new()];

    if let Some(lv) = ies.get_ie(GTPV2C_IE_FTEID) {
        for item in lv {
            let mut info  = control_info::new();

        	if let LVValue::Single(ref v) = item.v {
				info.interface_type = v[0] & 0x3f;
				info.teid = u32::from_be_bytes( v[1..5].try_into().expect("convert to teid problem") );
				info.addr = Ipv4Addr::new( v[5], v[6], v[7], v[8]);

				info!("Interface type: {}", info.interface_type);
				info!("Interface teid: {}", info.teid);
				info!("Interface addr: {}", info.addr);
			}

            if info.interface_type == 6 {
                peer.teid = info.teid;
            }

            if interface_type_map(info.interface_type) == GTPV2_CONTROL_INTERFACE {
                gtpc_interfaces.push(info);
            }
            else if interface_type_map(info.interface_type) == GTPV2_USER_INTERFACE {
                gtpu_interfaces.push(info);
            }
        }
    }

    trace!("GET APN IE");
    let mut apn = String::new();
    if let Some(lv) = ies.get_ie(GTPV2C_IE_APN) {
        if let LVValue::Single(ref v) = lv[0].v {
			if v.len() > 0{
				let ret =
					String::from_utf8(v.clone());
				match ret {
					Ok(val) => {
						apn = val;
						info!("Success to get APN: {}", apn);
					}
					Err(_) => {
						error!("Failed to get APN");
						return Err("Failed to get APN.".to_string());
					}
				}
			}
		}
		else {
			return Err("Single value is error".to_string());
		}
    }

    trace!("GET AMBR IE");
    let mut ambr_ul:u32 = 0;
    let mut ambr_dl:u32 = 0;
    if let Some(lv) = ies.get_ie(GTPV2C_IE_AMBR) {
		if let LVValue::Single(ref v) = lv[0].v {
			ambr_ul = u32::from_be_bytes(v[0..4].try_into().expect("convert to ambr_ul problem"));
			ambr_dl = u32::from_be_bytes(v[4..8].try_into().expect("convert to ambr_dl problem"));
		}
    }

	trace!("Get Bearer Context IE"); //Group IE
	let mut bearers: Vec<BearerCnxt> = vec![BearerCnxt::new()];
	if let Some(lv) = ies.get_ie(GTPV2C_IE_BEARER_CONTEXT) {

		if let LVValue::Nested( ref group_ies ) = lv[0].v {
			let mut fteid_index = 0;

			for tlv in group_ies {
				let mut bearer_info: BearerCnxt = BearerCnxt::new();

				match tlv.t {
					GTPV2C_IE_EBI => {
    					trace!("\tGET EBI IE");
						bearer_info.ebi = tlv.v[0];
					}

					GTPV2C_IE_FTEID => {
    					trace!("\tGET F_TEID IE");
            			// let mut info  = control_info::new();
						bearer_info.iface_info[fteid_index].used = true;
						bearer_info.iface_info[fteid_index].iface_type = tlv.v[0] & 0x3f;
						bearer_info.iface_info[fteid_index].teid = u32::from_be_bytes( tlv.v[1..5].try_into().expect("convert to teid problem") );
						bearer_info.iface_info[fteid_index].addr = Ipv4Addr::new( tlv.v[5], tlv.v[6], tlv.v[7], tlv.v[8]);
						fteid_index += 1;
					}

					GTPV2C_IE_BEARER_QOS => {
    					trace!("\tGET Bearer QoS IE");
						bearer_info.bearer_qos.flag   = tlv.v[0];
						bearer_info.bearer_qos.qci    = tlv.v[1];
						bearer_info.bearer_qos.mbr_ul = u32::from_be_bytes( tlv.v[2..6].try_into().expect("convert to mbr_ul problem"));
						bearer_info.bearer_qos.mbr_dl = u32::from_be_bytes( tlv.v[6..10].try_into().expect("convert to mbr_dl problem"));
						bearer_info.bearer_qos.gbr_ul = u32::from_be_bytes( tlv.v[10..14].try_into().expect("convert to gbr_ul problem"));
						bearer_info.bearer_qos.gbr_dl = u32::from_be_bytes( tlv.v[14..18].try_into().expect("convert to gbr_dl problem"));
					}

					_ => {
						error!("Unknown IE type: 0x{:x}", tlv.t);
					}
				}

				bearers.push(bearer_info);
			}
		}

	}

    if teid == 0 { //Initial Attach
        let mut session;

        trace!("Create New session");
        match find_session_or_create(imsi.clone(), sess_list) {
            Ok(value) => session = value,
            Err(error) => return Err(error),
        }

        let teid = if let Some(result) = generate_teid() {
			result
		}
        else {
			return Err("Failt to generate TEID".to_string());
		};

        trace!("Created TEID: 0x{:x?}", teid);
        teid_list.add(teid, Arc::clone(&session));

        trace!("Save MSISDN information");
        {
            session.lock().unwrap().msisdn = msisdn;
        }

        trace!("Allocation IP Address");
        let alloc_ip = allocate_ip();

        trace!("Setting for PDN");
        match alloc_pdn(&mut session, ebi, alloc_ip, ambr_dl, ambr_ul, apn) {
            Ok(value) => pdn_index = value,
            Err(error) => return Err(error),
        }

        trace!("Setting for Bearer");
		match alloc_bearer(&mut session, ebi, bearers) {
				Err(error) => return Err(error),
				Ok(ret ) => trace!("Success to allocate bearer. current bearere cnt: {ret}")}

        trace!("Control interface settings");
        session.lock().unwrap().control.extend(gtpc_interfaces);

        gtp_send_create_session_response(peer.clone(), session, pdn_index as usize).await;
	}
    else { // Multiple PDN Attach 
        trace!("Already Exist Session");

        let mut session: Arc<Mutex<Session>> = if let Some(result) = teid_list.get(&teid) {
            result
        }
        else {
            return Err(format!("Fail to get Session by teid{}", teid).to_string());
        };

        let imsi = session.lock().unwrap().imsi.clone();

        trace!("Allocation IP Address");
        let alloc_ip = allocate_ip();

        trace!("Setting for PDN");
        match alloc_pdn(&mut session, ebi, alloc_ip, ambr_dl, ambr_ul, apn) {
            Ok(value) => pdn_index = value,
            Err(error) => return Err(error),
        }

        trace!("Setting for Bearer");
		match alloc_bearer(&mut session, ebi, bearers) {
			Err(error) => return Err(error),
			Ok(ret ) => trace!("Success to allocate bearer. current bearere cnt: {ret}")}

        trace!("Control interface settings");
        session.lock().unwrap().control.extend(gtpc_interfaces);

        gtp_send_create_session_response(peer.clone(), session, pdn_index as usize).await;
    }

    Ok(())
}

async fn recv_echo_req( peer: & mut Peer, ies: IeMessage)
-> Result < (), String>
{
    let mut restart_counter = 0;

    trace!("GET IE Recovery");
    // if let Some(lv) = ies.get_ie(GTPV2C_IE_RECOVERY) {
    //     if let LVValue::Single(ref v) = lv[0].v {
    //     	restart_counter = v[0];
	// 	}
    // }
	if let Result::Ok(ret) = get_single_value(ies, GTPV2C_IE_RECOVERY) {
		restart_counter = ret[0];
	}
	else {
		return Err("Fail to get IE Recovery".to_string());
	}

    if peer.get_peer_status() == false {
        peer.activate_peer_status();
    }
    peer.update_last_active();

    gtp_send_echo_response(peer.clone() , restart_counter).await;   

    Ok(())
}

async fn recv_echo_rsp( peer: & mut Peer, ies: IeMessage)
-> Result < (), String>
{
    let mut restart_counter = 0;

    trace!("GET IE Recovery");
    // if let Some(lv) = ies.get_ie(GTPV2C_IE_RECOVERY) {
    //     if let LVValue::Single(ref v) = lv[0].v {
    //     	restart_counter = v[0];
	// 	}
    // }
	if let Result::Ok(ret) = get_single_value(ies, GTPV2C_IE_RECOVERY) {
		restart_counter = ret[0];
	}

    Ok(())
}


async fn pgw_recv( peer: &mut Peer, ies: IeMessage,
    msg_type: u8, teid: u32, rcv_seq: u32,
    teid_list: &TeidList, sess_list: &SessionList)
{
    match msg_type {
        GTPV2C_ECHO_REQ => {
            trace!("This message is Echo Request");
            recv_echo_req( peer, ies).await;
        }
        GTPV2C_ECHO_RSP => {
            trace!("This message is Echo Request");
            find_pkt_in_queue(peer.ip, rcv_seq);
            recv_echo_rsp( peer, ies).await;
        }

        // Create Session Reqeust
        GTPV2C_CREATE_SESSION_REQ => {
            trace!("This message is Create Session Request");
            recv_crte_sess_req( peer, ies, teid, teid_list, sess_list).await;
        }

        //Modify Bearere Request
        GTPV2C_MODIFY_BEARER_REQ => {
            trace!("This message is Modify Bearer Request");
            recv_modify_bearer_req( peer, ies, teid, teid_list, sess_list).await;
        }

        //Delete Session Request
        GTPV2C_DELETE_SESSION_REQ => {
            trace!("This message is Delete Session Request");
            recv_delete_session_req( peer, ies, teid, teid_list, sess_list).await;
        }
        _ => {
            error!("Unknown Message type: {}", msg_type);
        }
    }
}


pub async fn gtpv2_recv_task(socket: UdpSocket) {
    let mut buf = [0u8; BUFSIZ];
    let timeout = Duration::from_millis(1000);
    println!("Start Recv Task");

    let teid_list = TeidList::new();
    let sess_list = SessionList::new();

    loop {
        socket.set_read_timeout(Some(timeout)).unwrap();

        match socket.recv_from(&mut buf) {
            Ok((nbyte, addr)) => {
                info!("Receive GTP MESSAGE");
                if let SocketAddr::V4(addr) = addr {
                    // let sin_addr = addr.ip();
                    let sin_port = addr.port();

                    /* Only for TEST */
                    let sin_addr = &Ipv4Addr::new(10,10,2,72);
                    trace!("sin_addr info {:?}", sin_addr);

                    match get_peer(sin_addr) {
                        Err(_) => {
                            warn!("This is not registered PEER!");
                        } 

                        Ok(mut peer) => {
                            info!("Success get peer {}", peer.ip);
                            _ = recv_gtpv2(&buf[..nbyte], &mut peer, &teid_list, &sess_list ).await;
                        }
                    }
                }
            }

            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }

            Err(e) => {
                error!("Error receiving data: {:?}", e);
                break;
            }
        }
    }
}

pub async fn recv_gtpv2(_data: &[u8], peer: &mut Peer,
    teid_list: &TeidList, sess_list: &SessionList
)  -> Result<(), String> {
    
    let hdr_len;
    let rcv_msg_type;
    let msg_define;
    let mut teid = 0;
    let mut rcv_seq = 0;

    trace!("Processed GTPv2 data.");
    let ret = parse_header(_data.clone());
    match ret {
        Ok( (v, n)) => {
            rcv_msg_type = v.t;
            teid         = v.teid;
            rcv_seq      = v.s;
            hdr_len      = n;
        }
        _ => {
            error!("Error in parsing header");
            return Err(format!("error").to_string());
        },
    }

    trace!("Get dictionary based on receive message type");
    let dictionary = GTP_DICTIONARY.read().await;

    let result = dictionary 
            .iter().find(|msg | msg.msg_type == rcv_msg_type )
            .ok_or(format!("Unknown message type: 0x{:x}", rcv_msg_type));

    match result {
        Ok(data) =>{
            info!("Success! to get gtp dictionary");
            msg_define = data;
        }
        Err(err) => { println!("{}", err);
            error!("Fail to get dictionary");
            return Err(format!("error").to_string());
        },
    }


    let ies = check_ie_length(&_data[hdr_len..], &msg_define);
    if let Err(ret) = validate_length(&ies, &msg_define) {
        error!("Validation Check false: [{}]", ret);
    }

    trace!("get IE value");
    let ies = get_ie_value(&_data[hdr_len..]);

    trace!("PEER");
    peer.update_rseq(rcv_seq);

    trace!("Send to Next peer");
    pgw_recv( peer, ies, rcv_msg_type, teid, rcv_seq, teid_list, sess_list).await;

    Ok(())
}