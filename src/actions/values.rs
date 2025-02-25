use crate::{Result, Error};
use std::path::PathBuf;
use tracing::{debug, info};

pub fn default(base_path: PathBuf) -> Result<PathBuf> {
    debug!("Checking values-default.yaml exists");
    let values_default = base_path.join("values-default.yaml");
    if !values_default.exists() {
        return Err(Error::ValuesDefaultMissing(values_default));
    }

    Ok(values_default)
}

pub fn profile(base_path: PathBuf, profile: &Option<String>) -> Result<Option<PathBuf>> {
    debug!("Checking if profile is set");
    if let Some(p) = profile {
        let file_name = format!("values-{}.yaml", p);
        info!("Profile is set. Checking {} exists", file_name);
        let values_profile = base_path.join(file_name);
        if !values_profile.exists() {
            return Err(Error::ValuesProfileMissing(values_profile));
        }

        Ok(Some(values_profile))
    } else { Ok(None) }
}


