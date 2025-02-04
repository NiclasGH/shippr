use std::path::PathBuf;
use tracing::{debug, info};
use crate::{command::Command, config::*, Result};

pub fn check(profile: Option<String>, namespace: Option<String>, dir: PathBuf) -> Result<()> {
    debug!("Recieved the following parameters: profile: [{:?}], namespace: [{:?}], dir: [{:?}]", profile, namespace, dir);
    let deployment = Deployment::new(&dir, None)?;
    info!("Deployment file found. Checking deployment");

    std::process::exit(1);
}

