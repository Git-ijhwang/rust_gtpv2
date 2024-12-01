mod gtpv2_type;
mod gtpv2_send;
mod gtp_msg;
mod config;
mod udpsock;
mod packet;
mod timer;

use gtp_msg::{Peer, GTP2_PEER};
use gtpv2_type::{gtpv_msg_type_vals, gtpv_ie_type_vals};

use tokio::time::{self, Duration};
use std::{net::Ipv4Addr, str::FromStr};

use tokio::sync::mpsc;
// use std::sync::mpsc::channel;
use tokio::task;
use timer::start_timer;
use tokio::sync::oneshot;



use config::*;


#[tokio::main]
async fn main() {
    /* Read Config */
    // _ = read_conf("src/config");
    let config = CONFIG_MAP.read().unwrap();
    _ = read_peer("src/config_peer");

    Peer::print();

    
    let peers = GTP2_PEER.lock().unwrap();
    let key = u32::from(Ipv4Addr::from_str("192.168.2.1").unwrap()) ;
    let peer = peers.get(&key);

    //timer
    let (tx, mut rx) = mpsc::channel::<u64>(10);
    let (tx2, mut rx2) = mpsc::channel::<u64>(10);
    // peer.spawn_echo_req_task(rx);
    peer.unwrap().spawn_echo_req_task(rx2);
    peer.unwrap().spawn_stop_echo_req(rx, tx2);

    println!("End.");

}
