use std::collections::BTreeMap;
use std::path::PathBuf;
use toml::{self, Value};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SpecEntry {
    bins: Vec<String>,
}


type Spec = BTreeMap<String, SpecEntry>;


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
    let mut m: Spec = v.try_into().unwrap();
    m.insert(name.to_string(), SpecEntry { bins });

    toml::to_string(&m)
}
