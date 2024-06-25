use std::{collections::HashMap, sync::Arc};
use axum::{routing::get, Router, extract::State, response::Response};
use tokio::net::TcpListener;
use grola::parsers;


type Handlers = HashMap<&'static str, fn() -> Result<String, String>>;

async fn render(State(state): State<Arc<Handlers>>) -> Response<String>
{
    Response::builder()
        .body((Arc::clone(&state).get("blog-posts.html").unwrap())()
            .unwrap_or_else(|err| err))
        .unwrap()
}

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

    let _ = dynamic_server(handlers);
}

#[tokio::main]
async fn dynamic_server(handlers: Handlers) {
    let state = Arc::new(handlers);
    let server = Router::new()
        // `GET /` goes to `root`
        .route("/", get(render))
        .with_state(state);
        // `POST /users` goes to `create_user`
        //.route("/users", post(create_user));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, server).await.unwrap();

    //let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();

    //for mut stream in listener.incoming().flatten() {
    //    let mut rdr = std::io::BufReader::new(&mut stream);
    //    let mut l = String::new();
    //    rdr.read_line(&mut l).unwrap();
    //    match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
    //        ["GET", resource, "HTTP/1.1"] => {
    //            loop {
    //                let mut l = String::new();
    //                rdr.read_line(&mut l).unwrap();
    //                if l.trim().is_empty() { break; }
    //            }
    //            let mut p = std::path::PathBuf::new();
    //            p.push("htdocs");
    //            p.push(resource.trim_start_matches("/"));
    //            if resource.ends_with('/') { p.push("index.html"); }
    //            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
    //            //stream.write_all(&std::fs::read(p).unwrap()).unwrap();
    //            stream.write_all(
    //                (handlers.get("blog-posts.html").unwrap())()
    //                    .unwrap_or_else(|err| err).as_bytes()
    //            ).unwrap();
    //        }
    //        _ => todo!()
    //    }
    //}
}

#[cfg(not(feature = "make-parsers"))]
fn add_handlers_from_src_dir(
    handlers: &mut Handlers
)
{
    //...
    include!("handlers.rs");
    //...
}

#[cfg(feature = "make-parsers")]
fn add_handlers_from_out_dir(
    handlers: &mut Handlers
)
{
    //...
    include!(concat!(env!("OUT_DIR"), "/handlers.rs"));
    //...
}
