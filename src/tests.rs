use http::Method;

use super::Router;

#[test]
fn no_routes() {
    let router = Router::<()>::builder().build().unwrap();
    let matches = router
        .matches(Method::GET, "/".parse().unwrap())
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
        .matches(Method::POST, "/".parse().unwrap())
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
        .matches(Method::GET, "/foo/bar".parse().unwrap())
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
        .matches(Method::GET, "/".parse().unwrap())
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
        .matches(Method::GET, "/foo/abc".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(1, matches.len());
    assert_eq!(
        "abc",
        matches[0]
            .params()
            .find("bar")
            .unwrap()
            .decode_utf8()
            .unwrap()
    );
}

#[test]
fn param_init_match() {
    let router = Router::<()>::builder()
        .route(Method::GET, "/:foo/bar", ())
        .build()
        .unwrap();
    let matches = router
        .matches(Method::GET, "/abc/bar".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(1, matches.len());
    assert_eq!(
        "abc",
        matches[0]
            .params()
            .find("foo")
            .unwrap()
            .decode_utf8()
            .unwrap()
    );
}

#[test]
fn param_star_match() {
    let router = Router::<()>::builder()
        .route(Method::GET, "/foo/*bar", ())
        .build()
        .unwrap();
    let matches = router
        .matches(Method::GET, "/foo/abc/def".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(1, matches.len());
    assert_eq!(
        "abc/def",
        matches[0]
            .params()
            .find("bar")
            .unwrap()
            .decode_utf8()
            .unwrap()
    );
}

#[test]
fn multi_match() {
    let router = Router::builder()
        .route(Method::GET, "/foo/*bar", 1)
        .route(Method::GET, "/foo/:baz", 2)
        .route(Method::GET, "/foo", 3)
        .build()
        .unwrap();
    let matches = router
        .matches(Method::GET, "/foo/abc".parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(2, matches.len());
    assert_eq!(
        "abc",
        matches[0]
            .params()
            .find("bar")
            .unwrap()
            .decode_utf8()
            .unwrap()
    );
    assert_eq!(
        "abc",
        matches[1]
            .params()
            .find("baz")
            .unwrap()
            .decode_utf8()
            .unwrap()
    );
}
