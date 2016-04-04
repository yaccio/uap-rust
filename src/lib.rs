/*!
#ua-parser for rust
This is a user agent parser for Rust based on
[ua-parser](https://github.com/ua-parser).

##Usage example

```rust
use uap_rust::parser::Parser;
let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3".to_string();
let p = Parser::new().unwrap();
let c = p.parse(agent);

println!("{:?}",c);
 //Output: Client { user_agent: UserAgent { family: "Mobile Safari", major: Some("5"), minor: Some("1"), patch: None }, os: OS { family: "iOS", major: Some("5"), minor: Some("1"), patch: Some("1"), patch_minor: None }, device: Device { family: "iPhone", brand: Some("Apple"), model: Some("iPhone") } }
```
*/

extern crate yaml_rust;
extern crate regex;

pub mod parser;
pub mod client;
pub mod ua;
pub mod os;
pub mod device;
mod result;
mod yaml;

#[cfg(test)]
mod test {
    use parser;
    use client::Client;
    use ua::UserAgent;
    use device::Device;
    use os::OS;
    use yaml::*;
    use yaml_rust::{YamlLoader, Yaml};
    use std::io::prelude::*;
    use std::fs::{File};

    #[test]
    fn basic_au_test() {
        let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3".to_string();
        let p = parser::Parser::new().unwrap();
        let c = p.parse(agent);
        println!("{:?}",c);
        assert_eq!(Client {
            user_agent: UserAgent {
                family: "Mobile Safari".to_string(),
                major: Some("5".to_string()),
                minor: Some("1".to_string()),
                patch: None,
            },
            device: Device {
                family: "iPhone".to_string(),
                brand: Some("Apple".to_string()),
                model: Some("iPhone".to_string()),
                regex: Some("(iPhone)(?:;| Simulator;)".to_string()),
            },
            os: OS {
                family: "iOS".to_string(),
                major: Some("5".to_string()),
                minor: Some("1".to_string()),
                patch: Some("1".to_string()),
                patch_minor: None,
            }
        }, c);
    }

    #[test]
    fn test_device() {
        let p = parser::Parser::new().unwrap();
        assert!(p.devices_regex.len() > 0);
        let mut test_file = File::open("src/uap-core/tests/test_device.yaml").unwrap();
        let mut yaml_str = String::new();
        let _ = test_file.read_to_string(&mut yaml_str).unwrap();
        let y = YamlLoader::load_from_str(&yaml_str).unwrap();
        let cases = from_map(&y[0], "test_cases").unwrap();
        let failed = filter_map_over_arr(cases, |c| {
            let uas = from_map(c, "user_agent_string").unwrap().as_str().unwrap();
            let client = p.parse(uas.to_string());

            let family = compare_with_parsed(&Some(client.device.family.clone()), "family", c, &client);
            if family.is_some() {
                return family;
            }

            let brand = compare_with_parsed(&client.device.brand, "brand", c, &client);
            if brand.is_some() {
                return brand;
            }

            let model = compare_with_parsed(&client.device.model, "model", c, &client);
            if model.is_some() {
                return model;
            }
            None
        });

        //for f in failed.clone() {
        //    println!("{}", f);
        //}

        assert_eq!(0, failed.len());
    }

    #[test]
    fn test_user_agent() {
        let p = parser::Parser::new().unwrap();
        assert!(p.devices_regex.len() > 0);
        let mut test_file = File::open("src/uap-core/tests/test_ua.yaml").unwrap();
        let mut yaml_str = String::new();
        let _ = test_file.read_to_string(&mut yaml_str).unwrap();
        let y = YamlLoader::load_from_str(&yaml_str).unwrap();
        let cases = from_map(&y[0], "test_cases").unwrap();
        let failed = filter_map_over_arr(cases, |c| {
            let uas = from_map(c, "user_agent_string").unwrap().as_str().unwrap();
            let client = p.parse(uas.to_string());

            let family = compare_with_parsed(&Some(client.user_agent.family.clone()), "family", c, &client);
            if family.is_some() {
                return family;
            }

            let major = compare_with_parsed(&client.user_agent.major, "major", c, &client);
            if major.is_some() {
                return major;
            }

            let minor = compare_with_parsed(&client.user_agent.minor, "minor", c, &client);
            if minor.is_some() {
                return minor;
            }

            let patch = compare_with_parsed(&client.user_agent.patch, "patch", c, &client);
            if patch.is_some() {
                return patch;
            }

            None
        });

        //for f in failed.clone() {
        //    println!("{}", f);
        //}

        assert_eq!(0, failed.len());
    }

    #[test]
    fn test_os() {
        let p = parser::Parser::new().unwrap();
        assert!(p.devices_regex.len() > 0);
        let mut test_file = File::open("src/uap-core/tests/test_os.yaml").unwrap();
        let mut yaml_str = String::new();
        let _ = test_file.read_to_string(&mut yaml_str).unwrap();
        let y = YamlLoader::load_from_str(&yaml_str).unwrap();
        let cases = from_map(&y[0], "test_cases").unwrap();
        let failed = filter_map_over_arr(cases, |c| {
            let uas = from_map(c, "user_agent_string").unwrap().as_str().unwrap();
            let client = p.parse(uas.to_string());

            let family = compare_with_parsed(&Some(client.os.family.clone()), "family", c, &client);
            if family.is_some() {
                return family;
            }

            let major = compare_with_parsed(&client.os.major, "major", c, &client);
            if major.is_some() {
                return major;
            }

            let minor = compare_with_parsed(&client.os.minor, "minor", c, &client);
            if minor.is_some() {
                return minor;
            }

            let patch = compare_with_parsed(&client.os.patch, "patch", c, &client);
            if patch.is_some() {
                return patch;
            }

            let patch_minor = compare_with_parsed(&client.os.patch_minor, "patch_minor", c, &client);
            if patch_minor.is_some() {
                return patch_minor;
            }

            None
        });

        //for f in failed.clone() {
        //    println!("{}", f);
        //}

        assert_eq!(0, failed.len());
    }

    fn compare_with_parsed(actual: &Option<String>, mapkey: &str, c: &Yaml, client: &Client) -> Option<String> {
        let opt = from_map(c, mapkey).unwrap();
        if !opt.is_null() {
            let value = opt.as_str().unwrap().to_string();
            let parsed = actual.clone().unwrap_or(String::new());
            if parsed != value { 
                return Some(format!("{} does not match: {:?}, actual: {:?}", mapkey, c, client));
            }
        }
        None
    }

}
