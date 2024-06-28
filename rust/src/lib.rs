#[cfg(feature = "dynamic-server")]
mod config;
#[cfg(feature = "dynamic-server")]
pub use config::*;

#[cfg(feature = "dynamic-server")]
pub mod parsers;

#[cfg(feature = "make-parsers")]
pub fn make_parsers() {
    use std::{fs, path::Path};

    fs::write(
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/handlers.rs")),
        include_str!(concat!(env!("OUT_DIR"), "/handlers.rs")),
    )
    .unwrap();
}
