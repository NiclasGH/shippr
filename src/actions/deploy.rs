use std::path::PathBuf;
use tracing::{debug, info};

use crate::{command::Command, config::*, io::user_confirmation, Result};
use super::values;

pub fn deploy(profile: Option<String>, deploy_file_dir: PathBuf) -> Result<()> {
    debug!("Recieved the following parameters: profile: [{:?}], dir: [{:?}]", profile, deploy_file_dir);

    let deployment = Deployment::new(&deploy_file_dir, None)?;
    info!("Deployment file found. Checking deployment");

    let values_default = values::default(deploy_file_dir.clone())?;
    let values_profile = values::profile(deploy_file_dir, &profile)?;

    if !user_confirmation("Do you really want to deploy with the following profile: {profile:?}")? {
        return Ok(());
    }

    create_deploy(deployment, values_default, values_profile).execute()?;

    Ok(())
}

fn create_deploy(
    deployment: Deployment, 
    values_default: PathBuf, 
    values_profile: Option<PathBuf>
) -> Command {
    let mut command = Command::new("helm");
    command.args(["upgrade", "--install"]);
    deployment.append_deployment_information(&mut command);

    command
        .args(["-f", values_default.to_str().unwrap()]);

    if let Some(p) = values_profile {
        command.args(["-f", p.to_str().unwrap()]);
    }

    command.arg("--wait");

    command
}

#[cfg(test)]
mod tests {
    use std::{error::Error, path::PathBuf, str::FromStr};

    use crate::config::test_fixtures::deployment;

    use super::create_deploy;

    type TestResult = std::result::Result<(), Box<dyn Error>>;

    #[test]
    fn deploy_no_profile() -> TestResult {
        // given
        let deployment = deployment();

        let values_default = PathBuf::from_str("values-default.yaml")?;

        // when
        let result = create_deploy(deployment, values_default, None);

        // then
        assert_eq!(result.get_program(), "helm");
        assert_eq!(result.get_args(), [
            "upgrade", "--install", "TestRelease", "TestChartName",
            "--version", "TestVersion",
            "--namespace", "TestNamespace",
            "--create-namespace",
            "--repo", "TestRepo",
            "-f", "values-default.yaml",
            "--wait",
        ]);

        Ok(())
    }

    #[test]
    fn deploy_with_profile() -> TestResult {
        // given
        let deployment = deployment();

        let values_default = PathBuf::from_str("values-default.yaml")?;
        let values_profile = PathBuf::from_str("values-test.yaml")?;

        // when
        let result = create_deploy(deployment, values_default, Some(values_profile));

        // then
        assert_eq!(result.get_program(), "helm");
        assert_eq!(result.get_args(), [
            "upgrade", "--install", "TestRelease", "TestChartName",
            "--version", "TestVersion",
            "--namespace", "TestNamespace",
            "--create-namespace",
            "--repo", "TestRepo",
            "-f", "values-default.yaml",
            "-f", "values-test.yaml",
            "--wait",
        ]);

        Ok(())
    }
}
