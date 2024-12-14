use std::net::Ipv4Addr;
use std::str::FromStr;
use std::net::UdpSocket;
use std::io;

use crate::gtp_msg::*;
use crate::gtpv2_type::*;
use crate::packet::*;
use crate::timer::*;

pub async fn
create_gtpv2_header(pkt: &mut EncapPkt/* peer: GTP2_PEER */) -> usize
// (Vec<u8>, usize)
{
    let mut buffer: [u8; 40] = [0; 40]; 
    let mut len :usize = 0;

    (_, len) = Gtpv2CHeader::encode(
        &mut buffer, false,
        false, false,
        GTPV2C_ECHO_REQ, 9, 0, 1, 0) ;

    len += create_ie(&mut buffer[len..], GTPV2C_IE_RECOVERY,177);

    pkt.data[..len].copy_from_slice(&buffer[..len]);
    pkt.datalen = len;

    len
}



pub fn gtpv2_add_ie_tlv(
    msg:&mut[u8;1024], ie_type: u8,
    instance: u8, value: &[u8], len: usize)
-> usize {

    // let mut buffer:[u8;1024] = [0u8;1024];

    // Length check for safety
    // if len == 0 || len > 256 {
    //     eprintln!("(GTPv2-PROC) gtpv2_add_ie_tlv: length error");
    //     return Err("Invalid length".to_string());
    // }

    // Calculate required buffer size
    // let required_size = self.msglen + 4 + length;
    // if required_size > self.msgbuf.len() {
    //     eprintln!("(GTPv2-PROC) gtpv2_add_ie_tlv: buffer overflow");
    //     return Err("Buffer overflow");
    // }

    // Cast buffer slice to the GTPv2cIeTlv structure
    let mut pos = 0;

    //IE Type: 1byte
    msg[pos] = ie_type; 
    pos += 1;

    //IE Length: 2bytes
    msg[pos..pos + 2].copy_from_slice(&(len as u16).to_be_bytes());
    pos += 2;

    //CR flag and Instance: 1byte
    msg[pos ] = instance & 0x00ff;
    pos += 1;

    //IE Value: n bytes
    msg[pos..pos + len].copy_from_slice(&value[..len]);
    pos += len;

    pos
}


pub fn gtpv2_add_ie_paa( msg: &mut [u8;1024],
    instance: u8, pdn_type: u8, addr: Ipv4Addr, len: u8)
->usize {
    let mut buf = [0u8; 4];

    if pdn_type == 1 {
        buf = addr.octets();
    }

    gtpv2_add_ie_tlv(msg, GTPV2C_IE_PAA, instance, &buf, 4)
}


pub fn gtpv2_add_ie_cause( msg: &mut [u8;1024],
    instance: u8, cause: u8, flags: u8,
    ie_type: Option<u8>, ie_instance: u8,)
->usize {
    let mut buf = [0u8; 64];
    let mut p = 0;

    buf[p] = cause; p += 1;
    buf[p] = flags; p += 1;

    if let Some(ie_type_value) = ie_type {
        buf[p] = ie_type_value; p += 1;
        // Reserved byte
        buf[p] = 0; p += 1;
        // Reserved byte
        buf[p] = 0; p += 1;

        // Lower 4 bits of `ie_instance`
        buf[p] = ie_instance & 0x0f; p += 1;
    }

    gtpv2_add_ie_tlv(msg, GTPV2C_IE_CAUSE, instance, &buf, p)
}

pub fn gtpv2_add_ie_tv1<'a>( msg: &'a mut [u8;1024],
    ie_type: u8, instance: u8, value: u8, len: usize)
-> usize {
    let mut pos:usize = len;

    // Create the IE
    msg[pos] = ie_type;
    pos+=1;

    msg[pos..pos + 2].copy_from_slice(&(1 as u16).to_be_bytes());
    pos += 2;

    msg[pos] = instance & 0x00ff;
    pos +=1;

    msg[pos] = value;
    pos += 1;

    pos
}





pub fn send_udp_data(data: &[u8], ip: &str, port: u16) -> io::Result<()> {
    // Create a new UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?; // Binds to an available port on the local machine

    // Format the target address
    let addr = format!("{}:{}", ip, port);

    // Send the data
    socket.send_to(data, addr)?;

    println!("Data sent to {}:{}", ip, port);
    Ok(())
}
