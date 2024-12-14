extern crate lazy_static;
// use tokio::time::{self, Duration};
use std::sync::{Arc, Mutex};
use crate::gtpv2_type::*;
// use crate::recv_gtpv2::*;
// use tokio::sync::mpsc::{Receiver, Sender};
use crate::peers::*;
use crate::session::*;
use crate::gtpv2_send::*;
use crate::timer::*;


// #[derive(Debug, Clone, Copy)]
#[repr(C)]
// #[derive(Debug, Clone, Copy, Pod, Zeroable)] // bytemuck의 Pod와 Zeroable 트레잇 구현
pub struct Gtpv2CHeader {
    pub v:      u8,     // Version (3 bits) + P flag (1 bit) + T flag (1 bit) + MP flag (1bit + 2 spare bits 
    pub t:      u8,     // Message type (8 bits)
    pub l:      u16,    // Message length (16 bits)
    pub teid:   u32,    // TEID (32 bits), optional based on T flag
    pub s:      u32,    // Sequence number (24 bits) + 4  Message Priority + 4 spare bits
    // pub ies: Vec<u8>,
}


// GTPv2-C 헤더를 초기화하는 함수
impl Gtpv2CHeader {
    pub fn new() -> Self {
        Self {
            v : 0,
            t : 0,
            l : 0,
            teid : 0,
            s : 0,
        }
    }

    pub fn encode<'a> (p:&'a mut [u8], p_flag: bool,
            t_flag: bool, mp_flag: bool, t: u8,
            l: u16, teid: u32, s: u32, mp: u8)
    -> (&[u8], usize)
    {

        let mut v = (GTP_VERSION & 0b111) << 5; // Version은 상위 3비트
        let mut length = 8; //without TEID Field.
        let mut l = l;

        if p_flag {
            v |= 0b0001_0000; // P-flag 설정
        }

        if t_flag {
            v |= 0b0000_1000; // T-flag 설정
            length += 4; // Add 4bytes for TEID Field
            l += 4;
        }

        let mut s = s << 8;
        if mp_flag {
            v |= 0b0000_0100; // T-flag 설정
            // s |= mp as u32;
        }

        let len     = l.to_be_bytes();
        let teid  = teid.to_be_bytes();
        let seq     = s.to_be_bytes();

        if t_flag {
            p[..length].copy_from_slice(
                &[ v, t, len[0], len[1],
                teid[0], teid[1], teid[2], teid[3],
                seq[0], seq[1], seq[2], seq[3] ]);
        }
        else {
            p[..length].copy_from_slice(
                &[ v, t, len[0], len[1],
                seq[0], seq[1], seq[2], seq[3] ]);
        }

        (&p[..length as usize], length as usize)
    }

}


/* GTPv2 IE */
pub struct gtpv2c_ie1 {
    t:			u8,     /* type (1octet) */
    l:			u16,    /* length (2octet) */
    i:			u8,     /* spare (4bit) + instance  (4bit)*/
    v:			u8,     /* value (1octet) */
}

impl gtpv2c_ie1 {
    pub fn new(p: &mut [u8], t: u8, i: u8, v: u8){
        let l = 1u16.to_be_bytes();
        let mut i =i& 0x0f;

        &p[..5].copy_from_slice( &[ t, l[0], l[1], i, v, ] );
    }
}


struct gtpv2c_ie2 {
    t:			u8, /* type (1octet) */
    l:			u16, /* length (2octet) */
    i:			u8, /* spare (4bit) + instance (4bit) */
    v:			u16, /* value (2octet) */
}

struct gtpv2c_ie4 {
    t:			u8	, /* type (1octet) */
    l:			u16, /* length (2octet) */
    i:			u8, /* spare (4bit) + instance (4bit) */
    v:			u32, /* value (4octet) */
}

struct gtpv2c_ie8 {
    t:			u8, /* type (1octet) */
    l:			u16, /* length (2octet) */
    i:			u8, /* spare (4bit) + instance (4bit) */
    v:			u64, /* value (8octets) */
}

struct gtpv2c_ie_tlv {
    t:			u8, /* type (1octet)*/
    l:			u16, /* length (2octet) */
    i:			u8, /* spare (4bit) + instance (4bit) */
}


pub fn create_ie <'a> (p:&'a mut [u8], t:u8, val:u8) -> usize
{
    let mut len: usize = 0;

    match t {
        GTPV2C_IE_RECOVERY => {
            gtpv2c_ie1::new(p, t, 0, val);
            len += std::mem::size_of::<gtpv2c_ie1>();
        }
        _ => println!("Not yet Developed type: {}", t),
    }

    len
}



fn gtpv2_get_ie_tv1(data: &[u8], val: &mut u8) -> Result<usize, &'static str> {
    if data.is_empty() {
        return Err("Data length is insufficient");
    }
    *val = data[0];
    Ok(1)
}


fn gtpv2_get_ie_tv2(data: &[u8], val: &mut u16) -> Result<usize, &'static str> {
    if data.len() < 2 {
        return Err("Data length is insufficient");
    }
    *val = u16::from_be_bytes(data[0..2].try_into().unwrap());
    Ok(2)
}


fn gtpv2_get_ie_tv4(data: &[u8], val: &mut u32) -> Result<usize, &'static str> {
    if data.len() < 4 {
        return Err("Data length is insufficient");
    }
    *val = u32::from_be_bytes(data[0..4].try_into().unwrap());
    Ok(4)
}

fn gtpv2_get_ie_tlv(data: &[u8], val: &mut [u8]) -> Result<usize, &'static str> {
    if data.len() > val.len() {
        return Err("Destination buffer is too small");
    }
    val[..data.len()].copy_from_slice(data);
    Ok(data.len())
}


pub fn gtpv2_get_ie_under_tv4( data: &[u8],
    val_1: &mut Option<u8>, val_2: &mut Option<u16>,
    val_3: &mut Option<u32>, val_4: &mut Option<u32>,
) -> Result<(), &'static str> {
    match data.len() {
        1 => {
            *val_1 = Some(data[0]);
        }
        2 => {
            *val_2 = Some(u16::from_be_bytes(data[0..2].try_into().unwrap()));
        }
        3 => {
            let mut temp = [0u8; 4];
            temp[1..4].copy_from_slice(&data[0..3]);
            *val_3 = Some(u32::from_be_bytes(temp) >> 8);
        }
        4 => {
            *val_4 = Some(u32::from_be_bytes(data[0..4].try_into().unwrap()));
        }
        _ => return Err("Invalid data length"),
    }
    Ok(())
}


pub fn gtpv2_get_ie_tbcd(data: &[u8], val: &mut [u8])
-> Result<(), &'static str> {
    if data.len() > val.len() {
        return Err("Destination buffer is too small");
    }

    _dec_tbcd(data, val.len(), val);
    Ok(())
}


// Simulate _dec_tbcd function (you need to replace it with your actual implementation)
fn _dec_tbcd(data: &[u8], len: usize, val: &mut [u8]) {
    val[..len].copy_from_slice(&data[..len]);
}

// pub fn gtpv2_get_ie_cause(data: &[u8], causeie: &mut Gtpv2Error) -> Result<(), &'static str> {
//     if data.len() < 2 {
//         return Err("Insufficient data length for cause IE");
//     }
//     let mut p = &data[..];
//     causeie.cause = p[0];
//     causeie.flags = p[1];
//     p = &p[2..];

//     if p.len() == 4 {
//         causeie.offending_ie = Some(OffendingIe {
//             type_: p[0],
//             instance: p[3],
//         });
//     } else {
//         causeie.offending_ie = None;
//     }
//     Ok(())
// }

pub fn gtpv2_get_ie_mbr(data: &[u8], up: &mut u32, down: &mut u32) -> Result<(), &'static str> {
    if data.len() < 8 {
        return Err("Insufficient data length for MBR");
    }
    *up = u32::from_be_bytes(data[0..4].try_into().unwrap());
    *down = u32::from_be_bytes(data[4..8].try_into().unwrap());

    Ok(())
}


pub fn make_gtpv2(msg_type: u8, body: &[u8;1024],
                oldpeer: Peer, t_flag: bool, len: u8)
-> Result<(), String> {
    let mut peer;
    let mut length :usize = 0;
    let mut buffer: [u8; 1024] = [0; 1024]; 
    let bodylen: u16 = len as u16;

    let result = get_peer(&oldpeer.ip);
    match result {
        Ok(value) => peer = value,
        _ => return  Err ("test".to_string()),
    }

    (_, length) = Gtpv2CHeader::encode (&mut buffer, false,
        t_flag, false,
        msg_type, bodylen, oldpeer.teid, peer.tseq, 0) ;

    buffer[length..length + len as usize].copy_from_slice(&body[..len as usize]);

    let pkt = EncapPkt::new(peer.ip, msg_type, buffer[..length + len as usize].to_vec());

    send_udp_data(&buffer[..length+len as usize], &peer.ip.to_string(), peer.port);

    SHARED_QUEUE.lock().unwrap().push(pkt);


    peer.update_last_echo_snd_time();
    peer.increase_count();

    Ok(())
}


pub fn gtp_send_create_session_response
    (session_list: Arc<Mutex<SessionList>>, peer:Peer, imsi:&String, pdn_index: usize)
-> Result<(), String>
{
    let locked_sessionlist = session_list.lock().unwrap();
    let locked_session = locked_sessionlist.find_session_by_imsi(&imsi);
    let session;

    // Get Session.
    if let Some(session_arc) = locked_session {
        session = session_arc.lock().unwrap().clone();
    }
    else {
        return Err("Error No session".to_string());
    }

    let mut buffer:[u8;1024] = [0u8;1024];

    //IE CAUSE
    let mut total_len = gtpv2_add_ie_cause( &mut buffer, 0, 16, 0, None, 0);

    //IE F-TEID

    //IE PAA
    let ip = session.pdn[pdn_index-1].ip;
    total_len = gtpv2_add_ie_paa(&mut buffer, 0, 1, ip, total_len as u8);
    //IE PAN Restriction
    //IE AMBR

    //IE EBI
    let ebi = session.pdn[pdn_index-1].lbi;
    total_len = gtpv2_add_ie_tv1( &mut buffer, GTPV2C_IE_EBI, 0, ebi, total_len);

    //IE BEARER Context
    //IE Recovery

    //Make GTPv2 Header
    make_gtpv2(GTPV2C_CREATE_SESSION_RSP, &buffer, peer, true, total_len as u8);

    Ok(())
}


pub fn gtp_send_modify_bearer_response
    (session_list: Arc<Mutex<SessionList>>, peer:Peer, imsi:&String, pdn_index: usize)
-> Result<(), String>
{
    let locked_sessionlist = session_list.lock().unwrap();
    let locked_session = locked_sessionlist.find_session_by_imsi(&imsi);
    let session;

    // Get Session.
    if let Some(session_arc) = locked_session {
        session = session_arc.lock().unwrap().clone();
    }
    else {
        return Err("Error No session".to_string());
    }

    let mut buffer:[u8;1024] = [0u8;1024];

    //IE Cause
    //IE MSISDN
    //IE LBI
    //IE APN Restriction
    //IE PCO
    //IE Bearer Contexts modified
    //IE Change Report Action
    //IE Recovery

    //Make GTPv2 Header
    // make_gtpv2(GTPV2C_MODIFY_BEARER_RSP, &buffer, peer, true, total_len as u8);


    Ok(())
}


pub fn gtp_send_delete_session_request
    (session_list: Arc<Mutex<SessionList>>, peer:Peer, imsi:&String, pdn_index: usize)
-> Result<(), String>
{
    let locked_sessionlist = session_list.lock().unwrap();
    let locked_session = locked_sessionlist.find_session_by_imsi(&imsi);
    let session;

    // Get Session.
    if let Some(session_arc) = locked_session {
        session = session_arc.lock().unwrap().clone();
    }
    else {
        return Err("Error No session".to_string());
    }

    let mut buffer:[u8;1024] = [0u8;1024];

    //IE Cause
    //IE Recovery
    //IE PCO

    //Make GTPv2 Header
    // make_gtpv2(GTPV2C_DELETE_SESSION_RSP, &buffer, peer, true, total_len as u8);

    Ok(())
}