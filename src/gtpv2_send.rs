use std::io;
use std::sync::Arc ;
use std::net::Ipv4Addr;
use std::net::UdpSocket;

use crate::gtp_msg::*;
use crate::gtpv2_type::*;
use crate::timer::*;
use crate::peers::*;


pub fn gtpv2_add_ie_tlv( msg:&mut[u8;1024], ie_type: u8,
    instance: u8, value: &[u8], len: usize)
-> usize {

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

    //IE Type: 1byte
    msg[pos] = ie_type;
    pos+=1;

    //IE Length: 2bytes
    msg[pos..pos + 2].copy_from_slice(&(1 as u16).to_be_bytes());
    pos += 2;

    //CR flag and Instance: 1byte
    msg[pos] = instance & 0x00ff;
    pos +=1;

    //IE Value: 1 byte
    msg[pos] = value;
    pos += 1;

    pos
}

pub fn gtpv2_add_ie_tv2<'a>( msg: &'a mut [u8; 1024],
ie_type: u8, instance: u8, value: u16, len: usize)
-> usize {
    let mut pos: usize = len;

    // IE Type: 1 byte
    msg[pos] = ie_type;
    pos += 1;

    // IE Length: 2 bytes (fixed 0x0002)
    msg[pos..pos + 2].copy_from_slice(&2u16.to_be_bytes());
    pos += 2;

    // CR flag and Instance: 1 byte
    msg[pos] = instance & 0x00ff;
    pos += 1;

    // IE Value: 2 bytes (big-endian)
    msg[pos..pos + 2].copy_from_slice(&value.to_be_bytes());
    pos += 2;

    pos
}

pub fn gtpv2_add_ie_tv4<'a>( msg: &'a mut [u8; 1024], 
    ie_type: u8, instance: u8, value: u32, len: usize)
-> usize {
    let mut pos: usize = len;

    // IE Type: 1 byte
    msg[pos] = ie_type;
    pos += 1;

    // IE Length: 2 bytes (fixed 0x0004)
    msg[pos..pos + 2].copy_from_slice(&4u16.to_be_bytes());
    pos += 2;

    // CR flag and Instance: 1 byte
    msg[pos] = instance & 0x00ff;
    pos += 1;

    // IE Value: 4 bytes (big-endian)
    msg[pos..pos + 4].copy_from_slice(&value.to_be_bytes());
    pos += 4;

    pos
}

pub fn send_udp_data(data: &[u8], ip: &str, port: u16)
-> io::Result<()> {
    // Create a new UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?; // Binds to an available port on the local machine

    // Format the target address
    let addr = format!("{}:{}", ip, port);

    // Send the data
    socket.send_to(data, addr)?;

    Ok(())
}


pub async fn make_gtpv2(msg_type: u8, body: &[u8;1024],
    oldpeer: Peer, t_flag: bool, len: u8)
-> Result<(), String> {
    let mut peer;
    let mut length :usize = 0;
    let bodylen: u16 = len as u16;
    let mut buffer: [u8; 1024] = [0; 1024]; 

    let result = get_peer(&oldpeer.ip);
    match result {
        Ok(value) => peer = value,
        _ => return  Err ("Fail to get peer".to_string()),
    }

    (_, length) = Gtpv2CHeader::encode (&mut buffer, false,
                        t_flag, false, msg_type,
                        bodylen, oldpeer.teid, peer.tseq, 0) ;

    buffer[length..length + len as usize].copy_from_slice(&body[..len as usize]);

    let pkt = EncapPkt::new(peer.ip, msg_type, buffer[..length + len as usize].to_vec());

    send_udp_data(&buffer[..length+len as usize], &peer.ip.to_string(), peer.port);

    let mut queue_locked = SHARED_QUEUE.lock().await;
    queue_locked.push(pkt);

    peer.update_last_echo_snd_time();
    peer.increase_count();

    Ok(())
}

