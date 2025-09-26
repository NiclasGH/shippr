use tracing::{debug, info};

use crate::{Error, Result};
use std::{fs, io::Write};

pub fn initialize_configuration(deployment_name: String) -> Result<()> {
    debug!("Received the following parameters: name: [{deployment_name:?}");

    create_dir(&deployment_name)?;
    create_file(&deployment_name, "values-default.yaml", None)?;

    let default_deployment_file = create_default_deployment(&deployment_name);
    create_file(
        &deployment_name,
        "deployment.yaml",
        Some(&default_deployment_file),
    )?;

    println!("Created deployment files for {deployment_name}");
    println!("Please configure the namespace, version, location and values before deploying");

    Ok(())
}

fn create_dir(name: &str) -> Result<()> {
    if name.contains("/") {
        return Err(Error::ReleaseNameIsPath);
    }

    fs::create_dir(name)?;
    info!("Created directory: {name}");

    Ok(())
}

fn create_file(
    directory_name: &str,
    file_name: &str,
    content_optional: Option<&str>,
) -> Result<()> {
    if directory_name.contains("/") {
        return Err(Error::ReleaseNameIsPath);
    }

    let file_path = format!("{directory_name}/{file_name}");
    let mut file = fs::File::create(&file_path)?;
    if let Some(content) = content_optional {
        file.write_all(content.as_bytes())?;
    }
    info!("Created file: {file_path}");

    Ok(())
}

fn create_default_deployment(deployment_name: &str) -> String {
    format!(
        r#"name: {deployment_name}
version: 1.0
namespace: default
location: # TODO replace with actual location
  # repo: https://artifacthub.io/{deployment_name}
  # local: /home/root/charts/{deployment_name}"#
    )
}
