#[cfg(feature = "make-parsers")]
mod core {
    pub fn make_parsers() {
        use std::{fs, path::Path};
    
        fs::write(
            Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/handlers.rs")),
            include_str!(concat!(env!("OUT_DIR"), "/handlers.rs")),
        )
        .unwrap();
    }
}
#[cfg(feature = "make-parsers")]
pub use core::*;


#[cfg(any(feature = "dynamic-server", feature = "static-render"))]
mod config;

#[cfg(any(feature = "dynamic-server", feature = "static-render"))]
mod parsers;

#[cfg(any(feature = "dynamic-server", feature = "static-render"))]
mod extras {
    use std::collections::HashMap;
    use serde::{Serialize, Deserialize};

    pub use crate::config::*;
    pub use crate::parsers::*;

    pub type Handlers = HashMap<
        &'static str,
        fn(Attributes, HashMap<String, String>) -> Result<String, String>
    >;

    #[derive(Serialize, Deserialize)]
    struct Context<T>
    {
        #[serde(rename = "q")]
        query: HashMap<String, String>,
        #[serde(rename = "a")]
        attributes: Attributes,
        #[serde(rename = "_")]
        data: Option<T>,
    }
    
    #[cfg(not(feature = "make-parsers"))]
    pub fn add_handlers_from_src_dir(handlers: &mut Handlers) {
        //...
        include!("./handlers.rs");
        //...
    }
    
    #[cfg(feature = "make-parsers")]
    pub fn add_handlers_from_out_dir(handlers: &mut Handlers) {
        //...
        include!(concat!(env!("OUT_DIR"), "/handlers.rs"));
        //...
    }
}

#[cfg(any(feature = "dynamic-server", feature = "static-render"))]
pub use extras::*;
