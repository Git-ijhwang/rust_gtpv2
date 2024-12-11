extern crate lazy_static;
use tokio::time::{self, Duration};
use crate::{find_dictionary, gtpv2_type::*, GTP_DICTIONARY};
use tokio::sync::mpsc::{Receiver, Sender};
use crate::peers::*;


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
            t_flag: bool, mp_flag: bool,
            t: u8, l: u16, teid: u32,
            s: u32, mp: u8) -> (&[u8], usize)
    {

        let mut v = (GTP_VERSION & 0b111) << 5; // Version은 상위 3비트
        let mut len = 8; //without TEID Field.
        if p_flag {
            v |= 0b0001_0000; // P-flag 설정
        }
        if t_flag {
            v |= 0b0000_1000; // T-flag 설정
            len += 4; // Add 4bytes for TEID Field
        }

        let mut s = s << 8;
        if mp_flag {
            v |= 0b0000_0100; // T-flag 설정
            s |= mp as u32;
        }

        let l_bytes     = l.to_be_bytes();
        let teid_bytes  = teid.to_be_bytes();
        let s_bytes     = s.to_be_bytes();

        if t_flag {
            p[..len].copy_from_slice(
                &[ v, t,
                l_bytes[0], l_bytes[1],
                teid_bytes[0], teid_bytes[1], teid_bytes[2], teid_bytes[3],
                s_bytes[0], s_bytes[1], s_bytes[2], s_bytes[3] ]);
        }
        else {
            p[..len].copy_from_slice(
                &[ v, t,
                l_bytes[0], l_bytes[1],
                s_bytes[0], s_bytes[1], s_bytes[2], s_bytes[3], ]);
        }

        (&p[..len as usize], len as usize)
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
    /* value is variable lentgh */

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


// Get IE

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

pub fn gtpv2_get_ie_tbcd(data: &[u8], val: &mut [u8]) -> Result<(), &'static str> {
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




pub fn make_gtpv2(msg_type: u8, peer: &mut Peer) {
    let mut buffer: [u8; 1024] = [0; 1024]; 
    let mut len :usize = 0;

    let msg_info = find_dictionary(msg_type);

    // println!("{:#?}", msg_info);
    // let pkt = Gtpv2CHeader::new();

    (_, len) = Gtpv2CHeader::encode ( &mut buffer, false,
        false, false,
        msg_type, 9, 0, peer.tseq, 0) ;

    for item in msg_info.unwrap().ie_list {
        // println!("{}: {}, {}", item.ie_type,gtpv_ie_type_vals[item.ie_type as usize], item.presence);
        if item.presence == "MANDATORY" {
            len += create_ie(&mut buffer[len..], item.ie_type,177);
        }
    }
    // println!("Len: {} ==>{:?}",len, &buffer[..len]);
    peer.update_last_echo_snd_time();
    peer.increase_count();

}
