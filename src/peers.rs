use std::net::AddrParseError;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Mutex;
use std::collections::HashMap;
use lazy_static::lazy_static;
use tokio::time::{self, Duration};
use crate::gtpv2_type::*;
use crate::config::*;
use crate::gtp_msg::*;
// use std::sync::{Arc, Mutex};
use std::sync::MutexGuard;
extern crate lazy_static;
use std::time::Instant;


lazy_static! {
    pub static ref GTP2_PEER: Mutex<HashMap<u32, Peer>> = Mutex::new(HashMap::new());
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Peer {
    pub ip:             Ipv4Addr,
    pub port:           u16,    // peer port 
    pub node_type:          String,
    pub msglen:             u16,    // msg length 
    pub version:            u8,     // gtp version 
    pub status:             bool,     // gtp version 
    pub resend_count:       u8,

    pub teid:               u32,    // Recieved Sequence number
    pub rseq:               u32,    // Recieved Sequence number
    pub tseq:               u32,    // Transmit Sequence number 

    pub last_active_time:   Instant,   // 마지막 활동 시간
    pub last_echo_snd_time: Instant,   // 마지막  시간
    pub tx_count:           u32,              // 전송된 메시지 수
    pub rx_count:           u32,              // 수신된 메시지 수
    pub error_count:        u16,              // 에러 횟수
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
            last_active_time: Instant::now(),
            last_echo_snd_time: Instant::now(),
            tx_count:   0,
            rx_count:   0,
            error_count:    0,
        }
    }

    pub fn update_last_echo_snd_time(&mut self) {
        self.last_echo_snd_time = Instant::now();
    }

    pub fn update_last_active(&mut self) {
        self.last_active_time = Instant::now();
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

        let conf = CONFIG_MAP.read().unwrap();
        let mut peerlist = GTP2_PEER.lock().unwrap();
        let mut peer = Peer::new(ip, port, key.to_string());
        let addr = conf.get("Addr").unwrap();
        if let Ok(ip) = Ipv4Addr::from_str(&addr) {
            peer.tseq = u32::from(ip) ;
        }
        else {
            peer.tseq = 1;
        }

        // peer.tseq = u32::from( Ipv4Addr::from_str( &conf.get("Addr").unwrap())?).into();
        // peer.tseq = tseq;
        peerlist.insert(u32::from(peer.ip), peer);
    }

    // let peerlist = GTP2_PEER.lock().unwrap();

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

pub async fn peer_manage()
{
    let config = CONFIG_MAP.read().unwrap();

    let timeout     = config.get("GTPv2_MSG_TIMEOUT").unwrap().parse::<u8>().unwrap() ;
    let rexmit_cnt  = config.get("GTPv2_RETRANSMIT_COUNT").unwrap().parse::<u8>().unwrap() ;
    let echo_period = config.get("ECHO_PERIOD").unwrap().parse::<u64>().unwrap() ;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;

        let mut peers = GTP2_PEER.lock().unwrap();
        for (key, peer) in  peers.iter_mut(){
            // println!("{}, {:?}",key, peer);
            if Instant::now() >= peer.last_echo_snd_time+Duration::from_secs(echo_period) {
                if peer.status == false {
                    //Send Echo Request
                    // peer.activate_peer_status();

                    if peer.get_count() <= rexmit_cnt {
                        make_gtpv2(GTPV2C_ECHO_REQ,  peer);
                    }
                    else {
                        peer.update_last_active();
                        peer.deactivate_peer_status();

                        peer.reset_count();
                    }

                    continue;
                }
            }
        }
        break;
    }
}