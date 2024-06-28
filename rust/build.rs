#[cfg(not(feature = "make-parsers"))]
fn main() {}

#[cfg(feature = "make-parsers")]
fn main() -> Result<(), make_parsers::ParserMakerError> {
    use make_parsers::*;
    use std::{env, fs, path::Path};
    use toml::from_str;

    println!("\n> Making Parsers now");
    let templates_dir = env::var("TEMPLATES_DIR")?;
    let out_dir = env::var("OUT_DIR")?;
    let mut out_rs = String::new();

    out_rs += "
{
        ";

    for template_entry in fs::read_dir(Path::new(&templates_dir))? {
        let template_name = template_entry?.path();
        let template = fs::read_to_string(template_name.clone())?;
        let (header_toml, content) = parse_template(template);
        let header: TemplateHeader = from_str(&header_toml)?;

        out_rs += &format!(
            "
    handlers.insert(
        \"{template_name}\", 
        |attributes: Attributes, query: HashMap<String, String>|
        -> Result<String, String>
        {{
            use tinytemplate::TinyTemplate;
    
            #[allow(unused_mut)]
            let mut tt = TinyTemplate::new();
            tt.add_template(\"{template_name}\", r#\"{template}\"#)
                .map_err(|err| err.to_string())?;
            //...
            ",
            template = content,
            template_name = template_name.file_name().unwrap().to_str().unwrap(),
        );

        if let Some(includes) = header.includes {
            for include_name in includes {
                out_rs += &format!(
                    "
            tt.add_template(\"{include_name}\", r#\"{include}\"#)
                .map_err(|err| err.to_string())?;
                    ",
                    include_name = include_name,
                    include = fs::read_to_string(
                        Path::new(&env::var("TEMPLATES_DIR")?).join(include_name.clone())
                    )?,
                );
            }
        }

        if let Some(pull) = header.pull {
            out_rs += &format!(
                "
            //...
            use crate::parsers;
            use std::fs;
            use toml::from_str;

            let data: parsers::{parser} =
                from_str(&fs::read_to_string(\"{data}\")
                    .map_err(|err| err.to_string())?)
                        .map_err(|err| err.to_string())?;

            let context: Context<parsers::{parser}> = Context {{
                query,
                attributes,
                data: Some(data),
            }};

            tt.render(r#\"{template_name}\"#, &context)
                .map_err(|err| err.to_string())
        }}
    );
                ",
                parser = pull.parser,
                data = Path::new(&env::var("DATA_DIR")?).join(pull.data).display(),
                template_name = template_name.file_name().unwrap().to_str().unwrap(),
            );
        } else {
            out_rs += &format!(
                "
            //...
            let context: Context<()> = Context {{
                query,
                attributes,
                data: None,
            }};

            tt.render(r#\"{template_name}\"#, &context)
                .map_err(|err| err.to_string())
        }}
    );
                ",
                template_name = template_name.file_name().unwrap().to_str().unwrap(),
            );
        }
    }

    out_rs += "
}
        ";

    fs::write(Path::new(&out_dir).join(format!("handlers.rs")), out_rs)?;
    println!("cargo:rerun-if-changed={}", templates_dir);
    Ok(())
}

#[cfg(feature = "make-parsers")]
mod make_parsers {
    use std::{env, fmt, io};

    pub fn parse_template(template: String) -> (/*header: */ String, /*content: */ String) {
        let mut region = 0u8; //0 -> before first ===
                              //1 -> after first === & before second ===
                              //2 -> after second ===
        let mut content = String::new();
        let mut header = String::new();
        for line in template.lines() {
            if line == "===" {
                region += 1;
                continue;
            }

            if region == 0 || region == 2 {
                content += line;
                content += "\n";
            } else if region == 1 {
                header += line;
                header += "\n";
            } else {
                break;
            }
        }

        (header, content)
    }

    #[derive(serde::Deserialize)]
    pub struct PullOptions {
        pub data: String,
        pub parser: String,
    }

    #[derive(serde::Deserialize)]
    pub struct TemplateHeader {
        pub pull: Option<PullOptions>,
        pub includes: Option<Vec<String>>,
    }

    #[derive(Debug)]
    pub struct ParserMakerError(String);

    impl fmt::Display for ParserMakerError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl From<io::Error> for ParserMakerError {
        fn from(item: io::Error) -> ParserMakerError {
            ParserMakerError(item.to_string())
        }
    }

    impl From<env::VarError> for ParserMakerError {
        fn from(item: env::VarError) -> ParserMakerError {
            ParserMakerError(item.to_string())
        }
    }

    impl From<toml::de::Error> for ParserMakerError {
        fn from(item: toml::de::Error) -> ParserMakerError {
            ParserMakerError(item.to_string())
        }
    }
}
