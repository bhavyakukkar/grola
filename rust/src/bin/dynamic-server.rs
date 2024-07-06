use axum::{extract::Query, response::Response, routing::get, Router};
use tower_http::services::ServeDir;
use grola::{get_config, Config, Handlers};
use std::{collections::HashMap, env, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;


fn main() {
    let mut handlers = std::collections::HashMap::new();

    #[cfg(feature = "static-render")]
    {
        eprintln!(
            "This binary target does not accept the `static-render` feature. \
            Use the `static-render` binary target instead."
        );
        exit(1);
    }

    #[cfg(feature = "make-parsers")]
    {
        grola::add_handlers_from_out_dir(&mut handlers);
        grola::make_parsers();
    }
    #[cfg(not(feature = "make-parsers"))]
    {
        grola::add_handlers_from_src_dir(&mut handlers);
    }

    let config_file = env::args().nth(1).unwrap();
    let config = get_config(&PathBuf::from(config_file)).unwrap();
    let _ = dynamic_server(handlers, config);
}

#[tokio::main]
async fn dynamic_server(mut handlers: Handlers, config: Config) {
    let mut server = Router::new();
    if let Some(server_config) = config.server {
        if let Some(static_route) = server_config.static_route {
            server = server.nest_service(&static_route.0, ServeDir::new(static_route.1));
        }
    }

    for (route, options) in config.routes.into_iter() {
        let handler_maybe = Arc::new(
            handlers
                .remove(/*template*/&options.0 as &str)
                .unwrap_or(|_, _| Err("404".to_owned())),
        );
        server = server.route(
            &route,
            get(move |Query(params): Query<HashMap<String, String>>| {
                let body = handler_maybe(options.1, params)
                    .unwrap_or_else(|err| format!("Error:\n\n{}", err));
                let response = Response::builder().body(body).unwrap();
                async move { response }
            }),
        );
    }

    //https://docs.rs/axum/latest/axum/struct.Router.html#a-note-about-performance
    server = server.with_state(());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, server).await.unwrap();
}
