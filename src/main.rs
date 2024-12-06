mod gtpv2_type;
mod gtpv2_send;
mod gtp_msg;
mod config;
mod udpsock;
mod packet;
mod timer;
mod gtp_dictionary;
mod recv_gtpv2;
mod validate_gtpv2;
mod peers;

use config::*;
use gtp_msg::*;
use std::thread;
use std::io::Error;
use core::result::Result;
// use gtpv2_type::{gtpv_ie_type_vals, gtpv_msg_type_vals, GTPV2C_ECHO_REQ};
// use gtpv2_send::*;
// use timer::EncapPkt;
use tokio::time::{self, Duration};
// use std::sync::{Arc, Mutex};
// use std::{net::Ipv4Addr, str::FromStr};
// use timer::*;
use crate::peers::*;
use crate::udpsock::*;
use crate::recv_gtpv2::*;
use crate::gtp_dictionary::*;


#[tokio::main]
async fn main() -> Result<(), String>
{
    /* Read Config */
    read_conf("config/config", false);
    _ = read_conf("config/config_peer", true);

    let dictionary = load_gtp_dictionary("config/GTPv2_Dictionary.json");

    create_peer();
    // let  queue = Arc::new(Mutex::new(MsgQue::new()));
    let bind_addr = CONFIG_MAP.read().unwrap();

    // let peer = Peer::get_peer(Ipv4Addr::from_str("192.168.2.1").unwrap());
    // let mut pkt = EncapPkt::new(Ipv4Addr::from_str("192.168.2.1").unwrap(), GTPV2C_ECHO_REQ);
    // create_gtpv2_header(&mut pkt);
    // pkt.put_que(&mut queue.lock().unwrap());
    // // pkt.copy_data(&buffer, len);
    // println!("put queued");
    // queue.lock().unwrap().check_timer().await;
    // recv_gtpv2();
    let recv_socket = socket_create(
        format!("0.0.0.0:{}",bind_addr.get("SrcPort").unwrap())
    )?;

    thread::spawn(move || gtpv2_recv_task(recv_socket));

    loop {
        let mut cnt = 0;
        tokio::time::sleep(Duration::from_secs(1)).await; //sleep 100ms
        cnt += 1;
        println!("Main Func: {}", cnt);
    }

}
