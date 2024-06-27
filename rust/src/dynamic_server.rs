use axum::{extract::Query, response::Response, routing::get, Router};
use grola::{get_config, parsers, Config};
use std::{collections::HashMap, env, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;


type Handlers = HashMap<&'static str, fn() -> Result<String, String>>;

fn main() {
    let mut handlers = std::collections::HashMap::new();

    #[cfg(feature = "make-parsers")]
    {
        add_handlers_from_out_dir(&mut handlers);
        grola::make_parsers();
    }
    #[cfg(not(feature = "make-parsers"))]
    {
        add_handlers_from_src_dir(&mut handlers);
    }

    let config_file = env::args().nth(1).unwrap();
    let config = get_config(&PathBuf::from(config_file)).unwrap();
    let _ = dynamic_server(handlers, config);
}

#[tokio::main]
async fn dynamic_server(mut handlers: Handlers, config: Config) {
    let mut server = Router::new();

    for (route, options) in config.routes.into_iter() {
        let handler = Arc::new(
            handlers
                .remove(&options.0 as &str)
                .unwrap_or(|| Err("404".to_owned())),
        );
        server = server.route(
            &route,
            get(move |Query(_params): Query<HashMap<String, String>>| {
                let body = handler().unwrap_or_else(|err| format!("Error:\n\n{}", err));
                let response = Response::builder().body(body).unwrap();
                async move { response }
            }),
        );
    }

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, server).await.unwrap();
}

#[cfg(not(feature = "make-parsers"))]
fn add_handlers_from_src_dir(handlers: &mut Handlers) {
    //...
    include!("handlers.rs");
    //...
}

#[cfg(feature = "make-parsers")]
fn add_handlers_from_out_dir(handlers: &mut Handlers) {
    //...
    include!(concat!(env!("OUT_DIR"), "/handlers.rs"));
    //...
}
