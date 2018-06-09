//! A general purpose library for HTTP request routing.
//!
//! This crate provides basic building blocks to implement
//! URI and method-based routing and middleware functionality
//! in web applications as well as frameworks.
//!
//! The library is general purpose in the sense that it is un-opinionated
//! with regards to details such as the order of route evaluations, or any
//! fall-through behaviour. This allows consumers of the library to make
//! these choices according to their preferences.
//!
//! ## Router
//!
//! The main component is the `Router` type, which you can think of as a
//! general-purpose mapping between URIs and methods on one side, and
//! generic handlers on the other.
//!
//! Being generic with regards to handlers means that it works with both
//! blocking as well as futures-based route handlers, as well as any other
//! custom handler type.
//!
//! In additional to that, the router is also un-opinionated when it comes to
//! details such as the order of route evaluations, or fall-through behaviour.
//! This allows consumers of the library to make these choices according to their
//! preferences.
//!
//! To create a `Router`, you can use the builder pattern to assign route handlers
//! to paths:
//!
//! ```rust
//! # extern crate http;
//! # extern crate http_router;
//! use http::Method;
//! use http_router::Router;
//! # let get_post_handler = ();
//! # let create_post_handler = ();
//!
//! let router = Router::builder()
//!     .route(Method::GET, "/posts/:id", get_post_handler)
//!     .route(Method::POST, "/posts", create_post_handler)
//!     .build().unwrap();
//! ```
//!
//! Once built, you can use the `Router::matches` method to look up the set of assigned
//! handlers for a given `Uri` and `Method`.
//!
//! The method will return an iterator that contains all matching handlers in the order
//! they were registered in:
//!
//! ```rust
//! # extern crate http;
//! # extern crate http_router;
//! # use http::Method;
//! # use http_router::Router;
//! # let router = Router::<()>::builder().build().unwrap();
//! let uri = "/posts/123".parse().unwrap();
//!
//! for router_match in router.matches(Method::GET, &uri) {
//!     let handler = router_match.handler();
//!     let params = router_match.params();
//!
//!     assert_eq!("123", params.get("id").unwrap());
//! }
//! ```
//!
//! ## Patterns
//!
//! The router supports two kinds of patterns, which can be used to extract path parameters
//! from the `Uri`.
//!
//! **Segment patterns** will match on a single path segment and turn it into a parameter:
//!
//! ```rust
//! # extern crate http;
//! # extern crate http_router;
//! # use http::Method;
//! # use http_router::Router;
//! # let get_post_comments_handler = ();
//! let router = Router::builder()
//!     .route(Method::GET, "/posts/:id/comments", get_post_comments_handler)
//!     .build().unwrap();
//!
//! let uri = "/posts/123/comments".parse().unwrap();
//!
//! for router_match in router.matches(Method::GET, &uri) {
//!     let handler = router_match.handler();
//!     let params = router_match.params();
//!
//!     assert_eq!("123", params.get("id").unwrap());
//! }
//! ```
//!
//! **Wildcard patterns** will consume the remainder of the URI, which can span multiple segments:
//!
//! ```rust
//! # extern crate http;
//! # extern crate http_router;
//! # use http::Method;
//! # use http_router::Router;
//! # let files_handler = ();
//! let router = Router::builder()
//!     .route_any("/files/*", files_handler)
//!     .build().unwrap();
//!
//! let uri = "/files/documents/essays/essay01.txt".parse().unwrap();
//!
//! for router_match in router.matches(Method::GET, &uri) {
//!     let handler = router_match.handler();
//!     let params = router_match.params();
//!
//!     assert_eq!("documents/essays/essay01.txt", params.wildcard().unwrap());
//! }
//! ```

extern crate http;
extern crate percent_encoding;
extern crate regex;

mod errors;
mod layer;
mod param;
mod pattern;
mod router;
#[cfg(test)]
mod tests;

pub use errors::{BuildError, ParamError};
pub use param::Params;
pub use router::{Builder, Match, Matches, Router};
