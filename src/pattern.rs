use regex::{escape, Regex};

use error::Error;

pub fn parse(mut route: &str) -> Result<Regex, Error> {
    let mut pattern = "^/?".to_string();

    if route.len() != 0 && route.as_bytes()[0] == b'/' {
        route = &route[1..];
    }

    for (i, segment) in route.split('/').enumerate() {
        if i > 0 {
            pattern.push('/')
        }

        if segment.len() > 0 && segment.as_bytes()[0] == b':' {
            push_dynamic_segment(&segment[1..], &mut pattern);
        } else if segment.len() > 0 && segment.as_bytes()[0] == b'*' {
            push_star_segment(&segment[1..], &mut pattern);
        } else {
            push_static_segment(segment, &mut pattern);
        }
    }

    pattern.push('$');

    Regex::new(&pattern).map_err(Error::from_err)
}

#[inline]
fn push_dynamic_segment(name: &str, pattern: &mut String) {
    pattern.push_str(&format!("(?P<{}>[^/]+)", escape(name)));
}

#[inline]
fn push_star_segment(name: &str, pattern: &mut String) {
    pattern.push_str(&format!("(?P<{}>.*)", escape(name)));
}

#[inline]
fn push_static_segment(name: &str, pattern: &mut String) {
    pattern.push_str(&escape(name));
}
