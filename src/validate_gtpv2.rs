use crate::{gtp_dictionary::{self, *}, Gtpv2CHeader};
use std::io::Error;
use core::result::Result;
use crate::gtpv2_type::*;

// 단일 IE 및 중첩된 그룹 검증
fn validate_ie(
    expected_ie: &IeInfo, ies: &[(u8, usize, Vec<(u8, usize)>)], is_top_level: bool
) -> Result<(), String> {

    let mut matches =
            ies.iter().filter(|(ie_type, _, _)| *ie_type  == expected_ie.ie_type);

    match expected_ie.presence.as_str() {
        "MANDATORY" if matches.next().is_none() => {
            return Err(format!("Mandatory IE {}(0x{:x}) is missing",expected_ie.ie_type, expected_ie.ie_type)); 
        }
        "CONTIDIONAL" | "CONTIDIONAL_OPTION" | "OPTIONAL" => {}
        _ => (),
    }

    // 중첩 그룹 검증
    if let Some(group_info) = &expected_ie.group_ie_info {

        if let Some((_, _, nested_ies)) = ies.iter().find
            ( |(ie_type, _, _)| *ie_type  == expected_ie.ie_type ) {
                validate_group_ie(group_info, nested_ies, expected_ie.ie_type)?;
            }
        else {
            return Err(format! ( "Mandatory Group IE {}(0x{:x}) is missing",
                expected_ie.ie_type, expected_ie.ie_type));
        }
    }

    Ok(())
}


fn validate_group_ie(
    group_info: &[IeInfo], nested_ies: &Vec<(u8, usize)>, group_ie_type: u8)
-> Result<(), String> {

    for group_ie in group_info {
        let is_gie_present = nested_ies.iter().any(|(ie_type, _)| *ie_type  == group_ie.ie_type);
        if group_ie.presence == "MANDATORY" && !is_gie_present {
            return Err(format!("Mandatory Group IE {}(0x{:x}) of {} is missing",
                group_ie.ie_type, group_ie.ie_type, group_ie_type)); 
        }
    }
    Ok(())
}

// fn validate_message( msg_type: u8, ies: Vec<(u8, usize, Vec<(u8, usize)>)>, msg_def: &GtpMessage )
// -> Result<(), String> {
//     // let msg_def = dictionary
//     //     .iter()
//     //     .find(|msg| msg.msg_type == msg_type )
//     //     .ok_or(format!("Unknown message type: {}", msg_type))?;

//     for expected_ie in &msg_def.ie_list {
//         validate_ie(expected_ie, &ies, true)?;
//     }

//     Ok(())
// }

// dictionary: &[GtpMessage],
pub fn
validate_length(ies: &Vec<(u8, usize, Vec<(u8, usize)>)>, msg_dictionary: &GtpMessage)
-> Result<bool, String> {

    // let dictionary = GTP_DICTIONARY.read().unwrap();

    // match validate_message(rcv_msg_type, ies.clone(), &msg_dictionary) {
    //     Ok(()) => println!("Valid"),
    //     Err(e) => println!("Error {}", e),
    // }

    for expected_ie in &msg_dictionary.ie_list {
        validate_ie(expected_ie, &ies, true)?;
    }

    let ie_map = IEMap::make_ie_type_map();

    // Validate IE lengths
    for (ie_type, ie_length, _) in ies {
        if let Some(ie_def) =
            msg_dictionary.ie_list.iter().find(|def| def.ie_type == *ie_type ) {
            if ie_length < &ie_def.min_length || ie_length > &ie_def.max_length {
                return Err(format!(
                    "IE {} [{}] length {} is out of bounds (min: {}, max: {})",
                    ie_map.ie_type_to_string(*ie_type), ie_type,
                    ie_length, ie_def.min_length, ie_def.max_length)); 
            }
        }
    }

    Ok(true)
}


pub fn
parse_header (data: &[u8])  ->
// Result<(u8, usize), String>
Result<(Gtpv2CHeader, usize), String>
{
    let mut p : usize = 0;
    let mut teid= 0;
    let seqnum;
    println!("data len : {}", data.len());

    let mut hdr = Gtpv2CHeader::new();
    let version = (data[p] & 0xE0) >> 5;
    if version != GTP_VERSION {
        // send_gtpv2_version_not_supported(peer, peerip, peerport, seqnum);
        return Err(format!("Version is not supported {}", version).to_string());
    }

    //get Piggyback flag
    let pflag = (data[p] & GTPV2_P_FLAG) != 0;
    //get Teid flag
    let tflag = (data[p] & GTPV2_T_FLAG) != 0; p+=1;

    // let msg_type_map = make_msg_type_map();
    let msg_type = data[p]; p+=1;

    match get_gtpv2_msg_type(msg_type) {
        Ok(ret) => println!("This message is {}", ret.to_string()),
        Err(_) => return Err(format!("Unknown Message type {}", msg_type).to_string()),
    }

    let msg_len = u16::from_be_bytes([data[p], data[p+1]]) as usize; p+=2;
    if msg_len + 4 != data.len() {
        return Err(
            format!( "(GTPv2-RECV) Length error (received: {}, expected: {}). Discard.",
            data.len(), msg_len + 4)
        );
    }

    if tflag {
        teid = u32::from_be_bytes([data[p], data[p+1], data[p+2], 0]);
        p+=4;
    };

    seqnum = u32::from_be_bytes([data[p], data[p+1], data[p+2], 0]) >> 8;

    hdr.t = msg_type;
    hdr.l = msg_len as u16;
    hdr.teid = teid;
    hdr.s = seqnum;

    p+=4;

    return Ok((hdr, p));
}


pub fn
get_teid_from_header (data: &[u8])  -> u32
{
    let mut p: usize = 0;
    let mut teid: u32 = 0;

    let tflag = (data[p] & GTPV2_T_FLAG) != 0;
    p+=4;

    if tflag {
        teid = u32::from_be_bytes([data[p], data[p+1], data[p+2], 0]);
    };

    return teid;
}

