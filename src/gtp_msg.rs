#[macro_use]
use std::net::UdpSocket;
extern crate lazy_static;

use std::net::Ipv4Addr;
use std::sync::Mutex;
use lazy_static::lazy_static;
// use std::{net::Ipv4Addr, str::FromStr, sync::mpsc::{Receiver, Sender}};
// use std::sync::mpsc::Receiver;
 
// use tokio::sync::mpsc;
use tokio::time::{self, Duration};
// use bytemuck::{Pod, Zeroable};

use crate::gtpv2_type::*;
// use std::sync::mpsc::channel;
use tokio::sync::mpsc::{Receiver, Sender};

// const GTPV2C_PEER_MME: u16 = 1;
// const GTPV2C_PEER_SGW: u16 = 2;
// const GTPV2C_PEER_PGW: u16 = 3;


    // pub async fn spawn_echo_req_task(self,
    //     mut rx2: Receiver<u64>,
    // ) {
    //     while let Some(delay) = rx2.recv().await {
    //     }
    // }

    // pub fn spawn_stop_echo_req(self,
    //     mut rx: Receiver<u64>,
    //     mut tx2: Sender<u64>
    // ) {
    //     tokio::time::sleep(Duration::from_secs(delay as u64)).await;
    //     thread::spawn(move || {
    //     });
    // }




// #[derive(Debug, Clone, Copy)]
#[repr(C)]
// #[derive(Debug, Clone, Copy, Pod, Zeroable)] // bytemuck의 Pod와 Zeroable 트레잇 구현
pub struct Gtpv2CHeader {
    pub v: u8,  // Version (3 bits) + P flag (1 bit) + T flag (1 bit) + MP flag (1bit + 2 spare bits 
    pub t: u8,      // Message type (8 bits)
    pub l: u16,   // Message length (16 bits)
    pub teid: u32,     // TEID (32 bits), optional based on T flag
    pub s: u32,  // Sequence number (24 bits) + 4  Message Priority + 4 spare bits

    // pub ies: Vec<u8>,
}

impl Gtpv2CHeader {
    /// GTPv2-C 헤더를 초기화하는 함수
    pub fn encode<'a> (p:&'a mut [u8], p_flag: bool,
            t_flag: bool, mp_flag: bool,
            t: u8, l: u16, teid: u32,
            s: u32, mp: u8,) -> (&[u8], usize)
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

        let l_bytes = l.to_be_bytes();
        let teid_bytes = teid.to_be_bytes();
        let s_bytes = s.to_be_bytes();

        if t_flag {
            p[..len].copy_from_slice(
                &[ v,
                t,
                l_bytes[0], l_bytes[1],
                teid_bytes[0], teid_bytes[1], teid_bytes[2], teid_bytes[3],
                s_bytes[0], s_bytes[1], s_bytes[2], s_bytes[3] ]);
        }
        else {
            p[..len].copy_from_slice(
                &[ v,
                t,
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

        &p[..5].copy_from_slice(
            &[ t, l[0], l[1], i, v, ] );
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

