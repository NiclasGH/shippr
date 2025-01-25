use std::{io::{self, Write}, process::Command};

use crate::Result;

use tracing::debug;

pub fn set_cluster(name: String) -> Result<()> {
    debug!("Recieved the following parameters: name: [{:?}]", name);

    let output = Command::new("kubectl")
        .args(["config", "use-context", &name])
        .output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(())
}
