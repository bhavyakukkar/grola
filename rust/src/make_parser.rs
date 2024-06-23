use std::{env, fs, fmt, io, path::Path};
use toml::from_str;
use serde::Deserialize;

//impl<T: fmt::Display> Result<_, T> {
//    fn str_err(self) -> Result<_, String> {
//        self.map_err(|err| err.to_string())
//    }
//}

fn parse_template(template: String) -> (/*header: */String, /*content: */String) {
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

fn main() -> Result<(), ParserMakerError> {
    let out_dir = env::var("OUT_DIR")?;
    let mut out_rs = String::new();
    
    out_rs +=
        "
{
        ";

    for template_entry in fs::read_dir(Path::new(&env::var("TEMPLATES_DIR")?))? {
        let template_name = template_entry?.path();
        let template = fs::read_to_string(template_name.clone())?;
        let (header_toml, content) = parse_template(template);
        let header: TemplateHeader = from_str(&header_toml)?;

        //todo: add attributes
        out_rs += &format!(
            "
    pages.insert(
        \"{template_name}\", 
        || -> Result<String, String> {{
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

            tt.render(r#\"{template_name}\"#, &data)
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
            tt.render(r#\"{template}\"#, &())
                .map_err(|err| err.to_string())
        }}
    );
                ",
                template = content,
            );
        }
    }

    out_rs +=
        "
}
        ";
    
    fs::write(Path::new(&out_dir).join(format!("templates.rs")), out_rs)?;
    Ok(())
}

#[derive(Deserialize)]
struct PullOptions {
    data: String,
    parser: String,
}

#[derive(Deserialize)]
struct TemplateHeader {
    pull: Option<PullOptions>,
    includes: Option<Vec<String>>,
}

#[derive(Debug)]
struct ParserMakerError(String);

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
