use std::sync::Arc;

use http::{uri::PathAndQuery, Method, Uri};
use regex::{RegexSet, SetMatchesIntoIter};

use errors::BuildError;
use layer::Layer;
use param::Params;

pub struct Router<T> {
    regex_set: RegexSet,
    layers: Arc<[Layer<T>]>,
}

impl<T> Router<T> {
    pub fn builder() -> Builder<T> {
        Builder::new()
    }

    pub fn matches(&self, method: Method, uri: &Uri) -> Matches<T> {
        let set_matches = self.regex_set.matches(uri.path());
        Matches {
            method,
            paq: uri.path_and_query().cloned(),
            len: set_matches.len(),
            set_matches: set_matches.into_iter(),
            layers: self.layers.clone(),
        }
    }
}

pub struct Match<T> {
    paq: Option<PathAndQuery>,
    idx: usize,
    layers: Arc<[Layer<T>]>,
}

impl<T> Match<T> {
    pub fn handler(&self) -> &T {
        self.layers[self.idx].handler()
    }

    pub fn params(&self) -> Params {
        let path = self.paq.as_ref().map(|paq| paq.path()).unwrap_or("");
        let captures = self.layers[self.idx].captures(path).unwrap();
        Params::new(captures)
    }
}

pub struct Matches<T> {
    method: Method,
    paq: Option<PathAndQuery>,
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
                        paq: self.paq.clone(),
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
    error: Option<BuildError>,
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

    pub fn build(self) -> Result<Router<T>, BuildError> {
        if let Some(err) = self.error {
            return Err(err);
        }

        let regex_set = RegexSet::new(self.layers.iter().map(|layer| layer.pattern())).unwrap();

        Ok(Router {
            regex_set,
            layers: self.layers.into(),
        })
    }
}
