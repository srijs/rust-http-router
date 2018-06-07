use std::borrow::Cow;
use std::sync::Arc;

use http::{Method, Uri};
use percent_encoding::{percent_decode, PercentDecode};
use regex::{Captures, RegexSet, SetMatchesIntoIter};

use error::Error;
use layer::Layer;

pub struct Router<T> {
    regex_set: RegexSet,
    layers: Arc<[Layer<T>]>,
}

impl<T> Router<T> {
    pub fn builder() -> Builder<T> {
        Builder::new()
    }

    pub fn matches(&self, method: Method, uri: Uri) -> Matches<T> {
        let set_matches = self.regex_set.matches(uri.path());
        Matches {
            method,
            uri,
            len: set_matches.len(),
            set_matches: set_matches.into_iter(),
            layers: self.layers.clone(),
        }
    }
}

pub struct Match<T> {
    uri: Uri,
    idx: usize,
    layers: Arc<[Layer<T>]>,
}

impl<T> Match<T> {
    pub fn handler(&self) -> &T {
        self.layers[self.idx].handler()
    }

    pub fn params(&self) -> Params {
        let captures = self.layers[self.idx].captures(&self.uri).unwrap();
        Params { captures }
    }
}

pub struct Param<'a> {
    decode: PercentDecode<'a>,
}

impl<'a> Param<'a> {
    pub fn into_bytes(self) -> Cow<'a, [u8]> {
        self.decode.into()
    }

    pub fn decode_utf8(self) -> Result<Cow<'a, str>, ::std::str::Utf8Error> {
        self.decode.decode_utf8()
    }

    pub fn decode_utf8_lossy(self) -> Cow<'a, str> {
        self.decode.decode_utf8_lossy()
    }
}

pub struct Params<'a> {
    captures: Captures<'a>,
}

impl<'a> Params<'a> {
    pub fn find(&'a self, key: &str) -> Option<Param<'a>> {
        self.captures.name(key).map(|capture| Param {
            decode: percent_decode(capture.as_str().as_bytes()),
        })
    }
}

pub struct Matches<T> {
    method: Method,
    uri: Uri,
    len: usize,
    set_matches: SetMatchesIntoIter,
    layers: Arc<[Layer<T>]>,
}

impl<T> Iterator for Matches<T> {
    type Item = Match<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(idx) = self.set_matches.next() {
                self.len -= 1;
                if self.layers[idx].is_match(&self.method) {
                    return Some(Match {
                        uri: self.uri.clone(),
                        idx,
                        layers: self.layers.clone(),
                    });
                }
            } else {
                return None;
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len))
    }
}

pub struct Builder<T> {
    error: Option<Error>,
    layers: Vec<Layer<T>>,
}

impl<T> Builder<T> {
    pub fn new() -> Builder<T> {
        Builder {
            error: None,
            layers: Vec::new(),
        }
    }

    fn push_layer(&mut self, method: Option<Method>, path: &str, handler: T) {
        match Layer::from_parts(method, path, handler) {
            Ok(layer) => self.layers.push(layer),
            Err(err) => self.error = Some(err),
        }
    }

    pub fn route(mut self, method: Method, path: &str, handler: T) -> Self {
        self.push_layer(Some(method), path, handler);
        self
    }

    pub fn route_any(mut self, path: &str, handler: T) -> Self {
        self.push_layer(None, path, handler);
        self
    }

    pub fn build(self) -> Result<Router<T>, Error> {
        let regex_set = RegexSet::new(self.layers.iter().map(|layer| layer.pattern()))
            .map_err(Error::from_err)?;

        Ok(Router {
            regex_set,
            layers: self.layers.into(),
        })
    }
}
