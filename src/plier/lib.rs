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

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlierSpec {
    apps: HashMap<String, AppSpec>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AppSpec {
    home_name: String,
    bins: Vec<String>,
    versions: Option<Vec<Version>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Version {
    version: String,
    home_path: PathBuf,
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

impl fmt::Display for PlierSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", toml::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use std::collections::HashMap;
    use std::path::PathBuf;

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
               home_path = "/path/to/python/3.7.0"
               "#
        ));
        let versions = spec.apps.get("python").unwrap().versions.as_ref();

        assert_eq!(versions.unwrap()[0].version, "3.7.0");
    }

    #[test]
    fn fmt_converts_spec_to_string() {
        let version = super::Version {
            version: "1.10.3".to_string(),
            home_path: PathBuf::from("/path/to/elixir/1.10.3"),
        };
        let app_spec = super::AppSpec {
            home_name: "elixir_home".to_string(),
            bins: vec!["bin".to_string()],
            versions: Some(vec![version]),
        };
        let mut map: HashMap<String, super::AppSpec> = HashMap::new();
        map.insert("elixir".to_string(), app_spec);

        let spec = super::PlierSpec { apps: map };

        assert_eq!(
            format!("{}", &spec),
            indoc!(
                r#"[apps.elixir]
                   home_name = "elixir_home"
                   bins = ["bin"]

                   [[apps.elixir.versions]]
                   version = "1.10.3"
                   home_path = "/path/to/elixir/1.10.3"
                   "#
            )
        )
    }
}
