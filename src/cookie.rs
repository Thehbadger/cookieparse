use std::{collections::HashMap, ops::Deref};
use winnow::{
    combinator::{alt, separated, separated_pair},
    prelude::*,
    token::{rest, take_until},
};

use crate::error::CookieParseError;

/// This represents a list of cookies. Despite what you may believe cookies are just octets not characters https://httpwg.org/specs/rfc6265.html#cookie .
#[allow(dead_code)]
#[derive(Debug)]
pub struct CookieMap<'a>(pub HashMap<&'a [u8], &'a [u8]>);

impl<'a> Deref for CookieMap<'a> {
    type Target = HashMap<&'a [u8], &'a [u8]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
impl CookieMap<'_> {
    fn parse_from_bytes(input: &[u8]) -> Result<CookieMap, CookieParseError> {
        let map = parse
            .parse(input)
            .map_err(|e| CookieParseError(e.to_string()))?;
        Ok(CookieMap(map))
    }
}

fn parse<'i>(input: &mut &'i [u8]) -> winnow::Result<HashMap<&'i [u8], &'i [u8]>> {
    let key_val_parser =
        separated_pair(take_until(1.., '='), '=', alt((take_until(1.., ';'), rest)));
    separated(0.., key_val_parser, "; ").parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{prelude::*, string::bytes_regex};

    #[test]
    fn test_cookie() {
        let samples = "potato=watermelon; name2=value2; _name3=value3";
        // let samples = "1111=2222; 3333=4444";
        let sample_map = CookieMap::parse_from_bytes(samples.as_bytes()).unwrap();
        dbg!(&sample_map);
        assert_eq!(
            *sample_map.get("potato".as_bytes()).unwrap(),
            "watermelon".as_bytes()
        );
        assert_eq!(
            *sample_map.get("name2".as_bytes()).unwrap(),
            "value2".as_bytes()
        );
        assert_eq!(
            *sample_map.get("_name3".as_bytes()).unwrap(),
            "value3".as_bytes()
        );
    }

    // prop_compose! {
    //     fn key_or_value()(value in any()::<Vec<u8>>() ) -> Vec<u8> {
    //         value
    //     }
    // }

    // prop_compose! {
    //     // Generates a value like `xxxx=yyyy`, where x and y are just u8.
    //     fn key_value()(mut key in any::<Vec<u8>>(), mut val in any::<Vec<u8>>()) -> Vec<u8> {
    //         key.push(b'=');
    //         &mut key.append(&mut val);
    //         key
    //     }
    // }

    proptest! {
        #[test]
        fn test_all_possible_inputs(s in bytes_regex("(?s-u:([^=;]+=[^=;]+; )*[^=;]+=[^=;]+)").unwrap()) {
            CookieMap::parse_from_bytes(&s).unwrap();
        }
    }
}
