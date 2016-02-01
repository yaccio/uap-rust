use std::io;
use std::io::prelude::*;
use std::fs::{File};
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};

use client;
use ua::{UserAgentRegex};
use result::*;

#[derive(Debug)]
pub struct Parser {
    ua_regex: Vec<UserAgentRegex>,
    devices_regex: Vec<String>,
    os: Vec<String>,
}

impl Parser {
    pub fn new(regexes_file: &str) -> Result<Parser> {
        let mut file = try!(File::open(regexes_file));
        let mut yaml = String::new();
        file.read_to_string(&mut yaml);
        let docs = try!(YamlLoader::load_from_str(&yaml));
        let doc = docs[0].clone();
        match doc {
            Yaml::Hash(h) => {
                let get_yaml_vec = |s: &str, f| {
                    let y = h.get(&Yaml::String(s.to_string()));
                    match y.and_then(|arr| arr.as_vec()) {
                        Some(v) => Ok(v.into_iter().filter_map(f).collect()),
                        None => Err(Error::from(format!("YAML missing: {}", s))) 
                    }
                };
                let p = Parser {
                    devices_regex: Vec::new(),
                    ua_regex: try!(get_yaml_vec("user_agent_parsers",
                                           UserAgentRegex::from_yaml)),
                    os: Vec::new(),
                };
                Ok(p)
            },
            _ => Err(Error::from("YAML format error: Top level not hash")),
        }
    }

    pub fn parse(&self, agent: String) -> client::Client {
        panic!("Not implemented")
    }
}
