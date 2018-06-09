use http::Method;

use super::{BuildError, Router};

#[test]
fn no_routes() {
    let router = Router::<()>::builder().build().unwrap();
    let matches = router
        .matches(Method::GET, &"/".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(0, matches.len());
}

#[test]
fn method_mismatch() {
    let router = Router::<()>::builder()
        .route(Method::GET, "/", ())
        .build()
        .unwrap();
    let matches = router
        .matches(Method::POST, &"/".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(0, matches.len());
}

#[test]
fn path_mismatch() {
    let router = Router::<()>::builder()
        .route(Method::GET, "/foo", ())
        .build()
        .unwrap();
    let matches = router
        .matches(Method::GET, &"/foo/bar".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(0, matches.len());
}

#[test]
fn method_match() {
    let router = Router::<()>::builder()
        .route(Method::GET, "/", ())
        .build()
        .unwrap();
    let matches = router
        .matches(Method::GET, &"/".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(1, matches.len());
}

#[test]
fn param_tail_match() {
    let router = Router::<()>::builder()
        .route(Method::GET, "/foo/:bar", ())
        .build()
        .unwrap();
    let matches = router
        .matches(Method::GET, &"/foo/abc".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(1, matches.len());
    assert_eq!("abc", matches[0].params().get("bar").unwrap());
}

#[test]
fn param_init_match() {
    let router = Router::<()>::builder()
        .route(Method::GET, "/:foo/bar", ())
        .build()
        .unwrap();
    let matches = router
        .matches(Method::GET, &"/abc/bar".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(1, matches.len());
    assert_eq!("abc", matches[0].params().get("foo").unwrap());
}

#[test]
fn param_percent_decode() {
    let router = Router::<()>::builder()
        .route(Method::GET, "/greet/:greeting", ())
        .build()
        .unwrap();
    let matches = router
        .matches(Method::GET, &"/greet/Hello%20World".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(1, matches.len());
    assert_eq!("Hello World", matches[0].params().get("greeting").unwrap());
}

#[test]
fn param_wildcard_match() {
    let router = Router::<()>::builder()
        .route(Method::GET, "/foo/*", ())
        .build()
        .unwrap();
    let matches = router
        .matches(Method::GET, &"/foo/abc/def".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(1, matches.len());
    assert_eq!("abc/def", matches[0].params().wildcard().unwrap());
}

#[test]
fn param_empty_wildcard_match() {
    let router = Router::<()>::builder()
        .route(Method::GET, "/*", ())
        .build()
        .unwrap();
    let matches = router
        .matches(Method::GET, &"/".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(1, matches.len());
    assert_eq!("", matches[0].params().wildcard().unwrap());
}

#[test]
fn multi_match() {
    let router = Router::builder()
        .route(Method::GET, "/foo/*", 1)
        .route(Method::GET, "/foo/:bar", 2)
        .route(Method::GET, "/foo", 3)
        .build()
        .unwrap();
    let matches = router
        .matches(Method::GET, &"/foo/abc".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(2, matches.len());
    assert_eq!("abc", matches[0].params().wildcard().unwrap());
    assert_eq!("abc", matches[1].params().get("bar").unwrap());
}

#[test]
fn build_error_invalid_param_name() {
    let result = Router::builder()
        .route(Method::GET, "/foo/:123/bar", ())
        .build();

    let err = result.err().unwrap();

    if let BuildError::InvalidParamName = err {
    } else {
        panic!("wrong error: {}", err)
    }
}
