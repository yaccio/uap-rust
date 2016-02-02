extern crate yaml_rust;
extern crate regex;

mod parser;
mod client;
mod ua;
mod os;
mod device;
mod result;
mod yaml;

#[cfg(test)]
mod test {
    use parser;
    #[test]
    fn basic_au_test() {
        let agent = "Mozilla/5.0 (Windows; Windows NT 5.1; rv:2.0b3pre) Gecko/20100727 Minefield/4.0.1pre".to_string();
        let p = parser::Parser::new("uap-core/regexes.yaml").unwrap();
        let c = p.parse(agent);
        println!("{:?}", c);
    }
}
