use serde::Deserialize;
use std::{collections::HashMap, fs, io, path::PathBuf};
use toml::{de, from_str, map::Map, Table, Value};

pub type Page = String;
pub type Attributes = Table;
pub type Route = String;
pub type Routes = HashMap<Route, (Page, Attributes)>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    pub routes: Routes,
    pub server: Option<ServerConfig>,
}

#[derive(Debug)]
pub enum ConfigErr {
    RouteErr(RouteErr),
    ServerErr(ServerErr),
}

#[derive(Debug)]
pub enum RouteErr {
    FileNotFound(io::Error),
    DeErr(de::Error),
    InvalidTable,
    NoPageAttr,
    InvalidPageAttr,
}

#[derive(Debug)]
pub enum ServerErr {
    StaticExpectedTable,
    StaticNoRouteAttr,
    StaticNoDirAttr,
    StaticInvalidRouteAttr,
    StaticInvalidDirAttr,
}

#[derive(Debug)]
pub struct ServerConfig {
    pub static_route: Option<(String, String)>,
}

#[derive(Deserialize)]
struct RawConfig {
    routes: Table,
    server: Option<Table>,
}

pub fn get_config(path: &PathBuf) -> Result<Config, ConfigErr> {
    use RouteErr::*;
    use ServerErr::*;
    use ConfigErr::{RouteErr as RE, ServerErr as SE};

    let raw_config =
        from_str::<RawConfig>(&fs::read_to_string(path).map_err(|err| RE(FileNotFound(err)))?)
            .map_err(|err| RE(DeErr(err)))?;
    let mut routes: Routes = HashMap::new();
    for (route, options) in raw_config.routes {
        let _ = match options {
            Value::String(page) => routes.insert(route, (page, Map::new())),
            Value::Table(mut table) => routes.insert(
                route,
                (
                    if let Value::String(page) = table.remove("page").ok_or(RE(NoPageAttr))? {
                        page
                    } else {
                        return Err(RE(InvalidPageAttr));
                    },
                    table,
                ),
            ),
            _ => {
                return Err(RE(InvalidTable));
            }
        };
    }

    let server = if let Some(mut raw_server_config) = raw_config.server {
        Some(ServerConfig {
            static_route: if let Some(mut static_route) = raw_server_config.remove("static") {
                let static_route = static_route.as_table_mut().ok_or(SE(StaticExpectedTable))?;
                Some((
                    if let Value::String(route) = static_route.remove("route")
                        .ok_or(SE(StaticNoRouteAttr))?
                    {
                        route
                    }
                    else {
                        return Err(SE(StaticInvalidRouteAttr));
                    },
                    if let Value::String(dir) = static_route.remove("dir")
                        .ok_or(SE(StaticNoDirAttr))?
                    {
                        dir
                    }
                    else {
                        return Err(SE(StaticInvalidDirAttr));
                    }
                ))
            } else {
                None
            },
        })
    } else {
        None
    };

    Ok(Config { routes, server })
}
