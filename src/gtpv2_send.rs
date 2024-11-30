use crate::gtp_msg::*;
// use crate::gtpv2_send::*;
use crate::gtpv2_type::*;
// use udpsock;
use bytemuck::{Pod, Zeroable};


pub fn create_gtpv2_header(
    // peer: GTP2_PEER
)
    -> Vec<u8>
{

    // let mut msg = [u8; 1024];
    // let mut buffer:Vec<u8> = Vec::new();
    // let mut buffer: Vec<u8> = vec![0; 1024];
    let mut buffer: [u8; 40] = [0; 40]; 

    let (_, mut len) =
    Gtpv2CHeader::encode(
        &mut buffer, 
        false, false, false, GTPV2C_ECHO_REQ,
        9, 0, 1, 0) ;
    len += create_ie(&mut buffer[len..], GTPV2C_IE_RECOVERY,177);

    println!("len:{}, {:?}",len, &buffer[..len]);

    buffer[..len].to_vec()
}