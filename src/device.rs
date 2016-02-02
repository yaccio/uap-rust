use yaml_rust::{Yaml};
use yaml;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Device {
    pub family: String,
}

#[derive(Debug)]
pub struct DeviceParser {
    pub regex: Regex,
    pub family: Option<String>,
}

impl DeviceParser {
    pub fn from_yaml(y: &Yaml) -> Option<DeviceParser> {
            yaml::string_from_map(y, "regex")
            .and_then(|r| Regex::new(&r[..]).ok())
            .map(|r| DeviceParser {
                regex: r,
                family: yaml::string_from_map(y, "family_replacement"),
            })
    }
    pub fn parse(&self, agent: String) -> Option<Device> {
        self.regex.captures(&agent[..]).map(|c| {
            let family = self.family.clone()
                .and_then(|f| c.at(1).map(|a| f.replace("$1", a)))
                .unwrap_or(c.at(1).unwrap().to_string());
            Device {
                family: family,
            }
        })
    }
}
