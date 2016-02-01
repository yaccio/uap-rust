use yaml_rust::{Yaml, YamlLoader, YamlEmitter};

#[derive(Debug)]
pub struct UserAgent {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

#[derive(Debug)]
pub struct UserAgentRegex {
    pub regex: String,
    pub family: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

impl UserAgentRegex {
    pub fn from_yaml(yaml: &Yaml) -> Option<UserAgentRegex> {
        yaml.as_hash().and_then(|h|
            h.get(&Yaml::String("regex".to_string()))
            .and_then(|s| s.as_str())
            .map(|r| UserAgentRegex {
                regex: r.to_string(),
                family: None,
                major: None,
                minor: None,
                patch: None,
            })
        )
    }
}
