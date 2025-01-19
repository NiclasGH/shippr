use std::path::PathBuf;

use tracing::debug;

pub fn cleanup(namespace: Option<String>, dir: PathBuf) {
    debug!("Recieved the following parameters: namespace: [{:?}], dir: [{:?}]", namespace, dir);
}
