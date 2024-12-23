use std::hash::Hash;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket, SocketAddr};
use std::ptr::slice_from_raw_parts;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use core::result::Result;
use std::thread;
use std::collections::HashMap;
use log::{debug, error, info, trace, warn};

use crate::gtp_dictionary::{self, *};
use crate::{session, validate_gtpv2::*};
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


struct BearerQos {
    flag:   u8,
    qci:    u8,
    mbr_ul: u32,
    mbr_dl: u32,
    gbr_ul: u32,
    gbr_dl: u32,
}
impl BearerQos {
    fn new() -> Self {
        BearerQos {
            flag: 0,
            qci: 0,
            mbr_ul: 0,
            mbr_dl: 0,
            gbr_ul: 0,
            gbr_dl: 0,
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


#[derive(Debug)]
struct Gtpv2Msg {
    msg_type: u8,
    msg_buf: Vec<u8>,
    teid_present: bool,
    valid_ie: Vec<Vec<bool>>,
    pie: Vec<Vec<Option<Vec<u8>>>>,
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

fn send_gtpv2_version_not_supported(
    _peer: Arc<Mutex<Gtpv2Peer>>, _peerip: u32, _peerport: u16, _seqnum: u32
) {
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
struct LV {
    // t: u8,
    l: u16,
    i: u8,
    v: Vec<u8>,
}
impl LV {
    fn new() -> Self {
        LV{
            l : 0u16, //length
            i : 0u8,  //instance
            v : vec![], //value
        }
    }
}


#[derive(Debug, Clone)]
struct IeMessage {
    tlvs: HashMap<u8, Vec<LV>>,
    // tlvs: Vec<TLV>, // A list of TLVs
}

impl IeMessage {
    fn new() -> Self {
        IeMessage {
            tlvs: HashMap::new()
        }
    }

    fn add_ie(&mut self, id: u8, lv: LV) {
        // IE ID에 대해 기존 벡터가 없으면 새로 생성
        self.tlvs.entry(id).or_insert_with(Vec::new).push(lv);
    }

    fn get_ie(&self, id: u8) -> Option<&Vec<LV>> {
        self.tlvs.get(&id)
    }
}

// session: Arc<Session>,
pub fn get_ie_value(raw_data: &[u8])
    -> IeMessage
{
    // let mut ies = Vec::new();
    let mut index = 0;
    let mut table = IeMessage::new();
    let mut len: usize;


    while index + 4 <= raw_data.len() {

        let mut lv = LV::new();
        let t = raw_data[index];
        if t == 93 {
            index += 4;
            continue;
        }

        lv.l = u16::from_be_bytes([raw_data[index + 1], raw_data[index + 2]]);
        lv.i = raw_data[index + 3] & 0x0f;
        len = lv.l as usize;

        lv.v.resize(len, 0);
        lv.v.copy_from_slice( &raw_data[index+4..index+4+len]);
        table.add_ie( t, lv);
        index += 4 + len;
    }

    table
}


pub fn interface_type_map(interface_type: u8) -> u8 {
    match interface_type {
        0..=5 | 15 | 16 | 19..=23 | 29 | 31 | 33 | 34 | 37..=39 | 41 => { 0 } //GTP-C
        6..=14 | 17 | 18 | 24..=28 | 30 | 32 | 35 | 36 | 40 => { 1 } //GTP-U
        _ => {
            println!("Unknown interface type");
            255 // 반환값으로 알 수 없는 타입을 명시
        }
    }
}


async fn recv_delete_session_req( peer: & mut Peer, ies: IeMessage, teid: u32)
-> Result < (), String>
{
    let mut imsi: String;
    let mut ebi = 0;
    let mut pdn_index:u32 = 0;
    let mut address = [Ipv4Addr::new(0,0,0,0); 48];

    trace!("GET EBI IE");
    if let Some(lv) = ies.get_ie(GTPV2C_IE_EBI) {
        ebi = lv[0].v[0];
    }

    match get_imsi_by_teid(teid) {
        Ok(ret) => imsi = ret,
        Err(error) => return Err(error),
    }

    let mut session;

    match find_session_by_imsi(imsi.clone()) {
        Ok(value) => session = value,
        Err(error) => return Err(error),
    }

    delete_pdn_and_bearer(&mut session, ebi);

    gtp_send_delete_session_response(peer.clone(), &imsi.clone(), pdn_index as usize).await;

    Ok(())
}

async fn recv_modify_bearer_req( peer: & mut Peer, ies: IeMessage, teid: u32)
-> Result < (), String>
{
    let mut session;
    let mut imsi: String;
    let mut ebi = 0;
    let mut pdn_index:u32 = 0;
    let mut address = [Ipv4Addr::new(0,0,0,0); 48];

    // let mut gtpc_interfaces = vec![control_info::new()];
    let mut gtpu_interfaces = vec![control_info::new()];
    if let Some(lv) = ies.get_ie(GTPV2C_IE_FTEID) {
        for item in lv {
            let mut info  = control_info::new();
            info.interface_type = item.v[0] & 0x3f;
            info.teid = u32::from_be_bytes( item.v[1..5].try_into().expect("convert to teid problem") );
            info.addr = Ipv4Addr::new( item.v[5], item.v[6], item.v[7], item.v[8]);

            info!("Interface type: {}", info.interface_type);
            info!("Interface teid: {}", info.teid);
            info!("Interface addr: {}", info.addr);

            if interface_type_map(info.interface_type) == 1 {
                gtpu_interfaces.push(info);
            }
        }

    }

    match get_imsi_by_teid(teid) {
        Ok(value) => imsi = value,
        Err(error) => return Err(error),
    }

    match find_session_by_imsi(imsi.clone()) {
        Ok(value) => session = value,
        Err(error) => return Err(error),
    }

    trace!("Setting for Bearer");
    { //Bearer
            // let session = &mut arc_session.lock().unwrap();
            let vecbearer = &mut session.bearer;
    }

    Ok(())
}

async fn recv_crte_sess_req( peer: & mut Peer, ies: IeMessage, teid: u32)
-> Result < (), String>
{
    let imsi;
    let msisdn;
    let mut ebi = 0;
    let mut pdn_index = 0;
    let mut address = [Ipv4Addr::new(0,0,0,0); 48];

    trace!("GET IE IMSI");
    if let Some(lv) = ies.get_ie(GTPV2C_IE_IMSI) {
        // IMSI 데이터 복사 없이 참조로 처리
    
        match decode_tbcd(&lv[0].v) {
            Ok(decoded) => {
                info!("TBCD Convert for IMSI success");
                imsi = String::from_utf8_lossy(&decoded).into_owned();
            }
            Err(_) => {
                error!("Failed to decode IMSI");
                return Err("Failed to decode IMSI.".to_string());
            }
        }
    } else {
        error!("IMSI IE not found");
        return Err("IMSI IE not found.".to_string());
    }

    trace!("GET IE MSISDN");
    if let Some(lv) = ies.get_ie(GTPV2C_IE_MSISDN) {
    
        match decode_tbcd(&lv[0].v) {
            Ok(decoded) => {
                info!("TBCD Convert for MSISDN success");
                msisdn = String::from_utf8_lossy(&decoded).into_owned();
            }
            Err(_) => {
                error!("Failed to decode MSISDN");
                return Err("Failed to decode MSISDN.".to_string());
            }
        }
    } else {
        error!("MSISDN IE not found");
        return Err("MSISDN IE not found.".to_string());
    }
    
    trace!("GET EBI IE");
    if let Some(lv) = ies.get_ie(GTPV2C_IE_EBI) {
        ebi = lv[0].v[0];
    }

    trace!("GET FTEID IE");
    let mut gtpc_interfaces = vec![control_info::new()];
    let mut gtpu_interfaces = vec![control_info::new()];

    if let Some(lv) = ies.get_ie(GTPV2C_IE_FTEID) {
        for item in lv {
            let mut info  = control_info::new();

            info.interface_type = item.v[0] & 0x3f;
            info.teid = u32::from_be_bytes( item.v[1..5].try_into().expect("convert to teid problem") );
            info.addr = Ipv4Addr::new( item.v[5], item.v[6], item.v[7], item.v[8]);

            // Only Test
            // if peer.ip != info.addr {
            //     return Err("Unknown interface type.".to_string());
            // }

            if info.interface_type == 6 {
                peer.teid = info.teid;
            }

            if interface_type_map(info.interface_type) == 0 {
                gtpc_interfaces.push(info);
            }
            else if interface_type_map(info.interface_type) == 1 {
                gtpu_interfaces.push(info);
            }
        }
    }

    trace!("GET APN IE");
    let mut apn = String::new();
    if let Some(lv) = ies.get_ie(GTPV2C_IE_APN) {
        if lv[0].v.len() > 0{
            let ret = String::from_utf8(lv[0].v.clone());
            match ret {
                Ok(val) => {
                    info!("Success to get APN: {}", apn);
                    apn = val;
                }
                Err(_) => {
                    error!("Failed to get APN");
                    return Err("Failed to get APN.".to_string());
                }
            }
        }
    }

    trace!("GET AMBR IE");
    let mut ambr_ul:u32 = 0;
    let mut ambr_dl:u32 = 0;
    if let Some(lv) = ies.get_ie(GTPV2C_IE_AMBR) {
        ambr_ul = u32::from_be_bytes( lv[0].v[0..4].try_into().expect("convert to ambr_ul problem"));
        ambr_dl = u32::from_be_bytes( lv[0].v[4..8].try_into().expect("convert to ambr_ul problem"));
    }

    trace!("GET Bearer QoS IE");
    let mut bearer_qos = BearerQos::new();
    if let Some(lv) = ies.get_ie(GTPV2C_IE_BEARER_QOS) {
        for item in lv {
            bearer_qos.flag   = item.v[0];
            bearer_qos.qci    = item.v[1];
            bearer_qos.mbr_ul = u32::from_be_bytes( item.v[2..6].try_into().expect("convert to mbr_ul problem"));
            bearer_qos.mbr_dl = u32::from_be_bytes( item.v[6..10].try_into().expect("convert to mbr_dl problem"));
            bearer_qos.gbr_ul = u32::from_be_bytes( item.v[10..14].try_into().expect("convert to gbr_ul problem"));
            bearer_qos.gbr_dl = u32::from_be_bytes( item.v[14..18].try_into().expect("convert to gbr_dl problem"));
        }
    }


    if teid == 0 { //Initial Attach
        trace!("Create New session");

        let teid = generate_teid();
        trace!("Created TEID: 0x{:x?}", teid);
        // TEID_LIST.lock().unwrap().add_teid(teid.unwrap(), &imsi);

        let mut session;
        match find_session_or_create(imsi.clone()) {
            Ok(value) => session = value,
            Err(error) => return Err(error),
        }

        trace!("Session Setting");
        {
            session.msisdn = msisdn;
        }

        trace!("Allocation IP Address");
        let alloc_ip = allocate_ip();

        trace!("Setting for PDN");
        match alloc_pdn( &mut session, ebi, alloc_ip, ambr_dl, ambr_ul, apn) {
            Ok(value) => pdn_index = value,
            Err(error) => return Err(error),
        }

        trace!("Setting for Bearer");
        alloc_bearer(&mut session,
            ebi, ebi, gtpu_interfaces[0].teid,
            bearer_qos.flag, bearer_qos.qci,
            bearer_qos.mbr_ul, bearer_qos.mbr_dl,
            bearer_qos.gbr_ul, bearer_qos.gbr_dl);

        trace!("Control interface settings");
        session.control.extend(gtpc_interfaces);


        gtp_send_create_session_response(peer.clone(), imsi, pdn_index as usize).await;
    }
    else { // Multiple PDN Attach 
        trace!("Already Exist Session");

        let imsi;
        match get_imsi_by_teid(teid) {
            Ok(value) => imsi = value,
            Err(error) => return Err(error),
        }

        let mut session;
        match find_session_by_imsi(imsi.clone()) {
            Ok(value) => session = value,
            Err(error) => return Err(error),
        }

        trace!("Allocation IP Address");
        let alloc_ip = allocate_ip();

        trace!("Setting for PDN");
        match alloc_pdn( &mut session, ebi, alloc_ip, ambr_dl, ambr_ul, apn) {
            Ok(value) => pdn_index = value,
            Err(error) => return Err(error),
        }

        trace!("Setting for Bearer");
        alloc_bearer(&mut session,
            ebi, ebi, gtpu_interfaces[0].teid,
            bearer_qos.flag, bearer_qos.qci,
            bearer_qos.mbr_ul, bearer_qos.mbr_dl,
            bearer_qos.gbr_ul, bearer_qos.gbr_dl);

        trace!("Control interface settings");
        session.control.extend(gtpc_interfaces);

        gtp_send_create_session_response(peer.clone(), imsi, pdn_index as usize).await;
    }

    Ok(())
}

async fn recv_echo_req( peer: & mut Peer, ies: IeMessage)
-> Result < (), String>
{
    let mut restart_counter = 0;

    trace!("GET IE Recovery");
    if let Some(lv) = ies.get_ie(GTPV2C_IE_RECOVERY) {
        restart_counter = lv[0].v[0];
    }
    if peer.get_peer_status() == false {
        peer.activate_peer_status();
    }
    peer.update_last_active();

    gtp_send_echo_response(peer.clone() , restart_counter).await;   

    Ok(())
}


async fn pgw_recv( peer: &mut Peer, ies: IeMessage, msg_type: u8, teid: u32)
{
    match msg_type {
        GTPV2C_CREATE_SESSION_REQ => {
            trace!("This message is Echo Request");
            recv_echo_req( peer, ies).await;
        }
        GTPV2C_CREATE_SESSION_REQ => {
            trace!("This message is Create Session Request");
            recv_crte_sess_req( peer, ies, teid).await;
        }
        GTPV2C_MODIFY_BEARER_REQ => {
            trace!("This message is Modify Bearer Request");
            recv_modify_bearer_req( peer, ies, teid).await;
        }
        GTPV2C_DELETE_SESSION_REQ => {
            trace!("This message is Delete Session Request");
            recv_delete_session_req( peer, ies, teid).await;
        }
        _ => {
            error!("Unknown Message type: {}", msg_type);
        }
    }
}


pub async fn recv_gtpv2(_data: &[u8], peer: &mut Peer,
    // session_list: Arc<Mutex<SessionList>>, teid_list: Arc<Mutex<TeidList>>,
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
    info!("Get dictionary based on receive message type");
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
    pgw_recv( peer, ies, rcv_msg_type, teid).await;

    Ok(())
}

// pub async fn gtpv2_recv_task() {
// }
// pub async fn gtpv2_recv_task(socket: &UdpSocket, session_list: Arc<Mutex<SessionList>>, teid_list: Arc<Mutex<TeidList>>) {

pub async fn gtpv2_recv_task(socket: UdpSocket) {
    let mut buf = [0u8; BUFSIZ];
    let timeout = Duration::from_millis(1000);
    println!("Start Recv Task");

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
                            _ = recv_gtpv2(&buf[..nbyte], &mut peer ).await;
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