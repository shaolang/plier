/* Copyright 2020 Shaolang Ai
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

use serde_derive::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct PlierSpec {
    apps: HashMap<String, AppSpec>,
}

#[derive(Debug, Deserialize)]
struct AppSpec {
    home_name: String,
    bins: Vec<String>,
    versions: Option<Vec<Version>>,
}

#[derive(Debug, Deserialize)]
struct Version {
    version: String,
    path: PathBuf,
}

impl PlierSpec {
    pub fn load(string: &str) -> PlierSpec {
        if let Ok(spec) = toml::from_str::<PlierSpec>(string) {
            spec
        } else {
            PlierSpec {
                apps: HashMap::new(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn load_returns_empty_apps_map_when_input_is_empty() {
        let spec = super::PlierSpec::load("");

        assert_eq!(spec.apps.len(), 0);
    }

    #[test]
    fn load_returns_spec_with_missing_versions_when_versions_are_missing() {
        let spec = super::PlierSpec::load(indoc!(
            r#"[apps.java]
               home_name = "java_home"
               bins = ["bin"]
               "#
        ));

        assert_eq!(spec.apps.get("java").unwrap().home_name, "java_home")
    }

    #[test]
    fn load_returns_spec_with_versions_when_they_exist() {
        let spec = super::PlierSpec::load(indoc!(
            r#"[apps.python]
               home_name = 'pythonhome'
               bins = ['.', 'Scripts']

               [[apps.python.versions]]
               version = "3.7.0"
               path = "/path/to/python/3.7.0"
               "#
        ));
        let versions = spec.apps.get("python").unwrap().versions.as_ref();

        assert_eq!(versions.unwrap()[0].version, "3.7.0");
    }
}
