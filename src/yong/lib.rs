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

#[derive(Debug, Deserialize, Serialize)]
pub struct YongSpec {
    apps: HashMap<String, AppSpec>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AppSpec {
    home_name: String,
    bins: Vec<String>,
    versions: Option<Vec<Version>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Version {
    version: String,
    home_path: PathBuf,
}

impl YongSpec {
    pub fn load(string: &str) -> YongSpec {
        if let Ok(spec) = toml::from_str::<YongSpec>(string) {
            spec
        } else {
            YongSpec {
                apps: HashMap::new(),
            }
        }
    }

    pub fn add_app(&mut self, app_name: &str, home_name: &str, bins: &[&str]) {
        let app_spec = AppSpec {
            home_name: home_name.to_string(),
            bins: bins.iter().map(|s| s.to_string()).collect(),
            versions: None,
        };

        self.apps.insert(app_name.to_string(), app_spec);
    }

    pub fn upsert_version(&mut self, app_name: &str, ver: &str, home_path: &str) {
        let mut app = self.apps.get_mut(app_name).unwrap();
        let version = Version {
            version: ver.to_string(),
            home_path: PathBuf::from(home_path),
        };

        if let Some(versions) = &mut app.versions {
            let mut new_versions = Vec::with_capacity(versions.len());
            let mut is_upsert = false;
            let version = Version {
                version: ver.to_string(),
                home_path: PathBuf::from(home_path),
            };

            for v in versions.iter() {
                if v.version != ver {
                    new_versions.push(v.clone());
                } else {
                    is_upsert = true;

                    new_versions.push(version.clone());
                }
            }

            if !is_upsert {
                new_versions.push(version);
            }

            app.versions = Some(new_versions.to_vec());
        } else {
            app.versions = Some(vec![version]);
        }
    }
}

impl fmt::Display for YongSpec {
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
    fn add_app_with_no_versions() {
        let mut spec = super::YongSpec::load("");
        spec.add_app("java", "java_home", &["bin"]);

        assert_eq!(spec.apps.get("java").unwrap().home_name, "java_home");
        assert_eq!(spec.apps.get("java").unwrap().bins, &["bin".to_string()]);
    }

    #[test]
    fn load_returns_empty_apps_map_when_input_is_empty() {
        let spec = super::YongSpec::load("");

        assert_eq!(spec.apps.len(), 0);
    }

    #[test]
    fn load_returns_spec_with_missing_versions_when_versions_are_missing() {
        let spec = super::YongSpec::load(indoc!(
            r#"[apps.java]
               home_name = "java_home"
               bins = ["bin"]
               "#
        ));

        assert_eq!(spec.apps.get("java").unwrap().home_name, "java_home")
    }

    #[test]
    fn load_returns_spec_with_versions_when_they_exist() {
        let spec = super::YongSpec::load(indoc!(
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

        let spec = super::YongSpec { apps: map };

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

    #[test]
    fn add_version_to_existing_app_that_has_no_existing_versions() {
        let mut spec = super::YongSpec::load("");
        spec.add_app("java", "java_home", &["bin"]);
        spec.upsert_version("java", "11", "/path/to/java/11");

        assert_eq!(
            format!("{}", spec),
            indoc!(
                r#"[apps.java]
                   home_name = "java_home"
                   bins = ["bin"]

                   [[apps.java.versions]]
                   version = "11"
                   home_path = "/path/to/java/11"
                   "#
            )
        )
    }

    #[test]
    fn add_version_to_existing_app_that_has_existing_versions() {
        let mut spec = super::YongSpec::load("");
        spec.add_app("java", "java_home", &["bin"]);
        spec.upsert_version("java", "11", "/path/to/java/11");
        spec.upsert_version("java", "14", "/path/to/java/14");

        assert_eq!(
            format!("{}", spec),
            indoc!(
                r#"[apps.java]
                   home_name = "java_home"
                   bins = ["bin"]

                   [[apps.java.versions]]
                   version = "11"
                   home_path = "/path/to/java/11"

                   [[apps.java.versions]]
                   version = "14"
                   home_path = "/path/to/java/14"
                   "#
            )
        )
    }

    #[test]
    fn add_duplicate_version_overrides_previous_setup() {
        let mut spec = super::YongSpec::load("");
        spec.add_app("node", "node_home", &["."]);
        spec.upsert_version("node", "12.18.2", "/path/to/node/12.18.2");
        spec.upsert_version("node", "12.18.2", "/alt/path/to/node/12.18.2");

        assert_eq!(
            format!("{}", spec),
            indoc!(
                r#"[apps.node]
                   home_name = "node_home"
                   bins = ["."]

                   [[apps.node.versions]]
                   version = "12.18.2"
                   home_path = "/alt/path/to/node/12.18.2"
                   "#
            )
        )
    }
}
