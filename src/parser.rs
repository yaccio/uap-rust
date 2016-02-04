use std::io::prelude::*;
use std::fs::{File};
use yaml_rust::{YamlLoader};

use client::{Client};
use ua::{UserAgent, UserAgentParser};
use device::{Device, DeviceParser};
use os::{OS, OSParser};
use result::*;
use yaml;

#[derive(Debug)]
pub struct Parser {
    pub ua_regex: Vec<UserAgentParser>,
    pub devices_regex: Vec<DeviceParser>,
    pub os_regex: Vec<OSParser>,
}

impl Parser {
    pub fn new(regexes_file: &str) -> Result<Parser> {
        let mut file = try!(File::open(regexes_file));
        let mut yaml = String::new();
        let _ = file.read_to_string(&mut yaml);
        let docs = try!(YamlLoader::load_from_str(&yaml));

        let p = Parser {
            devices_regex: yaml::from_map(&docs[0],"device_parsers")
                .map(|y| yaml::map_over_arr(y, DeviceParser::from_yaml)).unwrap(),
            ua_regex: yaml::from_map(&docs[0],"user_agent_parsers")
                .map(|y| yaml::map_over_arr(y, UserAgentParser::from_yaml)).unwrap(),
            os_regex: yaml::from_map(&docs[0],"os_parsers")
                .map(|y| yaml::map_over_arr(y, OSParser::from_yaml)).unwrap(),
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
        let dev = self.devices_regex.iter().filter_map(|d| d.parse(agent.clone())).next();
        let d = dev.unwrap_or(Device {
            family: "Other".to_string(),
            model: None,
            brand: None,
        });
        let oss = self.os_regex.iter().filter_map(|d| d.parse(agent.clone())).next();
        let o = oss.unwrap_or(OS {
            family: "Other".to_string(),
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        });
        Client {
            user_agent: u,
            os: o,
            device: d,
        }
    }
}
