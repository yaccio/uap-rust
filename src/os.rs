use yaml_rust::{Yaml};
use yaml;
use regex::Regex;

///`OS` contains the operating system information from the user agent.
#[derive(Debug, PartialEq, Eq)]
pub struct OS {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>,
}

#[derive(Debug)]
pub struct OSParser {
    pub regex: Regex,
    pub family: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>,
}

impl OSParser {
    pub fn from_yaml(y: &Yaml) -> Option<OSParser> {
            yaml::string_from_map(y, "regex")
            .map(|r| r.replace(r"\-", r"-"))
            .map(|r| r.replace(r"\ ", r" "))
            .map(|r| r.replace(r"\/", r"/"))
            .and_then(|r| Regex::new(&r[..]).ok())
            .map(|r| OSParser {
                regex: r,
                family: yaml::string_from_map(y, "os_replacement"),
                major: yaml::string_from_map(y, "os_v1_replacement"),
                minor: yaml::string_from_map(y, "os_v2_replacement"),
                patch: yaml::string_from_map(y, "os_v3_replacement"),
                patch_minor: yaml::string_from_map(y, "os_v4_replacement"),
            })
    }

    pub fn parse(&self, agent: String) -> Option<OS> {
        self.regex.captures(&agent[..]).map(|c| {
            let family = self.family.clone()
                .and_then(|f| c.at(1).map(|a| f.replace("$1", a)))
                .unwrap_or(c.at(1).unwrap_or("Other").to_string());
            let major = self.major.clone()
                .and_then(|f| c.at(2).map(|a| f.replace("$2", a)))
                .or(c.at(2).map(String::from));
            let minor = self.minor.clone()
                .and_then(|f| c.at(3).map(|a| f.replace("$3", a)))
                .or(c.at(3).map(String::from));
            let patch = self.patch.clone()
                .and_then(|f| c.at(4).map(|a| f.replace("$4", a)))
                .or(c.at(4).map(String::from));
            let patch_minor = self.patch_minor.clone()
                .and_then(|f| c.at(5).map(|a| f.replace("$5", a)))
                .or(c.at(5).map(String::from));

            OS {
                family: family,
                major: major,
                minor: minor,
                patch: patch,
                patch_minor: patch_minor,
            }
        })
    }
}
