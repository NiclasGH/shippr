use std::{path::PathBuf, process::Command};
use tracing::{debug, info};
use crate::{config::*, Result};

pub fn check(profile: Option<String>, namespace: Option<String>, dir: PathBuf) -> Result<()> {
    debug!("Recieved the following parameters: profile: [{:?}], namespace: [{:?}], dir: [{:?}]", profile, namespace, dir);
    let deployment = Deployment::new(&dir, None)?;
    info!("Deployment file found. Checking deployment");

    let mut command = Command::new("helm");
    command
        .arg(arg)

    std::process::exit(1);
}
