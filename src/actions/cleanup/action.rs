use log::debug;
use std::path::PathBuf;

pub fn cleanup(namespace: String, dir: PathBuf) {
    debug!("Recieved the following parameters: namespace: [{:?}], dir: [{:?}]", namespace, dir);

    std::process::exit(1);
}
