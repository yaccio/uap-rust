use std::io;
use std::io::prelude::*;
use std::fs::{File};
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};

use client::{Client};
use ua::{UserAgent, UserAgentParser};
use device::{Device};
use os::{OS};
use result::*;
use yaml;

#[derive(Debug)]
pub struct Parser {
    pub ua_regex: Vec<UserAgentParser>,
    pub devices_regex: Vec<String>,
    pub os: Vec<String>,
}

impl Parser {
    pub fn new(regexes_file: &str) -> Result<Parser> {
        let mut file = try!(File::open(regexes_file));
        let mut yaml = String::new();
        file.read_to_string(&mut yaml);
        let docs = try!(YamlLoader::load_from_str(&yaml));

        //Uses f to parse yaml vec elems from hash key s
        let parse_yaml = |s: &str, f| {
            let y = yaml::from_map(&docs[0], s);
            match y.and_then(|arr| arr.as_vec()) {
                Some(v) => Ok(v.into_iter().filter_map(f).collect()),
                None => Err(Error::from(format!("YAML root key missing: {}", s))) 
            }
        };

        let p = Parser {
            devices_regex: Vec::new(),
            ua_regex: try!(parse_yaml("user_agent_parsers",
                                   UserAgentParser::from_yaml)),
            os: Vec::new(),
        };
        Ok(p)
    }

    pub fn parse(&self, agent: String) -> Client {
        let ua = self.ua_regex.iter().filter_map(|u| u.parse(agent.clone())).next();
        let u = ua.unwrap_or(
            UserAgent {
                family: "Other".to_string(),
                major: None,
                minor: None,
                patch: None,
            });
        let o = OS {
            family: "Other".to_string(),
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        };
        let d = Device {
            family: "Other".to_string(),
        };
        Client {
            user_agent: u,
            os: o,
            device: d,
        }
    }
}
