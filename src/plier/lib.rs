use std::collections::BTreeMap;
use std::path::PathBuf;
use toml::{self, Value};

pub fn batch_filename(exe_path: PathBuf, filename: &str) -> PathBuf {
    let exe_str = exe_path.to_str().expect("Unable to get binary's path");
    let mut file = PathBuf::from(exe_str).parent()
        .expect("Unable to get binary's directory").to_owned();
    file.push(filename);
    file
}


pub fn create_spec(existing: &str, name: &str, bins: Vec<String>)
        -> Result<String, toml::ser::Error> {
    let v: Value = toml::from_str(existing).unwrap();
    let mut m: BTreeMap<String, BTreeMap<String, Value>> = v.try_into().unwrap();

    let bins: Vec<Value> = bins.iter()
        .map(|s| Value::String(s.to_string()))
        .collect();
    let mut sm: BTreeMap<String, toml::Value> = BTreeMap::new();

    sm.insert("bins".to_string(), toml::Value::Array(bins));
    m.insert(name.to_string(), sm);

    toml::to_string(&m)
}
