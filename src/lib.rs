#[link(name = "glpk-rs"
       , vers = "0.0"
       , author = "Sébastien Crozet, rust-bindgen"
       , uuid = "0eb7593d-915e-4c54-8107-fbeed2471670")];
#[crate_type = "lib"];

pub use ffi::*;
pub use prob::*;

pub mod ffi;
pub mod prob;
pub mod hl;
