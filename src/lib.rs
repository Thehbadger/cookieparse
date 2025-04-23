mod error {
    use std::{error::Error, fmt::Display};

    #[derive(Debug)]
    pub struct CookieParseError(pub String);

    impl Display for CookieParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error Parsing Cookie Header, {:?}", self.0)
        }
    }
    impl Error for CookieParseError {}
}

mod cookie {
    use std::collections::HashMap;
    use winnow::{
        combinator::{separated, separated_pair},
        prelude::*,
        token::{rest, take_until},
    };

    use crate::error::CookieParseError;

    /// This represents a list of cookies. Despite what you may believe cookies are just octets not characters https://httpwg.org/specs/rfc6265.html#cookie .
    #[allow(dead_code)]
    pub struct CookieMap<'a>(pub HashMap<&'a [u8], &'a [u8]>);

    #[allow(dead_code)]
    impl<'a> CookieMap<'a> {
        fn parse_from_bytes(input: &[u8]) -> Result<CookieMap, CookieParseError> {
            let map = parse
                .parse(input)
                .map_err(|e| CookieParseError(e.to_string()))?;
            Ok(CookieMap(map))
        }
    }

    fn parse<'i>(input: &mut &'i [u8]) -> winnow::Result<HashMap<&'i [u8], &'i [u8]>> {
        let key_val_parser = separated_pair(take_until(1.., "="), "=", rest);
        separated(0.., key_val_parser, ";").parse_next(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_cookie() {
            let samples = "potato=watermelon; name2=value2; _name3=value3";
            let sample_map = CookieMap::parse_from_bytes(samples.as_bytes()).unwrap();
        }
    }
}
