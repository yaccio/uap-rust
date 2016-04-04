use yaml_rust::{Yaml};
use yaml;
use regex::{Regex, Captures};

///`Device` contains the device information from the user agent.
#[derive(Debug, PartialEq, Eq)]
pub struct Device {
    pub family: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub regex: Option<String>,
}

#[derive(Debug)]
pub struct DeviceParser {
    pub regex: Regex,
    pub family: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
}

impl DeviceParser {
    pub fn from_yaml(y: &Yaml) -> Option<DeviceParser> {
        let regex_flag = yaml::string_from_map(y, "regex_flag");
        yaml::string_from_map(y, "regex")
            .map(|r| 
                 if regex_flag.is_some() { 
                     format!("(?i){}", r)
                 }else{
                     r
                 }
            )
            .map(|r| r.replace(r"\-", r"-"))
            .map(|r| r.replace(r"\ ", r" "))
            .map(|r| r.replace(r"\/", r"/"))
            .and_then(|r| Regex::new(&r[..]).ok())
            .map(|r| DeviceParser {
                regex: r,
                family: yaml::string_from_map(y, "device_replacement"),
                brand: yaml::string_from_map(y, "brand_replacement"),
                model: yaml::string_from_map(y, "model_replacement"),
            })
    }
    fn replace(captures: &Captures, s: String) -> String {
        captures.iter().zip((0..captures.len()))
            .fold(s, |a, (c, i)| a.replace(&format!("${}", i)[..], c.unwrap_or("")))
            .trim().to_string()
    }

    pub fn parse(&self, agent: String) -> Option<Device> {
        self.regex.captures(&agent[..]).map(|c| {
            let family = self.family.clone()
                .map(|f| DeviceParser::replace(&c, f))
                .unwrap_or(c.at(1).unwrap_or("Other").to_string());
            let brand = self.brand.clone()
                .map(|f| DeviceParser::replace(&c, f))
                .or(c.at(1).map(|s| s.to_string()));
            let model = self.model.clone()
                .map(|f| DeviceParser::replace(&c, f))
                .or(c.at(1).map(|s| s.to_string()));
            Device {
                family: family,
                brand: brand,
                model: model,
                regex: Some(format!("{}", self.regex)),
            }
        })
    }
}
