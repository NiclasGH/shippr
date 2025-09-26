use crate::Result;
use crate::actions::cleanup::model::Releases;
use crate::command::Command;
use crate::io::user_confirmation;
use log::debug;
use std::path::PathBuf;

pub fn cleanup_all_namespaces(dir: PathBuf, no_verify: bool) -> Result<()> {
    debug!("Received the following parameters: all-namespace: [true], dir: [{dir:?}]");
    let currently_released = find_currently_released_in_all_namespace()?;
    let defined_releases = find_defined_releases(dir)?;

    let difference = currently_released.difference(&defined_releases);
    if difference.len() == 0 {
        println!("Nothing to cleanup");
        return Ok(());
    }

    let user_confirm = format!("The following would be undeployed: {difference}: Proceed? [Y/N]");
    if !no_verify && !user_confirmation(&user_confirm)? {
        return Ok(());
    }
    difference.undeploy_all_namespaces()?;

    Ok(())
}

pub fn cleanup_namespace(namespace: String, dir: PathBuf, no_verify: bool) -> Result<()> {
    debug!(
        "Received the following parameters: all-namespaces: [false] namespace: [{namespace:?}], dir: [{dir:?}]"
    );
    let currently_released = find_currently_released_in_namespace(&namespace)?;
    let defined_releases = find_defined_releases(dir)?;

    let difference = currently_released.difference(&defined_releases);
    if difference.len() == 0 {
        println!("Nothing to cleanup");
        return Ok(());
    }

    let user_confirm = format!("The following would be undeployed: {difference}: Proceed? [Y/N]");
    if !no_verify && !user_confirmation(&user_confirm)? {
        return Ok(());
    }
    difference.undeploy_namespace(&namespace)?;

    Ok(())
}

fn find_defined_releases(dir: PathBuf) -> Result<Releases> {
    let releases = std::fs::read_dir(dir)?;
    let releases: Vec<_> = releases
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                return entry.file_name().into_string().ok();
            }

            None
        })
        .collect();

    Ok(Releases::new(releases))
}

fn find_currently_released_in_namespace(namespace: &str) -> Result<Releases> {
    let releases = create_list_releases_in_namespace(namespace).output()?;

    releases.parse()
}

fn create_list_releases_in_namespace(namespace: &str) -> Command {
    let mut command = Command::new("helm");
    command
        .arg("list")
        .args(["--namespace", namespace])
        .args(["-o", "yaml"]);

    command
}

fn find_currently_released_in_all_namespace() -> Result<Releases> {
    let releases = create_list_releases_in_all_namespace().output()?;

    releases.parse()
}

fn create_list_releases_in_all_namespace() -> Command {
    let mut command = Command::new("helm");
    command.arg("list").arg("-A").args(["-o", "yaml"]);

    command
}

#[cfg(test)]
mod tests {
    use crate::actions::cleanup::action::create_list_releases_in_all_namespace;
    use crate::actions::cleanup::action::create_list_releases_in_namespace;

    #[test]
    fn create_list_releases_namespace_correct_helm_command() {
        //given
        let namespace = "test";

        // when
        let result = create_list_releases_in_namespace(namespace);

        // then
        assert_eq!(result.get_program(), "helm");
        assert_eq!(
            result.get_args(),
            ["list", "--namespace", namespace, "-o", "yaml"]
        );
    }

    #[test]
    fn create_list_releases_all_namespaces_correct_helm_command() {
        //given

        // when
        let result = create_list_releases_in_all_namespace();

        // then
        assert_eq!(result.get_program(), "helm");
        assert_eq!(result.get_args(), ["list", "-A", "-o", "yaml"]);
    }
}
