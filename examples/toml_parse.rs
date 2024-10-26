use std::collections::HashMap;

fn main() {
    let mut values = Vec::new();
    for _ in 0..2 {
        let mut map = HashMap::new();
        map.insert("delimiter", ",".to_string());
        map.insert("header", "true".to_string());
        map.insert("format", "json".to_string());

        values.push(map);
    }

    let ss = toml::to_string(&values);
    println!("{:?}", ss);

    let mut map = HashMap::new();
    map.insert("delimiter", ",".to_string());
    map.insert("header", "true".to_string());
    map.insert("format", "json".to_string());
    let ss = toml::to_string(&map);
    println!("{:?}", ss);
}
