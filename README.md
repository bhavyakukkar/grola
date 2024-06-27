
# ROADMAP
+ [x] split making parsers and making server into two binaries,
      the first binary just copies the contents of OUT_DIR/templates.rs into a file in src/,
      the second binary then include!'s src/templates.rs instead
+ [x] make build.rs have to invoke every time files in templates/ change
+ [ ] include TOML route attributes as well as request parameters in the template attributes
+ [ ] rename parsers to handlers
+ [ ] toml strings are parsed as markdown to allow rich-text
+ [ ] make data-pull relative so that server binary can find data-files based on passed argument
      flag. after done, make bin/ in root where executables get moved from rust/target/release
+ [ ] allow static generation (every template parsed once and stored as html)
+ [ ] make binary that creates entire grola directory-tree and executes all grola commands


# PREREQUISITES
+ [TOML](https://toml.io/en/) syntax
+ [tinytemplate](https://docs.rs/tinytemplate/) syntax
+ [Rust Types](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html) and their [serialization](https://serde.rs/)
+ Format of deserialization into Rust types followed by [the 'toml' crate](https://docs.rs/toml/)


What grola does when a request is made
1. looks up `routes.toml` to match `ROUTE` to template page `pages/page.html`
2. furbishes `page.html` that requires `parser` with `data/
3. 

Compile-time:
+ Register blog-posts.html to use parser BlogPosts that parses blog-posts.toml

Serve-time:
+ Actually parse blog-posts.toml with BlogPosts into blog-posts.html

```sh
parser --parser BlogPosts --data blog-posts.toml --template blog-posts.html
```
