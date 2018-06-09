use regex::{escape, Regex};

use errors::BuildError;

pub fn parse(mut route: &str) -> Result<Regex, BuildError> {
    let mut pattern = "^/?".to_string();

    if route.len() != 0 && route.as_bytes()[0] == b'/' {
        route = &route[1..];
    }

    for (i, segment) in route.split('/').enumerate() {
        if i > 0 {
            pattern.push('/')
        }

        if segment.len() > 0 && segment.as_bytes()[0] == b':' {
            if !is_valid_param_name(&segment[1..]) {
                return Err(BuildError::InvalidParamName);
            }
            push_dynamic_segment(&segment[1..], &mut pattern);
        } else if segment.len() == 1 && segment.as_bytes()[0] == b'*' {
            push_wildcard(&mut pattern);
            break;
        } else {
            push_static_segment(segment, &mut pattern);
        }
    }

    pattern.push('$');

    Ok(Regex::new(&pattern).unwrap())
}

#[inline]
fn is_valid_param_name(name: &str) -> bool {
    let mut chars = name.chars();

    if let Some(first_char) = chars.next() {
        if first_char.is_ascii_alphabetic() {
            return chars.all(|c| c.is_ascii_alphanumeric());
        }
    }

    return false;
}

#[inline]
fn push_dynamic_segment(name: &str, pattern: &mut String) {
    pattern.push_str(&format!("(?P<{}>[^/]+)", escape(name)));
}

#[inline]
fn push_wildcard(pattern: &mut String) {
    pattern.push_str(&format!("(.*)"));
}

#[inline]
fn push_static_segment(name: &str, pattern: &mut String) {
    pattern.push_str(&escape(name));
}
