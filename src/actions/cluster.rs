use tracing::debug;

pub fn set_cluster(name: String) {
    debug!("Recieved the following parameters: name: [{:?}]", name);
}
