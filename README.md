#ua-parser for rust
User agent parser library for Rust based on the
[ua-parser](https://github.com/ua-parser) project.

Add to your `Cargo.toml`:

```
[dependencies]
uap-rust = "0.0.*"
```

##Usage example

```rust
use uap_rust::parser::Parser;
let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3".to_string();
let p = Parser::new().unwrap();
let c = p.parse(agent);

println!("{:?}",c);
 //Output: Client { user_agent: UserAgent { family: "Mobile Safari", major: Some("5"), minor: Some("1"), patch: None }, os: OS { family: "iOS", major: Some("5"), minor: Some("1"), patch: Some("1"), patch_minor: None }, device: Device { family: "iPhone", brand: Some("Apple"), model: Some("iPhone") } }
```

##Documentation

Documentation is available [here](https://mrbechcrates.github.io/uap-rust-doc/uap_rust/index.html)

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
