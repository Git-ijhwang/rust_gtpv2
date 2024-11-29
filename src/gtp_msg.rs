#[macro_use]
use std::net::UdpSocket;
extern crate lazy_static;

use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::Mutex;
use lazy_static::lazy_static;
 
use bytemuck::{Pod, Zeroable};

const GTPV2C_PEER_MME: u16 = 1;
const GTPV2C_PEER_SGW: u16 = 2;
const GTPV2C_PEER_PGW: u16 = 3;

const GTPV2_VERSION: u8 = 2;


#[repr(C)]
#[derive(Debug, Clone)]
pub struct Peer {
    pub ip:             Ipv4Addr,
    pub port:           u16,    // peer port 
    msglen:             u16,    // msg length 
    version:            u8,     // gtp version 
    status:            u8,     // gtp version 

    teid:           u32,    // Recieved Sequence number
    rseq:           u32,    // Recieved Sequence number
    tseq:           u32,    // Transmit Sequence number 
}

lazy_static! {
    pub static ref GTP2_PEER: Mutex<HashMap<u32, Peer>> = Mutex::new(HashMap::new());
}

impl Peer {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {

        Peer {
            ip,
            port,
            msglen: 0,
            version: GTPV2_VERSION,
            status: 1, //defalut 1
            teid: 0,
            rseq: 0,
            tseq: 0,
        }
    }

    pub fn put_peer(peer: Peer) {
        let mut list = GTP2_PEER.lock().unwrap();
        if let Some(key) = u32::from(peer.ip).into() {
            list.insert(key, peer);
        }
    }

    pub fn get_peer(ip: Ipv4Addr) -> Result<Self,()> {
        let list = GTP2_PEER.lock().unwrap();
        let key = u32::from(ip).into();

        if let Some(peer) = list.get(&key) {
            Ok( peer.clone())
        }
        else {
            return  Err(());
        }
    }

    pub fn print() {
        let list = GTP2_PEER.lock().unwrap();
        println!("{:#?}", list);

    }

}



// #[derive(Debug, Clone, Copy)]
#[repr(C)]
// #[derive(Debug, Clone, Copy, Pod, Zeroable)] // bytemuck의 Pod와 Zeroable 트레잇 구현
pub struct Gtpv2CHeader {
    pub v: u8,  // Version (3 bits) + P flag (1 bit) + T flag (1 bit) + 3 spare bits
    pub t: u8,      // Message type (8 bits)
    pub l: u16,   // Message length (16 bits)
    pub teid: u32,     // TEID (32 bits), optional based on T flag
    pub s: u16,  // Sequence number (24 bits) + 8 spare bits

    pub ies: Vec<u8>,
}

impl Gtpv2CHeader {
    /// GTPv2-C 헤더를 초기화하는 함수
    pub fn new (
            buffer: &mut [u8],
            p_flag: bool,
            t_flag: bool,
            t: u8,
            l: u16,
            teid: u32,
            s: u16
        ) {
        let mut v = (GTPV2_VERSION & 0b111) << 5; // Version은 상위 3비트
        if p_flag {
            v |= 0b0001_0000; // P-flag 설정
        }
        if t_flag {
            v |= 0b0000_1000; // T-flag 설정
        }

        buffer[0] = v;
        buffer[1] = t;
        buffer[2..4].copy_from_slice(&l.to_be_bytes() );
        buffer[4..8].copy_from_slice(&teid.to_be_bytes() );
        buffer[8..10].copy_from_slice(&s.to_be_bytes() );
    }

    /// T-flag로 TEID의 존재 여부를 반환
    pub fn has_teid(&self) -> bool {
        self.v & 0b0000_1000 != 0
    }

    pub fn encode_gtp_hdr<'a>(&'a self, p: &'a mut [u8]) -> &[u8] {
        let teid_bytes: [u8; 4] = self.teid.to_be_bytes();

        let l_bytes = self.l.to_be_bytes();
        let s_bytes = self.s.to_be_bytes();

        p[..10].copy_from_slice(&[
            self.v,
            self.t,
            l_bytes[0],                                    // Elapsed seconds MSB
            l_bytes[1],                                    // Elapsed seconds LSB
            teid_bytes[0],                                     // Transaction ID MSB
            teid_bytes[1],                                     // Transaction ID 중간 바이트
            teid_bytes[2],                                     // Transaction ID 중간 바이트
            teid_bytes[3],                                     // Transaction ID LSB
            s_bytes[0],                                    // Elapsed seconds MSB
            s_bytes[1],                                    // Elapsed seconds LSB
        ]);

        &p[..10]
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
    pub fn new(t: u8, i: u8, v: u8) -> Self {
        gtpv2c_ie1 {
	        t,
	        l : u16::to_be(0x00001),
	        i : i & 0x00ff,
	        v,
        }
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



