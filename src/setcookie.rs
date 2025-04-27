use std::{collections::HashMap, path::PathBuf, str::FromStr};

use winnow::{
    combinator::{Alt, alt, delimited, opt, preceded, separated_pair},
    prelude::*,
    stream::AsChar,
    token::{none_of, rest, take_until, take_while},
};

use crate::{cookie, error::CookieParseError};

/// Represents a SetCookie header. Reference here: <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Set-Cookie>
#[derive(Debug, PartialEq, Eq)]
pub struct Cookie {
    name: CookieName,
    value: CookieValue,
    /// Defines the host to which the cookie will be sent.
    domain: Option<String>,
    ///Indicates the maximum lifetime of the cookie as an HTTP-date timestamp. See Date for the required formatting.
    ///
    ///If unspecified, the cookie becomes a session cookie. A session finishes when the client shuts down, after which the session cookie is removed.
    expires: Option<CookieDate>,
    /// Forbids JavaScript from accessing the cookie.
    http_only: bool,
    /// Indicates the number of seconds until the cookie expires.
    max_age: Option<i32>,
    /// Indicates that the cookie should be stored using partitioned storage.
    partitioned: bool,
    /// Indicates the path that must exist in the requested URL for the browser to send the Cookie header.
    path: Option<PathBuf>,
    /// Controls whether or not a cookie is sent with cross-site requests.
    same_site: Option<SameSiteValue>,
    /// Indicates that the cookie is sent to the server only when a request is made with the https: scheme.
    secure: bool,
}

/// A `CookieName` can contain any US-ASCII characters except for: control characters
/// (ASCII characters 0 up to 31 and ASCII character 127) or separator characters
/// (space, tab and the characters: ( ) < > @ , ; : \ " / [ ] ? = { })
#[derive(Debug, PartialEq, Eq)]
pub struct CookieName(String);

impl FromStr for CookieName {
    type Err = CookieParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn parse_cookie_name(input: &mut &str) -> winnow::Result<CookieName> {
    take_while(
        1..,
        (
            0x21,
            0x23..=0x27,
            0x2A,
            0x2B,
            0x2D,
            0x2E,
            0x30..=0x39,
            0x41..=0x5A,
            0x5E..=0x7E,
        ),
    )
    .map(|x: &str| CookieName(x.to_string()))
    .parse_next(input)
}

/// A <cookie-value> can optionally be wrapped in double quotes and include any US-ASCII character
/// excluding control characters (ASCII characters 0 up to 31 and ASCII character 127), Whitespace,
/// double quotes, commas, semicolons, and backslashes.
#[derive(Debug, PartialEq, Eq)]
pub struct CookieValue(String);

fn parse_cookie_value(input: &mut &str) -> winnow::Result<CookieValue> {
    alt((delimited('"', value_parser, '"'), value_parser))
        .map(|x| CookieValue(x.to_string()))
        .parse_next(input)
}

fn value_parser<'a>(input: &mut &'a str) -> winnow::Result<&'a str> {
    take_while(
        1..,
        (
            0x21,
            0x23..=0x2B,
            0x2D,
            0x2E,
            0x2F,
            0x30..=0x3A,
            0x3C..=0x5B,
            0x5D..=0x7E,
        ),
    )
    .parse_next(input)
}

// TODO: Add feature for support for chrono or jiff.
/// HTTP headers are of the format `Date: <day-name>, <day> <month> <year> <hour>:<minute>:<second> GMT`
#[derive(Debug, PartialEq, Eq)]
pub struct CookieDate {
    date_name: String,
    day: usize,
    month: String,
    year: usize,
    minute: usize,
    second: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SameSiteValue {
    Strict,
    Lax,
    None,
}

pub fn parse_setcookies_from_str(input: &str) -> Result<Cookie, CookieParseError> {
    todo!();
}

fn parse_str<'i>(input: &mut &'i str) -> winnow::Result<Cookie> {
    let (name, value) =
        separated_pair(parse_cookie_name, "=", parse_cookie_value).parse_next(input)?;
    if let Some(_) = opt(";").parse_next(input)? {}

    // TODO: Add builder.
    let mut cookie = Cookie {
        name,
        value,
        domain: todo!(),
        expires: todo!(),
        http_only: todo!(),
        max_age: todo!(),
        partitioned: todo!(),
        path: todo!(),
        same_site: todo!(),
        secure: todo!(),
    };
    Ok(cookie)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setcookie_bytes() {
        let sample = "cookie-name=cookie-value; Domain=domain-value; Secure; HttpOnly";
        let cookie = parse_setcookies_from_str(sample).unwrap();
        let expected_cookie = Cookie {
            name: CookieName("cookie-name".to_string()),
            value: CookieValue("cookie-value".to_string()),
            domain: Some("domain-value".to_string()),
            expires: None,
            http_only: true,
            max_age: None,
            partitioned: false,
            path: None,
            same_site: None,
            secure: true,
        };
        assert_eq!(cookie, expected_cookie);
    }
}
