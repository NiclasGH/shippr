use std::path::PathBuf;
use tracing::{debug, info};

use crate::{command::Command, config::*, Error, Result};

pub fn check(profile: Option<String>, deploy_file_dir: PathBuf) -> Result<()> {
    debug!("Recieved the following parameters: profile: [{:?}], dir: [{:?}]", profile, deploy_file_dir);

    let deployment = Deployment::new(&deploy_file_dir, None)?;
    info!("Deployment file found. Checking deployment");

    debug!("Checking values-default.yaml exists");
    let values_default = deploy_file_dir.clone().join("values-default.yaml");
    if !values_default.exists() {
        return Err(Error::ValuesDefaultMissing(values_default));
    }

    let values_profile = if let Some(p) = &profile {
        let file_name = format!("values-{}.yaml", p);
        info!("Profile is set. Checking {} exists", file_name);
        let values_profile = deploy_file_dir.clone().join(file_name);
        if !values_profile.exists() {
            return Err(Error::ValuesProfileMissing(values_profile));
        }

        Some(values_profile)
    } else { None };

    create_check(deployment, values_default, values_profile)?.execute()?;

    Ok(())
}

fn create_check(
    deployment: Deployment, 
    values_default: PathBuf, 
    values_profile: Option<PathBuf>
) -> Result<Command> {
    let mut command = Command::new("helm");
    command.args(["upgrade", "--install"]);
    deployment.append_deployment_information(&mut command)?;

    command
        .arg("--dry-run")
        .args(["-f", values_default.to_str().unwrap()]);

    if let Some(p) = values_profile {
        command.args(["-f", p.to_str().unwrap()]);
    }

    Ok(command)
}

#[cfg(test)]
mod tests {
    // TODO write a test for create_check with and without profile
}
