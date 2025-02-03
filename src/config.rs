use std::{path::PathBuf, process::Command};

use config::File;
use serde::Deserialize;
use tracing::{debug, warn};

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct Deployment {
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
    pub fn new(base_path: &PathBuf, file_name: Option<DeploymentFileName>) -> Result<Self> {
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

        Ok(config)
    }

    pub fn append_chart_location(&self, command: &mut Command) -> Result<()> {
        if self.chart.has_duplicate_location() {
            warn!("The charts have a duplicate location. Please ensure your deployment file only has 1 location");
            return Err(Error::DuplicateLocation)
        }

        self.chart.append_chart_location(command);

        Ok(())
    }
}

impl DeployChart {
    fn has_duplicate_location(&self) -> bool {
        self.location.has_duplicate_location()
    }

    fn append_chart_location(&self, command: &mut Command) {
        if let Some(v) = self.location.local.clone() {
            command.arg(v);
        } else if let Some(v) = self.location.repo.clone() {
            command.arg(format!("--repo {v}"));
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
        let result = Deployment::new(&std::env::temp_dir(), Some(DeploymentFileName(file_name.to_string())))?;

        // then
        assert_eq!(result.chart.name, "TestName");
        assert_eq!(result.chart.version, Some(String::from("TestVersion")));
        assert_eq!(result.chart.namespace, Some(String::from("TestNamespace")));
        assert_eq!(result.chart.location.repo, Some(String::from("TestRepo")));
        assert_eq!(result.chart.location.local, Some(String::from("TestPath")));

        Ok(())
    }
}
