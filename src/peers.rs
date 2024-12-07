use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Mutex;
use std::collections::HashMap;
extern crate lazy_static;
use lazy_static::lazy_static;
use crate::config::*;
use crate::gtp_msg::*;
use crate::gtpv2_type::*;

lazy_static! {
    pub static ref GTP2_PEER: Mutex<HashMap<u32, Peer>> = Mutex::new(HashMap::new());
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Peer {
    pub ip:             Ipv4Addr,
    pub port:           u16,    // peer port 
    node_type:          String,
    msglen:             u16,    // msg length 
    version:            u8,     // gtp version 
    status:             bool,     // gtp version 
    resend_count:       u8,

    teid:               u32,    // Recieved Sequence number
    rseq:               u32,    // Recieved Sequence number
    tseq:               u32,    // Transmit Sequence number 
}


impl Peer {

    pub fn new(ip: Ipv4Addr, port: u16, node_type: String) -> Self {
        Peer {
            ip,
            port,
            node_type,
            msglen:     0,
            version:    GTP_VERSION,
            status:     false,
            teid:       0,
            rseq:       0,
            tseq:       0,
            resend_count: 0,
        }
    }

    pub fn check_peer(ip: Ipv4Addr) -> Result<Self,()> {
        let list = GTP2_PEER.lock().unwrap();
        let key = u32::from(ip).into();

        if let Some(peer) = list.get(&key) {
            Ok( peer.clone())
        }
        else {
            return  Err(());
        }
    }

    pub fn put_peer(peer: Peer) {
        let mut list = GTP2_PEER.lock().unwrap();
        if let Some(key) = u32::from(peer.ip).into() {
            list.insert(key, peer);
        }
    }

    pub fn change_peer_status(&mut self) -> bool {
        self.status = !self.status;
        self.status
    }


    pub fn get_peer_status(& self) -> bool {
        self.status
    }

    pub fn deactivate_peer_status(& mut self) -> bool {
        self.status = false;
        self.status
    }

    pub fn activate_peer_status(& mut self) -> bool {
        self.status = true;
        self.status
    }

    pub fn increase_count(&mut self) {
        self.resend_count += 1;
    }

    pub fn get_count(&self) -> u8 {
        self.resend_count
    }

    pub fn reset_count(&mut self){
        self.resend_count = 0;
    }

    pub fn print() {
        let list = GTP2_PEER.lock().unwrap();
        println!("{:#?}", list);
    }

}


pub fn create_peer () {
    let peers = PEER_MAP.read().unwrap();

    for (key, value) in peers.iter() {
        let parts: Vec<&str> = value.split(':').collect();
        let ip = Ipv4Addr::from_str(parts[0]).unwrap();
        let port = u16::from_str(parts[1]).unwrap();

        let mut peerlist = GTP2_PEER.lock().unwrap();
        let peer = Peer::new(ip, port, key.to_string());
        peerlist.insert(u32::from(peer.ip), peer);
    }

    let peerlist = GTP2_PEER.lock().unwrap();

    // peerlist.iter().for_each(|x| {
    //     println!("{:?}", x);
    // });
}


pub fn get_peer(ip: &Ipv4Addr) -> Result<Peer, ()> {
    let list = GTP2_PEER.lock().unwrap();
    let key = u32::from(*ip).into();

    if let Some(peer) = list.get(&key) {
        Ok( peer.clone())
    }
    else {
        Err(())
    }
}
