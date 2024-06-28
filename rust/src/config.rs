use serde::Deserialize;
use std::{collections::HashMap, fs, io, path::PathBuf};
use toml::{de, from_str, map::Map, Table, Value};

type Page = String;
type Attributes = Table;
type Route = String;
type Routes = HashMap<Route, (Page, Attributes)>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    pub routes: Routes,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(io::Error),
    DeErr(de::Error),
    InvalidTable,
    NoPageAttr,
    InvalidPageAttr,
}

#[derive(Deserialize)]
struct RawConfig {
    routes: Table,
}

pub fn get_config(path: &PathBuf) -> Result<Config, ConfigError> {
    use ConfigError::*;

    let raw_config =
        from_str::<RawConfig>(&fs::read_to_string(path).map_err(|err| FileNotFound(err))?)
            .map_err(|err| DeErr(err))?;
    let mut routes: Routes = HashMap::new();
    for (route, options) in raw_config.routes {
        let _ = match options {
            Value::String(page) => routes.insert(route, (page, Map::new())),
            Value::Table(mut table) => routes.insert(
                route,
                (
                    if let Value::String(page) = table.remove("page").ok_or(NoPageAttr)? {
                        page
                    } else {
                        return Err(InvalidPageAttr);
                    },
                    table,
                ),
            ),
            _ => {
                return Err(InvalidTable);
            }
        };
    }
    Ok(Config { routes })
}
