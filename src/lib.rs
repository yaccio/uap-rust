extern crate yaml_rust;

mod parser;
mod client;
mod ua;
mod os;
mod device;
mod result;

#[cfg(test)]
mod test {
    use parser;
    #[test]
    fn basic_au_test() {
        let ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3".to_string();
        let p = parser::Parser::new("uap-core/regexes.yaml").unwrap();
    }
}
