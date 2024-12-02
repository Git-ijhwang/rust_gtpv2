use std::net::Ipv4Addr;
use std::str::FromStr;

use crate::gtp_msg::*;
// use crate::gtpv2_send::*;
use crate::gtpv2_type::*;
use crate::packet::*;
use crate::timer::*;

pub async fn
create_gtpv2_header(pkt: &mut EncapPkt/* peer: GTP2_PEER */) -> 
// (Vec<u8>, usize)
 usize
{
    // let mut msg = [u8; 1024];
    // let mut buffer:Vec<u8> = Vec::new();
    // let mut buffer: Vec<u8> = vec![0; 1024];
    let mut buffer: [u8; 40] = [0; 40]; 
    let mut len :usize = 0;


     (_, len) = Gtpv2CHeader::encode( &mut buffer, 
        false, false, false, GTPV2C_ECHO_REQ, 9, 0, 1, 0) ;

    len += create_ie(&mut buffer[len..], GTPV2C_IE_RECOVERY,177);

    println!("len:{}, {:?}",len, &buffer[..len]);
    // let mut pkt = EncapPkt::new(Ipv4Addr::from_str("192.168.2.1").unwrap(), GTPV2C_ECHO_REQ);
    // pkt.copy_data(&buffer, len);
    // pkt.put_que(&mut msgque);
    pkt.data[..len].copy_from_slice(&buffer[..len]);
    pkt.datalen = len;


    // return (buffer[..len].to_vec(), len);
    len
}