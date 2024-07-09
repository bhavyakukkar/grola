use std::{collections::HashMap, env, path::PathBuf, fs};
use grola::{get_config};

fn main() -> Result<(), String> {
    let mut handlers = HashMap::new();

    #[cfg(feature = "dynamic-server")]
    {
        eprintln!(
            "This binary target does not accept the `dynamic-server` feature. \
            Use the `dynamic-server` binary target instead."
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

    for (page, options) in config.routes.into_iter() {
        let html = handlers.remove(/*template*/&options.0 as &str)
            .map(|handler| handler(options.1, HashMap::new()))
            .ok_or(format!("Template not found: {}", options.0))?
            .map_err(|err| format!("Error while rendering template: {}", err))?;

        let render_dir = PathBuf::from(env::var("RENDER_DIR").unwrap_or(".".to_owned()));
        println!("Generating {}", render_dir.join(page.clone()).display());
        fs::write(render_dir.join(page), html)
            .map_err(|err| format!("Error while writing to page: {}", err))?;
    }
    Ok(())
}
