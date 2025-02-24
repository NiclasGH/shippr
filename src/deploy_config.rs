use std::path::Path;

use config::File;
use serde::Deserialize;
use tracing::{debug, warn};

use crate::{command::Command, Error, Result};

#[derive(Debug, Deserialize)]
pub struct Deployment {
    release: Release,
    chart: DeployChart,
}
#[derive(Debug, Deserialize)]
struct DeployChart {
    name: String,
    version: Option<String>,
    namespace: Option<String>,
    location: Location,
}
#[derive(Debug, Deserialize)]
struct Release {
    //image: String, this is a possible extension. Use values files for now
    release_name: String,
}
#[derive(Debug, Deserialize)]
struct Location {
    repo: Option<String>,
    local: Option<String>,
}

pub struct DeploymentFileName(String);

impl Default for DeploymentFileName {
    fn default() -> Self {
        Self(String::from("deployment"))
    }
}

impl Deployment {
    pub fn new(base_path: &Path, file_name: Option<DeploymentFileName>) -> Result<Self> {
        let directory_string = base_path.as_os_str().to_str().ok_or(
            Error::InvalidDirectory
        )?;

        let file_name = file_name.unwrap_or_else(|| {
            debug!("File name not provided. Taking default");
            DeploymentFileName::default()
        }).0;

        let config_path = format!("{directory_string}/{file_name}");

        let config: Self = config::Config::builder()
            .add_source(File::with_name(&config_path))
            .build()?
            .try_deserialize()?;

        if config.chart.has_duplicate_location() {
            warn!("The charts have a duplicate location. Please ensure your deployment file only has 1 location");
            return Err(Error::DuplicateLocation)
        }

        Ok(config)
    }

    pub fn append_deployment_information(&self, command: &mut Command) {
        self.release.append_release_information(command);
        self.chart.append_chart_information(command);
    }
}

impl Release {
    fn append_release_information(&self, command: &mut Command) {
        command.arg(&self.release_name);
    }
}

impl DeployChart {
    fn has_duplicate_location(&self) -> bool {
        self.location.has_duplicate_location()
    }

    fn append_chart_information(&self, command: &mut Command) {
        command.arg(&self.name);

        if let Some(version) = &self.version {
            command.args(["--version", version]);
        }

        if let Some(namespace) = &self.namespace {
            command.args(["--namespace", namespace]);
            command.arg("--create-namespace");
        }

        self.append_chart_location(command);
    }

    fn append_chart_location(&self, command: &mut Command) {
        if let Some(v) = self.location.local.clone() {
            command.arg(v);
        } else if let Some(v) = self.location.repo.clone() {
            command.args(["--repo", &v]);
        }
    }
}

impl Location {
    fn has_duplicate_location(&self) -> bool {
        self.local.is_some() == self.repo.is_some()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::Builder;
    use std::matches;

    use super::{Deployment, DeploymentFileName};
    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn file_deserialized_correctly() -> TestResult {
        // given
        let mut deployment_file = Builder::new()
            .prefix("deployment")
            .suffix(".yaml")
            .tempfile()?;
        let file_content = r#"
        release:
            release_name: TestRelease

        chart:
            name: TestName
            version: TestVersion
            namespace: TestNamespace
            location:
                repo: TestRepo
        "#;
        writeln!(&mut deployment_file, "{}", file_content)?;
        let binding = deployment_file.into_temp_path();
        let file_name = binding
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        // when
        let result = Deployment::new(&std::env::temp_dir(), Some(DeploymentFileName(file_name.to_string())))?;

        // then
        assert_eq!(result.chart.name, "TestName");
        assert_eq!(result.chart.version, Some(String::from("TestVersion")));
        assert_eq!(result.chart.namespace, Some(String::from("TestNamespace")));
        assert_eq!(result.chart.location.repo, Some(String::from("TestRepo")));
        assert_eq!(result.chart.location.local, None);
        assert_eq!(result.release.release_name, String::from("TestRelease"));

        Ok(())
    }

    #[test]
    fn file_deserialized_duplicate_location_error() -> TestResult {
        // given
        let mut deployment_file = Builder::new()
            .prefix("deployment")
            .suffix(".yaml")
            .tempfile()?;
        let file_content = r#"
        release:
            release_name: TestRelease

        chart:
            name: TestName
            version: TestVersion
            namespace: TestNamespace
            location:
                repo: TestRepo
                local: TestPath
        "#;
        writeln!(&mut deployment_file, "{}", file_content)?;
        let binding = deployment_file.into_temp_path();
        let file_name = binding
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        // when
        let result = Deployment::new(&std::env::temp_dir(), Some(DeploymentFileName(file_name.to_string())));

        // then
        assert!(matches!(result.err(), Some(crate::Error::DuplicateLocation)));

        Ok(())
    }
}

#[cfg(test)]
pub mod test_fixtures {
    use super::{Deployment, Release, DeployChart, Location};

    pub fn deployment() -> Deployment {
        Deployment {
            release: Release {
                release_name: String::from("TestRelease"),
            },
            chart: DeployChart {
                name: String::from("TestChartName"),
                version: Some(String::from("TestVersion")),
                namespace: Some(String::from("TestNamespace")),
                location: Location {
                    repo: Some(String::from("TestRepo")),
                    local: None,
                },
            },
        }
    }
}
