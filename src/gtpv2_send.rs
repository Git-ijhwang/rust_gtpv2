use crate::gtp_msg::*;
// use crate::gtpv2_send::*;
use crate::gtpv2_type::*;
// use udpsock;
use bytemuck::{Pod, Zeroable};


fn make_ie1(t: u8, v:u8, ){
    let ie = gtpv2c_ie1::new(t, 0, v);
}

fn create_gtpv2_header(peer: GTP2_PEER) {

    // let mut msg = [u8; 1024];
    // let mut buffer:Vec<u8> = Vec::new();
    // let mut buffer: Vec<u8> = vec![0; 1024];
    let mut buffer: [u8; 40] = [0; 40]; 

    Gtpv2CHeader::new(&mut buffer, 
        false, false, GTPV2C_ECHO_REQ,
        9, 0x3214, 0x3444) ;
    println!("{:?}", buffer);
    // let hdr = Gtpv2CHeader {
    //     v: 2,
    //     f: 0b0001_0000,
    //     t: 1,
    //     l: 9,
    //     teid: 0x3214,
    //     s: 0x3444,
    // };
    // let hdr_bytes = bytemuck::cast_slice(&[hdr]);
    // println!("Header bytes: {:?}", hdr_bytes);
    // let ie = make_ie(GTPV2C_IE_RECOVERY, 1 );


}