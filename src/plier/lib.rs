/*
* Copyright 2020 Shaolang Ai
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

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


pub fn add_spec_entry(spec: &mut Spec, name: &str, bins: Vec<String>) {
    spec.insert(name.to_string(), SpecEntry { bins });
}


pub fn load_spec(s: &str) -> Result<Spec, toml::de::Error> {
    let v: Value = toml::from_str(s).unwrap();
    v.try_into::<Spec>()
}
