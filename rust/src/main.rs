mod parsers;

use std::collections::HashMap;

fn main() {
    let mut pages: HashMap<&str, fn() -> Result<String, String>> = HashMap::new();
    //let routes: Routes

    //...
    include!(concat!(env!("OUT_DIR"), "/templates.rs"));
    //...

    use std::io::{BufRead, Write};
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
                    (pages.get("blog-posts.html").unwrap())().unwrap_or_else(|err| err).as_bytes()
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
