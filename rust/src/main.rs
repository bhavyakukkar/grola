
fn main() {
    #[cfg(feature = "make-parsers")]
    {
        #[cfg(feature = "dynamic-server")]
        {
            let mut handlers = std::collections::HashMap::new();
            add_handlers_from_out_dir(&mut handlers);
            make_parsers();
            dynamic_server(&mut handlers);
        }
        #[cfg(not(feature = "dynamic-server"))]
        {
            make_parsers();
        }
    }
    #[cfg(not(feature = "make-parsers"))]
    {
        #[cfg(feature = "dynamic-server")]
        {
            let mut handlers = std::collections::HashMap::new();
            add_handlers_from_src_dir(&mut handlers);
            dynamic_server(&mut handlers);
        }
        #[cfg(not(feature = "dynamic-server"))]
        {
            eprintln!("You need to select at least one feature.");
            exit(1);
        }
    }
}

#[cfg(feature = "make-parsers")]
fn make_parsers() {
    use std::{fs, path::Path};

    fs::write(
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/handlers.rs")), 
        include_str!(concat!(env!("OUT_DIR"), "/handlers.rs"))
    ).unwrap();
}


#[cfg(feature = "dynamic-server")]
mod parsers;

#[cfg(all(feature = "dynamic-server", not(feature = "make-parsers")))]
fn add_handlers_from_src_dir(
    handlers: &mut std::collections::HashMap<&str, fn() -> Result<String, String>>
)
{
    //...
    include!("handlers.rs");
    //...
}

#[cfg(all(feature = "dynamic-server", feature = "make-parsers"))]
fn add_handlers_from_out_dir(
    handlers: &mut std::collections::HashMap<&str, fn() -> Result<String, String>>
)
{
    //...
    include!(concat!(env!("OUT_DIR"), "/handlers.rs"));
    //...
}

#[cfg(feature = "dynamic-server")]
fn dynamic_server(handlers: &mut std::collections::HashMap<&str, fn() -> Result<String, String>>) {
    use std::io::{BufRead, Write};

    //over here
    //let routes: Routes
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();

    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();
        rdr.read_line(&mut l).unwrap();
        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut l = String::new();
                    rdr.read_line(&mut l).unwrap();
                    if l.trim().is_empty() { break; }
                }
                let mut p = std::path::PathBuf::new();
                p.push("htdocs");
                p.push(resource.trim_start_matches("/"));
                if resource.ends_with('/') { p.push("index.html"); }
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                //stream.write_all(&std::fs::read(p).unwrap()).unwrap();
                stream.write_all(
                    (handlers.get("blog-posts.html").unwrap())()
                        .unwrap_or_else(|err| err).as_bytes()
                ).unwrap();
            }
            _ => todo!()
        }
    }

    //println!("{}", (pages.get("blog-posts.html").unwrap())().unwrap());
    //loop {
    //    //let { page, attributes } = look_up(route);
    //    let page = "blog.html";

    //    println!("{}", (pages.get(page).unwrap())().unwrap());
    //    //page.parse();
    //    // src/main.rs
    //}
}
