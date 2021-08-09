use nom::IResult;
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
}