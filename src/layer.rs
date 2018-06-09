use http::Method;
use regex::{Captures, Regex};

use errors::BuildError;
use pattern;

pub(crate) struct Layer<T> {
    method: Option<Method>,
    regex: Regex,
    handler: T,
}

impl<T> Layer<T> {
    #[inline]
    pub(crate) fn from_parts(
        method: Option<Method>,
        path: &str,
        handler: T,
    ) -> Result<Layer<T>, BuildError> {
        let regex = pattern::parse(path)?;
        Ok(Layer {
            method,
            regex,
            handler,
        })
    }

    #[inline]
    pub(crate) fn is_match(&self, method: &Method) -> bool {
        match self.method {
            Some(ref self_method) => self_method == method,
            None => true,
        }
    }

    #[inline]
    pub(crate) fn handler(&self) -> &T {
        &self.handler
    }

    #[inline]
    pub(crate) fn captures<'a>(&self, path: &'a str) -> Option<Captures<'a>> {
        self.regex.captures(path)
    }

    #[inline]
    pub(crate) fn pattern(&self) -> &str {
        self.regex.as_str()
    }
}
