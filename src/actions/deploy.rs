use std::path::PathBuf;

use tracing::debug;

pub fn deploy(profile: Option<String>, dir: PathBuf) {
    debug!("Recieved the following parameters: profile: [{:?}], dir: [{:?}]", profile, dir);

    std::process::exit(1);
}
