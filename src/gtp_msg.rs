#[macro_use]
use std::net::UdpSocket;
extern crate lazy_static;

use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::Mutex;
use lazy_static::lazy_static;
 

const GTPV2C_PEER_MME: i16 = 1;
const GTPV2C_PEER_SGW: i16 = 2;
const GTPV2C_PEER_PGW: i16 = 3;


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
            version: 2,
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


pub struct Clients {
    pub port:           u8,
    pub socket:         UdpSocket,
    pub tranxid:        [u8;4],
    pub hostname:       String,
    pub hw_addr:        [u8; 16],
    pub magic_cookie:   [u8; 4],
    pub msg_type:       u8,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Gtpv2CHeader {
    pub version_and_flags: u8,  // Version (3 bits) + P flag (1 bit) + T flag (1 bit) + 3 spare bits
    pub message_type: u8,      // Message type (8 bits)
    pub message_length: u16,   // Message length (16 bits)
    pub teid: Option<u32>,     // TEID (32 bits), optional based on T flag
    pub sequence_number: u32,  // Sequence number (24 bits) + 8 spare bits
}

impl Gtpv2CHeader {
    /// GTPv2-C 헤더를 초기화하는 함수
    pub fn new(version: u8, p_flag: bool, t_flag: bool, message_type: u8, message_length: u16, teid: Option<u32>, sequence_number: u32) -> Self {
        let mut version_and_flags = (version & 0b111) << 5; // Version은 상위 3비트
        if p_flag {
            version_and_flags |= 0b0001_0000; // P-flag 설정
        }
        if t_flag {
            version_and_flags |= 0b0000_1000; // T-flag 설정
        }

        Gtpv2CHeader {
            version_and_flags,
            message_type,
            message_length,
            teid,
            sequence_number,
        }
    }

    /// T-flag로 TEID의 존재 여부를 반환
    pub fn has_teid(&self) -> bool {
        self.version_and_flags & 0b0000_1000 != 0
    }

}


pub fn send_gtpv2() {
    
}