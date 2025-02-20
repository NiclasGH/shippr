use std::path::PathBuf;
use tracing::{debug, info};
use crate::{command::Command, config::*, Result};

pub fn check(profile: Option<String>, deploy_file_dir: PathBuf) -> Result<()> {
    debug!("Recieved the following parameters: profile: [{:?}], dir: [{:?}]", profile, deploy_file_dir);

    let deployment = Deployment::new(&deploy_file_dir, None)?;
    info!("Deployment file found. Checking deployment");

    debug!("Checking values-default.yaml exists");
    // TODO verify values-default.yaml exists

    if let Some(p) = &profile {
        info!("Profile is set. Loading values-{}.yaml exists", p);
        // TODO verify values-{profile}.yaml exists
    }

    let command = create_check(profile);
    std::process::exit(1);
}

fn create_check(profile: Option<String>) -> Command {
    let mut command = Command::new("helm");
    command
        .args(["upgrade", "--install"])
        .arg("--wait")
        .arg("--dry-run")
        .args(["-f", "values-default.yaml"]);

    if let Some(p) = profile {
        command.args(["-f", &format!("values-{p}.yaml")]);
    }

    command
}

