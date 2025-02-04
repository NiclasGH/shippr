use std::path::PathBuf;
use tracing::{debug, info, warn};
use crate::{command::Command, config::*, Result};

pub fn check(profile: Option<String>, namespace: Option<String>, deploy_file_dir: PathBuf) -> Result<()> {
    debug!("Recieved the following parameters: profile: [{:?}], namespace: [{:?}], dir: [{:?}]", profile, namespace, deploy_file_dir);
    let deployment = Deployment::new(&deploy_file_dir, None)?;
    info!("Deployment file found. Checking deployment");

    debug!("Checking values-default.yaml exists");
    // TODO verify values-default.yaml exists

    if let Some(p) = &profile {
        info!("Profile is set. Verifying values-{}.yaml exists", p);
        // TODO verify values-{profile}.yaml exists
    }

    if let Some(_) = &namespace {
        warn!("Namespace through the cli is not yet implemented!");
    }

    let command = create_check(profile);
    std::process::exit(1);
}

fn create_check(profile: Option<String>) -> Command {
    Command::new("helm")
    .args(["upgrade", "--install", )
}

