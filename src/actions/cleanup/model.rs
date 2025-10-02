use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::Error;
use crate::command::Command;
use yaml_rust2::{Yaml, YamlLoader};

#[derive(Default, Debug)]
pub(super) struct Releases {
    content: Vec<Release>,
}
pub(super) type Release = (Name, Namespace);
pub(super) type Name = String;
pub(super) type Namespace = String;

impl Releases {
    pub(super) fn new(content: Vec<Release>) -> Self {
        Releases { content }
    }

    pub(super) fn difference(self, other: &Releases) -> Self {
        let difference = self
            .content
            .into_iter()
            .filter(|r| !other.content.contains(r))
            .collect();

        Self {
            content: difference,
        }
    }

    pub(super) fn len(&self) -> usize {
        self.content.len()
    }

    pub(super) fn undeploy(self) -> Result<(), Error> {
        for release in &self.content {
            println!("Undeploying {} in namespace {}", release.0, release.1);
            create_undeploy(release).execute()?;
        }
        Ok(())
    }
}

impl Display for Releases {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let releases_list: Vec<String> = self
            .content
            .iter()
            .map(|r| format!("Name: {}, Namespace: {}", r.0, r.1))
            .collect();
        write!(f, "{}", releases_list.join(" ; "))
    }
}

impl FromStr for Releases {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let docs = YamlLoader::load_from_str(s).map_err(|_| Error::CouldNotFigureOutReleaseName)?;

        if docs.is_empty() {
            return Ok(Self::default());
        }

        let yaml = &docs[0];

        match yaml {
            Yaml::Array(releases) => {
                let content = releases.iter().filter_map(find_release_info).collect();
                Ok(Self { content })
            }
            _ => Ok(Self::default()),
        }
    }
}

fn create_undeploy(release: &Release) -> Command {
    let mut command = Command::new("helm");
    command.args(["uninstall", &release.0]);
    command.args(["--namespace", &release.1]);
    command
}

fn find_release_info(release: &Yaml) -> Option<(Name, Namespace)> {
    match release {
        Yaml::Hash(hash) => {
            let name: Name = hash
                .get(&Yaml::String("name".to_string()))
                .and_then(|name| {
                    if let Yaml::String(name_str) = name {
                        Some(name_str.to_string())
                    } else {
                        None
                    }
                })?;

            let namespace: Namespace =
                hash.get(&Yaml::String("namespace".to_string()))
                    .and_then(|namespace| {
                        if let Yaml::String(namespace_str) = namespace {
                            Some(namespace_str.to_string())
                        } else {
                            None
                        }
                    })?;

            Some((name, namespace))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::actions::cleanup::model::Releases;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse_correctly() -> TestResult {
        // given
        let yaml = helper::create_yaml();

        // when
        let result: Releases = yaml.parse()?;

        // then
        assert_eq!(result.content.len(), 3);
        let content = result.content;
        assert_eq!(content[0], ("test1".to_string(), "namespace1".to_string()));
        assert_eq!(content[1], ("test2".to_string(), "namespace2".to_string()));
        assert_eq!(content[2], ("test3".to_string(), "namespace3".to_string()));

        Ok(())
    }

    #[test]
    fn parse_empty_yaml_ok() -> TestResult {
        // given
        let yaml = "";

        // when
        let result: Releases = yaml.parse()?;

        // then
        assert_eq!(result.content.len(), 0);

        Ok(())
    }

    #[test]
    fn parse_wrong_file_format_err() {
        // given
        let toml = "
            [my-toml]
            name = test2
            chart = my-test-chart
        ";

        // when
        let result: Result<Releases, Error> = toml.parse();

        // then
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            Error::CouldNotFigureOutReleaseName
        ));
    }

    mod helper {
        pub fn create_yaml() -> String {
            "
        - app_version:
          chart: nginx-1.0.0
          name: test1
          namespace: namespace1
          revision: '1'
          status: deployed
          updated: 2025-02-24 14:29:36.687922 +0100 CET
        - app_version:
          chart: nginx-1.0.0
          name: test2
          namespace: namespace2
          revision: '2'
          status: deployed
          updated: 2025-02-24 14:29:36.687922 +0100 CET
        - app_version:
          chart: nginx-1.0.0
          name: test3
          namespace: namespace3
          revision: '3'
          status: deployed
          updated: 2025-02-24 14:29:36.687922 +0100 CET
            "
            .to_string()
        }
    }
}
