use std::net::Ipv4Addr;
use log::{debug, error, info, trace, warn};
use crate::gtpv2_type::*;

pub const PDN_TYPE_IPV4:u8 =  1;
pub const PDN_TYPE_IPV6:u8 =  2;

fn bcd_encode(a: u8, b: u8) -> u8 {
    (a << 4) | (b & 0x0F)
}


fn _dec_tbcd(input: &[u8], n: usize, output: &mut Vec<u8>) -> usize {
    if n < 6 || n > 9 {
        return 0;
    }

    let mut c = 0;

    for i in 0..n - 1 {
        output.push((input[i] & 0x0F) + b'0');
        output.push((input[i] >> 4) + b'0');
        c += 2;
    }

    output.push((input[n - 1] & 0x0F) + b'0');
    c += 1;

    if (input[n - 1] & 0xF0) != 0xF0 {
        output.push((input[n - 1] >> 4) + b'0');
        c += 1;
    }

    c
}


fn _enc_tbcd(input: &[u8], length: usize, output: &mut [u8])
-> usize {
    if length < 2 || length > 32 {
        return 0;
    }

    let mut c = 0;

    if length % 2 == 1 {
        // Odd length
        for i in (0..length - 1).step_by(2) {
			output[c] = bcd_encode(input[i + 1], input[i]);
			c += 1;
        }
        output[c] = bcd_encode(0xFF, input[length - 1]);
        c += 1;

    } else {
        // Even length
        for i in (0..length).step_by(2) {
            output[c] = bcd_encode(input[i + 1], input[i]);
            c += 1;
        }
    }

    c
}


fn _enc_mccmnc(s: &[u8], n: usize, r: &mut Vec<u8>)
-> usize {
    if !(n == 5 || n == 6) {
        return 0;
    }

    let mut c = 0;

    r.push(bcd_encode(s[1], s[0]));
    c += 1;

    if n == 5 {
        r.push(bcd_encode(0xFF, s[2]));
        c += 1;
    }
    else {
        r.push(bcd_encode(s[5], s[2]));
        c += 1;
    }

    r.push(bcd_encode(s[4], s[3]));
    c += 1;

    c
}


pub fn gtpv2_add_ie_tv1<'a>( msg: &'a mut [u8;1024],
    ie_type: u8, instance: u8, value: u8)
-> usize {
    let mut pos: usize = 0;

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
    ie_type: u8, instance: u8, value: u16)
-> usize {
    let mut pos: usize = 0;

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
    msg[pos] = instance & 0x00ff;
    pos += 1;

    //IE Value: n bytes
    msg[pos..pos + len].copy_from_slice(&value[..len]);
    pos += len;

    pos
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


pub fn gtpv2_add_ie_paa( msg: &mut [u8;1024], instance: u8,
    pdn_type: u8, addr: Ipv4Addr)
-> usize {
    let mut buf = [0u8; 4];
    let mut len = 0;

    if pdn_type == PDN_TYPE_IPV4 {
        buf = addr.octets();
        len = 4;
    }
    else if pdn_type == PDN_TYPE_IPV6 {
        buf = addr.octets();
        len = 16;
    }

    gtpv2_add_ie_tlv(msg, GTPV2C_IE_PAA, instance, &buf, len)
}


pub fn gtpv2_add_ie_tbcd( msg: &mut [u8; 1024],
    msg_type: u8, instance: u8, value: &[u8])
-> usize {
    let mut buf = vec![0u8; 64];
    
    let len = _enc_tbcd(value, value.len(), &mut buf);
    if len <= 0 {
        error!("Fail to encode the TBCD");
        return 0;
    }

    gtpv2_add_ie_tlv(msg, msg_type, instance, &buf[..len], len )
}


pub fn gtpv2_add_ie_mccmnc( msg: &mut [u8; 1024], 
    msg_type: u8, instance: u8, value: &[u8])
-> usize {
    let mut buf = vec![0u8; 64];

    let len = _enc_mccmnc(value, value.len(), &mut buf);
    if len <= 0 {
        error!("Fail to encode the MCCMNC");
        return 0;
    }

    gtpv2_add_ie_tlv(msg, msg_type, instance, &buf[..len], len)
}
