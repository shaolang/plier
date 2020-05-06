use std::collections::BTreeMap;
use std::path::PathBuf;
use toml::{self, Value};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SpecEntry {
    pub bins: Vec<String>,
}


pub type Spec = BTreeMap<String, SpecEntry>;

pub fn batch_filename(exe_path: PathBuf, filename: &str) -> PathBuf {
    let exe_str = exe_path.to_str().expect("Unable to get binary's path");
    let mut file = PathBuf::from(exe_str).parent()
        .expect("Unable to get binary's directory").to_owned();
    file.push(filename);
    file
}


pub fn add_spec_entry(existing: &str, name: &str, bins: Vec<String>)
        -> Result<String, toml::ser::Error> {
    let v: Value = toml::from_str(existing).unwrap();
    let mut m: Spec = v.try_into().unwrap();
    m.insert(name.to_string(), SpecEntry { bins });

    toml::to_string(&m)
}


pub fn load_spec(s: &str) -> Result<Spec, toml::de::Error> {
    let v: Value = toml::from_str(s).unwrap();
    v.try_into::<Spec>()
}
