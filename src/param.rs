use std::borrow::Cow;
use std::error;
use std::fmt;
use std::str::Utf8Error;

use percent_encoding::percent_decode;
use regex::Captures;

use errors::ParamError;

pub struct Params<'a> {
    captures: Captures<'a>,
}

impl<'a> Params<'a> {
    pub(crate) fn new(captures: Captures<'a>) -> Self {
        Params { captures }
    }

    pub fn get(&self, key: &str) -> Result<String, ParamError> {
        self.get_raw(key)
            .ok_or(ParamError::NotFound)
            .and_then(|bytes| {
                String::from_utf8(bytes.into_owned())
                    .map_err(|err| ParamError::Decode(err.utf8_error()))
            })
    }

    pub fn get_raw(&'a self, key: &str) -> Option<Cow<'a, [u8]>> {
        self.captures
            .name(key)
            .map(|capture| percent_decode(capture.as_str().as_bytes()).into())
    }

    pub fn wildcard(&'a self) -> Option<&'a str> {
        self.captures.get(1).map(|capture| capture.as_str())
    }
}
