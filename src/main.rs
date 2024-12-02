mod gtpv2_type;
mod gtpv2_send;
mod gtp_msg;
mod config;
mod udpsock;
mod packet;
mod timer;

use gtp_msg::*;
use gtpv2_type::{gtpv_ie_type_vals, gtpv_msg_type_vals, GTPV2C_ECHO_REQ};
use gtpv2_send::*;
use timer::EncapPkt;
use tokio::time::{self, Duration};
use std::sync::{Arc, Mutex};
use std::{net::Ipv4Addr, str::FromStr};
use config::*;
use timer::*;


#[tokio::main]
async fn main()
{
    /* Read Config */
    // _ = read_conf("src/config");
    let config = CONFIG_MAP.read().unwrap();
    _ = read_peer("src/config_peer");

    let  queue = Arc::new(Mutex::new(MsgQue::new()));
    // Peer::print();

    let peer = Peer::get_peer(Ipv4Addr::from_str("192.168.2.1").unwrap());
    let mut pkt = EncapPkt::new(Ipv4Addr::from_str("192.168.2.1").unwrap(), GTPV2C_ECHO_REQ);

    // let len =
    create_gtpv2_header(&mut pkt);

    println!("Assume message sent");

    pkt.put_que(&mut queue.lock().unwrap());
    // pkt.copy_data(&buffer, len);
    println!("put queued");
    queue.lock().unwrap().check_timer().await;

    loop {
        let mut cnt = 0;
        tokio::time::sleep(Duration::from_secs(1)).await; //sleep 100ms
        cnt += 1;
        println!("Main Func: {}", cnt);
    }
    println!("End.");

}
