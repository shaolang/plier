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

use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use toml::{self, Value};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SpecEntry {
    pub bins: Vec<String>,
}

pub type Spec = BTreeMap<String, SpecEntry>;

trait Speccable<T> {
    fn add_entry(&mut self, name: &str, bins: Vec<String>);
    fn load(s: &str) -> Result<T, toml::de::Error>;
}

impl Speccable<Spec> for Spec {
    fn add_entry(&mut self, name: &str, bins: Vec<String>) {
        self.insert(name.to_string(), SpecEntry { bins });
    }

    fn load(s: &str) -> Result<Spec, toml::de::Error> {
        let v: Value = toml::from_str(s).unwrap();
        v.try_into::<Spec>()
    }
}

pub fn batch_filename(exe_path: PathBuf, filename: &str) -> PathBuf {
    let exe_str = exe_path.to_str().expect("Unable to get binary's path");
    let mut file = PathBuf::from(exe_str)
        .parent()
        .expect("Unable to get binary's directory")
        .to_owned();
    file.push(filename);
    file
}

#[cfg(test)]
mod tests {
    use super::Speccable;
    use indoc::indoc;
    use std::path::PathBuf;

    #[test]
    fn batch_filename_combines_executables_directory_with_given_filename() {
        let exe_path = PathBuf::from("path/to/binary.exe");
        let actual = super::batch_filename(exe_path, "batch.sh");
        let mut expected = PathBuf::from("path");
        expected.push("to");
        expected.push("batch.sh");

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_spec_for_new_devkit() {
        let mut actual = super::Spec::load("").unwrap();
        actual.add_entry("java", vec!["bin".to_string()]);
        let expected = super::Spec::load(indoc!(
            r#"[java]
               bins = ["bin"]
               "#
        ))
        .unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_spec_when_others_exists() {
        let mut actual = super::Spec::load("[java]\nbins = [\"bin\"]\n").unwrap();
        actual.add_entry("python", vec![".".to_string(), "Scripts".to_string()]);
        let expected = super::Spec::load(indoc!(
            r#"[java]
               bins = ["bin"]

               [python]
               bins = [".", "Scripts"]
               "#
        ))
        .unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn delete_non_existent_spec_entry() {
        let mut actual = super::Spec::load("[java]\nbins = ['bin']\n").unwrap();
        let expected = actual.clone();
        actual.remove("python");

        assert_eq!(actual, expected);
    }

    #[test]
    fn delete_known_entry() {
        let mut actual = super::Spec::load(indoc!(
            r#"[java]
               bins = ['bin']

               [python]
               bins = ['.', 'Scripts']
               "#
        ))
        .unwrap();

        actual.remove("python");

        assert_eq!(
            actual,
            super::Spec::load("[java]\nbins = ['bin']\n").unwrap()
        );
    }

    #[test]
    fn load_spec_from_empty_string() {
        let actual = super::Spec::load("").unwrap();

        assert_eq!(actual, super::Spec::new());
    }

    #[test]
    fn load_spec_from_valid_spec_string() {
        let actual = super::Spec::load(indoc!(
            r#"[java]
               bins = ["bin"]

               [python]
               bins = [".", "Scripts"]
               "#
        ))
        .unwrap();
        let mut expected = super::Spec::new();

        expected.insert(
            "java".to_string(),
            super::SpecEntry {
                bins: vec!["bin".to_string()],
            },
        );
        expected.insert(
            "python".to_string(),
            super::SpecEntry {
                bins: vec![".".to_string(), "Scripts".to_string()],
            },
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn load_spec_from_invalid_spec_string() {
        let actual = super::Spec::load(indoc!(
            r#"[hello]
               bins = ['sbin']

               [world]
               greeting = ['what?']
               "#
        ));

        assert!(actual.is_err());
    }
}
