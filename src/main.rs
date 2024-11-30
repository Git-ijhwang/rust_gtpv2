mod gtpv2_type;
mod gtpv2_send;
mod gtp_msg;
mod config;
mod udpsock;
mod packet;

use gtp_msg::Peer;
use gtpv2_type::{gtpv_msg_type_vals, gtpv_ie_type_vals};
use config::*;


fn main() {
    /* Read Config */
    // _ = read_conf("src/config");
    let config = CONFIG_MAP.read().unwrap();
    // _ = read_peer("src/config_peer");

    // Peer::print();

    let buf = gtpv2_send::create_gtpv2_header();
    // let socket = socket_create(src_bind);
    // if socket.is_err() {
    //     println!("{}", socket.unwrap_err());
    //     return Err(());
    // }

}
