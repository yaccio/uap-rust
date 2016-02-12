#ua-parser for rust
This is a user agent parser for Rust based on
[ua-parser](https://github.com/ua-parser).

##Usage example

```rust
let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3".to_string();
let p = parser::Parser::new().unwrap();
let c = p.parse(agent);

println!("{:?}",c);
//Output: Client { user_agent: UserAgent { family: "Mobile Safari", major: Some("5"), minor: Some("1"), patch: None }, os: OS { family: "iOS", major: Some("5"), minor: Some("1"), patch: Some("1"), patch_minor: None }, device: Device { family: "iPhone", brand: Some("Apple"), model: Some("iPhone") } }
```


##Building from source.
Recursive clone the [uap-core](https://github.com/ua-parser/uap-core) project for the parser regexes:

```
git submodule update --init --recursive
```

Then simply build or run tests via cargo:

```
cargo build
cargo test
```

##Documentation
