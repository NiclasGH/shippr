use std::path::PathBuf;
use tracing::{debug, info};

use super::values;
use crate::{Result, command::Command, deploy_config::*};

pub fn check(profile: Option<String>, deploy_file_dir: PathBuf) -> Result<()> {
    debug!(
        "Received the following parameters: profile: [{:?}], dir: [{:?}]",
        profile, deploy_file_dir
    );

    let deployment = Deployment::new(&deploy_file_dir, None)?;
    info!("Deployment file found. Checking deployment");

    let values_default = values::default(deploy_file_dir.clone())?;
    let values_profile = values::profile(deploy_file_dir, &profile)?;

    create_check(deployment, values_default, values_profile).execute()?;

    Ok(())
}

fn create_check(
    deployment: Deployment,
    values_default: PathBuf,
    values_profile: Option<PathBuf>,
) -> Command {
    let mut command = Command::new("helm");
    command
        .args(["upgrade", "--install"])
        .arg("--dry-run")
        .args(["-f", values_default.to_str().unwrap()]);

    if let Some(p) = values_profile {
        command.args(["-f", p.to_str().unwrap()]);
    }
    deployment.append_deployment_information(&mut command);

    command
}

#[cfg(test)]
mod tests {
    use std::{error::Error, path::PathBuf, str::FromStr};

    use crate::deploy_config::test_fixtures::deployment;

    use super::create_check;

    type TestResult = std::result::Result<(), Box<dyn Error>>;

    #[rustfmt::skip]
    #[test]
    fn check_no_profile() -> TestResult {
        // given
        let deployment = deployment();

        let values_default = PathBuf::from_str("values-default.yaml")?;

        // when
        let result = create_check(deployment, values_default, None);

        // then
        assert_eq!(result.get_program(), "helm");
        assert_eq!(result.get_args(), [
            "upgrade", "--install",
            "--dry-run",
            "-f", "values-default.yaml",
            "--version", "TestVersion",
            "--namespace", "TestNamespace",
            "--create-namespace",
            "TestRelease", "TestChartName",
            "--repo", "TestRepo",
        ]);

        Ok(())
    }

    #[rustfmt::skip]
    #[test]
    fn check_with_profile() -> TestResult {
        // given
        let deployment = deployment();

        let values_default = PathBuf::from_str("values-default.yaml")?;
        let values_profile = PathBuf::from_str("values-test.yaml")?;

        // when
        let result = create_check(deployment, values_default, Some(values_profile));

        // then
        assert_eq!(result.get_program(), "helm");
        assert_eq!(result.get_args(), [
            "upgrade", "--install",
            "--dry-run",
            "-f", "values-default.yaml",
            "-f", "values-test.yaml",
            "--version", "TestVersion",
            "--namespace", "TestNamespace",
            "--create-namespace",
            "TestRelease", "TestChartName",
            "--repo", "TestRepo",
        ]);


        Ok(())
    }
}
