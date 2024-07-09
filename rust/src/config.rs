use serde::Deserialize;
use std::{collections::HashMap, fs, io, path::PathBuf, fmt};
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
    FileNotFound(io::Error),
    DeErr(de::Error),
    RouteErr(RouteErr),
    ServerErr(ServerErr),
}

impl fmt::Display for ConfigErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ConfigErr::*;
        match self {
            FileNotFound(err) => write!(f, "Config File not found:\n{}", err),
            DeErr(err) => write!(f, "Config File deserialization error:\n{}", err),
            RouteErr(err) => write!(f, "Config Error in [routes]:\n{}", err),
            ServerErr(err) => write!(f, "Config Error in [server]:\n{}", err),
        }
    }
}

#[derive(Debug)]
pub enum RouteErr {
    InvalidTable,
    NoPageAttr,
    InvalidPageAttr,
}

impl fmt::Display for RouteErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RouteErr::*;
        write!(f, "{}", match self {
            InvalidTable => "Expected route fields to be TOML tables or strings",
            NoPageAttr => "Required attribute 'page' not found in at least one route field",
            InvalidPageAttr => "Expected attribute 'page' in route fields to be a TOML string",
        })
    }
}

#[derive(Debug)]
pub enum ServerErr {
    StaticExpectedTable,
    StaticNoRouteAttr,
    StaticNoDirAttr,
    StaticInvalidRouteAttr,
    StaticInvalidDirAttr,
}

impl fmt::Display for ServerErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ServerErr::*;
        write!(f, "{}", match self {
            StaticExpectedTable => "Expected field 'static' to be a TOML table",
            StaticNoRouteAttr => "Required attribute 'route' in field 'static' not found",
            StaticNoDirAttr => "Required attribute 'dir' in field 'static' not found",
            StaticInvalidRouteAttr => "Expected attribute 'route' in field 'static' to be a TOML string",
            StaticInvalidDirAttr => "Expected attribute 'dir' in field 'static' to be a TOML string",
        })
    }
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
    use ConfigErr::{FileNotFound as FNF, DeErr as DE, RouteErr as RE, ServerErr as SE};

    let raw_config =
        from_str::<RawConfig>(&fs::read_to_string(path).map_err(|err| FNF(err))?)
            .map_err(|err| DE(err))?;
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
