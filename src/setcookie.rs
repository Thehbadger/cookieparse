use std::{collections::HashMap, path::PathBuf};

use winnow::{
    combinator::{alt, separated_pair},
    token::{rest, take_until},
};

use crate::error::CookieParseError;

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

/// A <cookie-value> can optionally be wrapped in double quotes and include any US-ASCII character
/// excluding control characters (ASCII characters 0 up to 31 and ASCII character 127), Whitespace,
/// double quotes, commas, semicolons, and backslashes.
#[derive(Debug, PartialEq, Eq)]
pub struct CookieValue(String);

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

fn parse_str<'i>(input: &mut &'i str) -> winnow::Result<HashMap<&'i str, &'i str>> {
    let kv_pair = separated_pair(take_until(1.., "="), "=", alt((take_until(1.., ";"), rest)));
    todo!();
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
