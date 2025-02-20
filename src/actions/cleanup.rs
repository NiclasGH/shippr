use std::path::PathBuf;

use tracing::debug;

pub fn cleanup(namespace: String, dir: PathBuf) {
    debug!("Recieved the following parameters: namespace: [{:?}], dir: [{:?}]", namespace, dir);

    std::process::exit(1);
}
