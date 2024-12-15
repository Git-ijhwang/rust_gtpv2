use std::net::UdpSocket;

use crate::gtp_msg::*;
use crate::timer::*;
use crate::peers::*;


pub fn send_udp_data(data: &[u8], ip: &str, port: u16)
-> Result<usize, String> {
    // Create a new UDP socket
    // Binds to an available port on the local machine
    let socket;
    let ret = UdpSocket::bind("0.0.0.0:0");
    match ret {
        Ok(v) => socket = v,
        _ => return Err ("Socket Open Error occoured".to_string()),
    }


    // Format the target address
    let addr = format!("{}:{}", ip, port);

    // Send the data
    let ret = socket.send_to(data, addr);
    match ret {
        Ok(v) => Ok(v),
        _ => Err ("Send Error occoured".to_string()),
    }
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

    let ret = send_udp_data(&buffer[..length+len as usize], &peer.ip.to_string(), peer.port);
    match ret {
        Ok(v) => {
            if v <= 0 {
                return  Err ("Fail to send message to peer".to_string());
            }

            let mut queue_locked = SHARED_QUEUE.lock().await;
            queue_locked.push(pkt);

            peer.update_last_echo_snd_time();
            peer.increase_count();

            Ok(())
        },
        _ => return  Err ("Fail to send message to peer".to_string()),
    }

}

