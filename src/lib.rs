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
}
