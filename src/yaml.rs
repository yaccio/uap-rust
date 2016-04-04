use yaml_rust::{Yaml};

pub fn from_map<'a>(map: &'a Yaml, key: &str) -> Option<&'a Yaml> {
    map.as_hash().and_then(|h| h.get(&Yaml::String(key.to_string())))
}

pub fn string_from_map(map: &Yaml, key: &str) -> Option<String> {
    from_map(map, key).and_then(|y| y.as_str()).map(|s| s.to_string())
}

pub fn filter_map_over_arr<T,F>(arr: &Yaml, f: F) -> Vec<T> where
F: Fn(&Yaml) -> Option<T>{
    arr.as_vec().map(|a| a.iter().filter_map(f).collect()).unwrap_or(Vec::new())
}
