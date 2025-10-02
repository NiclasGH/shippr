use std::path::PathBuf;

use tracing::{debug, info};

use crate::{Result, command::Command, deploy_config::Deployment, io::user_confirmation};

pub fn undeploy(deploy_file_dir: PathBuf, no_verify: bool) -> Result<()> {
    debug!(
        "Received the following parameters: dir: [{:?}], no-verify: [{:?}]",
        deploy_file_dir, no_verify
    );

    let deployment = Deployment::new(&deploy_file_dir, None)?;
    info!("Deployment file found. Checking deployment");

    let prompt = String::from("Do you really want to undeploy? [Y/N]");
    if !no_verify && !user_confirmation(&prompt)? {
        return Ok(());
    }

    println!("Undeploying chart..");
    create_undeploy(deployment).execute()?;

    Ok(())
}

fn create_undeploy(deployment: Deployment) -> Command {
    let mut command = Command::new("helm");
    command.arg("uninstall");

    deployment.append_undeployment_information(&mut command);

    command
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::deploy_config::test_fixtures::deployment;

    use super::create_undeploy;

    type TestResult = std::result::Result<(), Box<dyn Error>>;

    #[rustfmt::skip]
    #[test]
    fn undeploy() -> TestResult {
        // given
        let deployment = deployment();

        // when
        let result = create_undeploy(deployment);

        // then
        assert_eq!(result.get_program(), "helm");
        assert_eq!(result.get_args(), [
            "uninstall",
            "TestRelease",
            "--namespace", "TestNamespace"
        ]);

        Ok(())
    }
}
