use std::path::PathBuf;

use tracing::debug;

pub fn deploy(profile: Option<String>, namespace: Option<String>, dir: PathBuf) {
    debug!("Recieved the following parameters: profile: [{:?}], namespace: [{:?}], dir: [{:?}]", profile, namespace, dir);

    std::process::exit(1);
}
