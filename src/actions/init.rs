use tracing::{debug, info};

use crate::{Error, Result};
use std::fs;

pub fn initialize_configuration(name: String) -> Result<()> {
    debug!("Received the following parameters: name: [{name:?}");

    create_dir(&name)?;

    todo!("Unimplemented");
    // TODO create deployment file
    // TODO create empty values-default.yaml

    Ok(())
}

fn create_dir(name: &str) -> Result<()> {
    if name.contains("/") {
        return Err(Error::ReleaseNameIsPath);
    }

    fs::create_dir(name)?;
    info!("Created directory");

    Ok(())
}
