use yaml_rust::{Yaml};
use yaml;
use regex::Regex;


///`UserAgent` contains the user agent information.
#[derive(Debug, PartialEq, Eq)]
pub struct UserAgent {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

#[derive(Debug)]
pub struct UserAgentParser {
    pub regex: Regex,
    pub family: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

impl UserAgentParser {
    pub fn from_yaml(y: &Yaml) -> Option<UserAgentParser> {
            yaml::string_from_map(y, "regex")
            .and_then(|r| Regex::new(&r[..]).ok())
            .map(|r| UserAgentParser {
                regex: r,
                family: yaml::string_from_map(y, "family_replacement"),
                major: yaml::string_from_map(y, "v1_replacement"),
                minor: yaml::string_from_map(y, "v2_replacement"),
                patch: yaml::string_from_map(y, "v3_replacement"),
            })
    }

    pub fn parse(&self, agent: String) -> Option<UserAgent> {
        self.regex.captures(&agent[..]).map(|c| {
            let family = self.family.clone()
                .and_then(|f| c.at(1).map(|a| f.replace("$1", a)))
                .unwrap_or(c.at(1).unwrap().to_string());
            let major = self.major.clone()
                .and_then(|f| c.at(2).map(|a| f.replace("$2", a)))
                .or(c.at(2).map(String::from));
            let minor = self.minor.clone()
                .and_then(|f| c.at(3).map(|a| f.replace("$3", a)))
                .or(c.at(3).map(String::from));
            let patch = self.patch.clone()
                .and_then(|f| c.at(4).map(|a| f.replace("$4", a)))
                .or(c.at(4).map(String::from));

            UserAgent {
                family: family,
                major: major,
                minor: minor,
                patch: patch,
            }
        })
    }
}
