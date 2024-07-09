
# Grola
Grola is a semi static site generator.

Offering flexibility halfway between a static & a responsive site, you can use Grola:
1. As conventional site-gen: render source data into the output HTML, or
2. Halfway to conventional site-gen: compile handlers but have source data fetched during requests 

There are three main constructs in Grola:
1. Data, your source data, written in **TOML**
2. Template, your views, written with **tinytemplate** for templating
3. Parser, the data-definition deserializing your data & serializing it into the template,
   currently written in **🦀 Rust 🦀**

The source-code of your website (parsers) is also the source-code of Grola, as the parsers are written in Rust, allowing for great **flexibility** in how your data gets filled into your templates.

Meaning, if you want to make your website with Grola, you need to start by **cloning this project**
and installing [Cargo](https://doc.rust-lang.org/cargo/) (on its own, or using [Rustup](https://www.rust-lang.org/tools/install)).

Parsers for popular generic usages are shown in [examples](./examples/parsers.rs)

The parsers & templates together compile into the handlers, meaning if either change, the handlers
must be compiled.


```sh
# compile the handlers into /rust/src/handlers.rs
make parsers

# recompile the handlers (if changed) and start the dynamic server after reading /config-server.toml
make server+

# just start the dynamic server after reading /config-server.toml (won't work if handlers aren't compiled)
make server

# recompile the handlers (if changed) and generate the output HTML (conventional site-gen) after reading /config-pages.toml
make pages+

# just generate the output HTML after reading /config-pages.toml (won't work if handlers aren't compiled)
make pages
```


# ROADMAP
+ [x] split making parsers and making server into two binaries,
      the first binary just copies the contents of OUT_DIR/templates.rs into a file in src/,
      the second binary then include!'s src/templates.rs instead
+ [x] make build.rs have to invoke every time files in templates/ change
+ [x] include TOML route attributes as well as request parameters in the template attributes
+ [ ] rename parsers to handlers
+ [x] allow static generation (every template parsed once and stored as html)
+ [x] 'public' folder for media, css and js
+ [ ] toml strings are parsed as markdown to allow rich-text
+ [ ] make data-pull relative so that server binary can find data-files based on passed argument
      flag. after done, make bin/ in root where executables get moved from rust/target/release
+ [ ] make binary that creates entire grola directory-tree and executes all grola commands (tough)


# PREREQUISITES
+ [TOML](https://toml.io/en/) syntax
+ [tinytemplate](https://docs.rs/tinytemplate/) syntax
+ [Rust Types](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html) and their [serialization](https://serde.rs/)
+ Format of deserialization into Rust types followed by [the 'toml' crate](https://docs.rs/toml/)

