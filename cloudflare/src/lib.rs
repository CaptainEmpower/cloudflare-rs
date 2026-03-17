#![forbid(unsafe_code)]
// Allow module_inception for legitimate module organization patterns
// This is the standard Rust pattern for module structure and test organization
#![allow(clippy::module_inception)]

pub mod endpoints;
pub mod framework;
