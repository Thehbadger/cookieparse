//! This crate lets you parse a Header Cookie list. These often look like `cookie1=value1; cookie2=value2`. This is not to be confused with a Set-Cookie, which has a similar, but different format. See [`setcookie`](crate::setcookie) for a parser for those fields.
//!  Despite what you may believe cookies are just octets not characters <https://httpwg.org/specs/rfc6265.html#cookie> .
use std::collections::HashMap;
use winnow::{
    combinator::{alt, separated, separated_pair},
    prelude::*,
    token::{rest, take_until},
};

use crate::error::CookieParseError;

/// Cookies are technically just octets.
/// ## Example
/// ```
/// # use cookieparse::cookie::parse_cookies_from_bytes;
/// let bytes = b"name1=value1; name2=value2";
/// let map = parse_cookies_from_bytes(bytes).unwrap();
/// assert_eq!(*map.get("name1".as_bytes()).unwrap(), b"value1");
/// assert_eq!(*map.get("name2".as_bytes()).unwrap(), b"value2");
/// ```
#[allow(dead_code)]
pub fn parse_cookies_from_bytes(input: &[u8]) -> Result<HashMap<&[u8], &[u8]>, CookieParseError> {
    parse_bytes
        .parse(input)
        .map_err(|e| CookieParseError(e.to_string()))
}

/// Cookies can be octet values, but your data might all be utf8, so it might be easier just to use str.
/// ## Example
/// ```
/// # use cookieparse::cookie::parse_cookies_from_str;
/// let string = "name1=value1; name2=value2";
/// let map = parse_cookies_from_str(string).unwrap();
/// assert_eq!(*map.get("name1").unwrap(), "value1");
/// assert_eq!(*map.get("name2").unwrap(), "value2");
/// ```
#[allow(dead_code)]
pub fn parse_cookies_from_str(input: &str) -> Result<HashMap<&str, &str>, CookieParseError> {
    parse_str
        .parse(input)
        .map_err(|e| CookieParseError(e.to_string()))
}

fn parse_bytes<'i>(input: &mut &'i [u8]) -> winnow::Result<HashMap<&'i [u8], &'i [u8]>> {
    let key_val_parser =
        separated_pair(take_until(1.., '='), '=', alt((take_until(1.., ';'), rest)));
    separated(0.., key_val_parser, "; ").parse_next(input)
}

fn parse_str<'i>(input: &mut &'i str) -> winnow::Result<HashMap<&'i str, &'i str>> {
    let key_val_parser =
        separated_pair(take_until(1.., '='), '=', alt((take_until(1.., ';'), rest)));
    separated(0.., key_val_parser, "; ").parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{prelude::*, string::bytes_regex};

    #[test]
    fn test_cookie_bytes() {
        let samples = "potato=watermelon; name2=value2; _name3=value3";
        let sample_map = parse_cookies_from_bytes(samples.as_bytes()).unwrap();
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

    #[test]
    fn test_cookie_str() {
        let samples = "potato=watermelon; name2=value2; _name3=value3";
        let sample_map = parse_cookies_from_str(samples).unwrap();
        assert_eq!(*sample_map.get("potato").unwrap(), "watermelon");
        assert_eq!(*sample_map.get("name2").unwrap(), "value2");
        assert_eq!(*sample_map.get("_name3").unwrap(), "value3");
    }

    proptest! {
        #[test]
        fn test_possible_inputs_bytes(s in bytes_regex("(?s-u:([^=;]+=[^=;]+; )*[^=;]+=[^=;]+)").unwrap()) {
            parse_cookies_from_bytes(&s).unwrap();
        }

        #[test]
        fn test_possible_inputs_str(s in "([^=;]+=[^=;]+; )*[^=;]+=[^=;]+") {
            parse_cookies_from_str(&s).unwrap();
        }
    }
}
