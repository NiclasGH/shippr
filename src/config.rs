use std::path::PathBuf;

use config::File;
use serde::Deserialize;

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
struct AppConfig {
    chart: String,
}
impl AppConfig {
    pub fn new(base_path: &PathBuf, file_name: &str) -> Result<Self> {
        let directory_string = base_path.as_os_str().to_str().ok_or(
            Error::InvalidDirectory
        )?;
        let config_path = format!("{directory_string}/{file_name}");

        let config = config::Config::builder()
            .add_source(File::with_name(&config_path))
            .build()?
            .try_deserialize()?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::Builder;

    use super::AppConfig;
    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn file_deserialized_correctly() -> TestResult {
        // given
        let mut deployment_file = Builder::new()
            .prefix("deployment")
            .suffix(".yaml")
            .tempfile()?;
        let file_content = r#"
        chart: MyChart
        "#;
        writeln!(&mut deployment_file, "{}", file_content)?;
        let binding = deployment_file.into_temp_path();
        let file_name = binding
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        // when
        let result = AppConfig::new(&std::env::temp_dir(), file_name)?;

        // then
        assert_eq!(result.chart, "MyChart");

        Ok(())
    }
}
