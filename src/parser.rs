use std::io::prelude::*;
use std::fs::{File};
use yaml_rust::{YamlLoader};

use client::{Client};
use ua::{UserAgent, UserAgentParser};
use device::{Device, DeviceParser};
use os::{OS, OSParser};
use result::*;
use yaml;

///The `Parser` type, used for parsing user agent strings into `Client` structs.
///
///Example usage:
///
///```rust
///let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3".to_string();
///let p = parser::Parser::new().unwrap();
///let c = p.parse(agent);
///
///println!("{:?}",c);
/// //Output: Client { user_agent: UserAgent { family: "Mobile Safari", major: Some("5"), minor: Some("1"), patch: None }, os: OS { family: "iOS", major: Some("5"), minor: Some("1"), patch: Some("1"), patch_minor: None }, device: Device { family: "iPhone", brand: Some("Apple"), model: Some("iPhone") } }
///```
#[derive(Debug)]
pub struct Parser {
    pub ua_regex: Vec<UserAgentParser>,
    pub devices_regex: Vec<DeviceParser>,
    pub os_regex: Vec<OSParser>,
}

impl Parser {

    ///Constructs a `Parser` from a file path to a regexes file.
    ///
    ///See [uap-core](https://github.com/ua-parser/uap-core/) documentation for information on the
    ///file format.
    pub fn from_file(regexes_file: &str) -> Result<Parser> {
        let mut file = try!(File::open(regexes_file));
        let mut yaml = String::new();
        let _ = file.read_to_string(&mut yaml);
        Parser::from_str(&yaml)
    }

    ///Constructs a `Parser` from an str containing regexes.
    ///
    ///See [uap-core](https://github.com/ua-parser/uap-core/) documentation for information on the
    ///format.
    pub fn from_str(s: &str) -> Result<Parser> {
        //Parse the yaml.
        let docs = try!(YamlLoader::load_from_str(&s));
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

    ///Constructs a `Parser` from the staticly complied regexes file data.
    ///
    ///See [uap-core](https://github.com/ua-parser/uap-core/) documentation for information on the
    ///format.
    pub fn new() -> Result<Parser> {
        let s = include_str!("uap-core/regexes.yaml");
        Parser::from_str(&s)
    }

    ///Parses a user agent string into a `Client` struct.
    pub fn parse(&self, agent: String) -> Client {
        //For each of the attributes, we find the first regex that matches and use that. Otherwise
        //we use a default value.

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
