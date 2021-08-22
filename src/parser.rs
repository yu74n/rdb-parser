use nom::IResult;
use nom::bits;
use nom::bytes;
use nom::bytes::complete::tag;
use nom::character::is_digit;
use nom::bytes::complete::take_while;

use std::str;

#[derive(Debug,PartialEq)]
struct Header {
    pub version: String,
}

fn header(input: &[u8]) -> IResult<&[u8], Header> {
    let (input, _) = tag("REDIS")(input)?;
    let (input, version) = take_while(is_digit)(input)?;

    Ok((input, Header {
        version: str::from_utf8(version).unwrap().to_string()
    }))
}

fn aux(input: &[u8]) -> IResult<&[u8], (String, String)> {
    let (input, key) = string_encoding(input).unwrap();
    let (input, value) = string_encoding(input).unwrap();
    
    Ok((input, (key, value)))
}

fn take_bits(input: (&[u8], usize), count: usize) -> IResult<(&[u8], usize), u8> {
    bits::complete::take(count)(input)
}

fn take_bytes(input: &[u8], count: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(count)(input)
}

fn length_encoding(input: &[u8]) -> IResult<&[u8], usize> {
    let ((input, _), bits) = take_bits((input, 0), 2).unwrap();
    let ((input, _), rest) = take_bits((input, 2), 6).unwrap();
    let length = match bits {
        0 => {
            rest as usize
        },
        _ => {
            0
        }
    };
    Ok((input, length))
}

fn string_encoding(input: &[u8]) -> IResult<&[u8], String> {
    let (input, length) = length_encoding(input).unwrap();
    let (input, raw_string) = take_bytes(input, length).unwrap();
    Ok((input, str::from_utf8(raw_string).unwrap().to_string()))
}

pub fn parse(input: &[u8]) {
    header(input);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_test() {
        let bytes = include_bytes!("../assets/dump.rdb");
        assert_eq!(
            header(&bytes[..9]),
            Ok((
                &b""[..],
                Header { version: String::from("0009") }
            )));
    }

    #[test]
    fn length_encoding_test() {
        let bytes = include_bytes!("../assets/dump.rdb");
        let (_, size) = length_encoding(&bytes[10..28]).unwrap();
        assert_eq!(size, 9);
    }

    #[test]
    fn string_encoding_test() {
        let bytes = include_bytes!("../assets/dump.rdb");
        let (_, str) = string_encoding(&bytes[10..28]).unwrap();
        assert_eq!(str, "redis-ver");
    }

    #[test]
    fn aux_test() {
        let bytes = include_bytes!("../assets/dump.rdb");
        let (_, (key, value)) = aux(&bytes[10..28]).unwrap();
        assert_eq!(key, "redis-ver");
        assert_eq!(value, "6.0.10");
    }
}