use crate::gtpv2_type;
use std::net::Ipv4Addr;


pub enum CustomErr<I> {
    NomError((I, ErrorKind)),
    NonUtf8String,
    UnrecognizedMessageType,
    InvalidHlen,
}

pub enum ErrorKind {
    Tag,
    MapRes,
    ManyTill,
    Eof,
    Custom(u32),
}



type IResult<I, O> = Result<(I, O), CustomErr<I>>;

fn custom_be_u8(input: &[u8]) -> IResult<&[u8], u8> {
    if input.is_empty() {
        return Err(CustomErr::InvalidHlen);
    }

    Ok((&input[1..], input[0]))
}

fn custom_be_u16(input: &[u8]) -> IResult<&[u8], u16> {
    if input.len() < 2 {
        return Err(CustomErr::InvalidHlen);
    }

    let value = u16::from_be_bytes([input[0], input[1]]);
    Ok((&input[2..], value))
}
fn custom_be_u32(input: &[u8]) -> IResult<&[u8], u32> {
    if input.len() < 4 {
        return Err(CustomErr::InvalidHlen);
    }

    let value = u32::from_be_bytes([input[0], input[1], input[2], input[3]]);
    Ok((&input[4..], value))
}

fn custom_take<'a>(n: usize) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input: &'a [u8]| {
        if input.len() >= n {
            Ok((&input[n..], &input[0..n]))
        } else {
            Err(CustomErr::InvalidHlen)
        }
    }
}



fn decode_ipv4(p: &[u8]) -> IResult<&[u8], Ipv4Addr> {
    let (input, addr) = custom_take(4usize)(p)?;
    Ok((input, Ipv4Addr::new(addr[0], addr[1], addr[2], addr[3])))
}
