use std::str::FromStr;

use yaml_rust2::{Yaml, YamlLoader};

use crate::Error;

pub(super) struct Releases {
    content: Vec<Release>,
}
pub(super) type Release = String;


impl Default for Releases {
    fn default() -> Self {
        Self { content: Vec::new(), }
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
                let content = releases
                    .iter()
                    .filter_map(find_release_name)
                    .collect();
                Ok(Self { content })
            },
            _ => Ok(Self::default())
        }
    }
}

fn find_release_name(release: &Yaml) -> Option<String> {
    match release {
        Yaml::Hash(hash) => hash.get(&Yaml::String("name".to_string()))
            .and_then(|name| {
                if let Yaml::String(name_str) = name {
                    Some(name_str.to_string())
                } else {
                    None
                }
            }),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::cleanup::model::Releases;
    use crate::Error;

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
        assert_eq!(content[0], "test1");
        assert_eq!(content[1], "test2");
        assert_eq!(content[2], "test3");

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
        assert!(matches!(result.err().unwrap(), Error::CouldNotFigureOutReleaseName));
    }

    mod helper {
        pub fn create_yaml() -> String {
            "
        - app_version:
          chart: nginx-1.0.0
          name: test1
          namespace: manual
          revision: '1'
          status: deployed
          updated: 2025-02-24 14:29:36.687922 +0100 CET
        - app_version:
          chart: nginx-1.0.0
          name: test2
          namespace: doesnt matter
          revision: '2'
          status: deployed
          updated: 2025-02-24 14:29:36.687922 +0100 CET
        - app_version:
          chart: nginx-1.0.0
          name: test3
          namespace: manual
          revision: '3'
          status: deployed
          updated: 2025-02-24 14:29:36.687922 +0100 CET
            ".to_string()
        }
    }
}