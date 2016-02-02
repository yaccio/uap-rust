use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use yaml;
use regex::Regex;


#[derive(Debug)]
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

            UserAgent {
                family: family,
                major: None,
                minor: None,
                patch: None,
            }
        })
    }
}
