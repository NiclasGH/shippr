use crate::Result;
use crate::actions::cleanup::model::Releases;
use crate::command::Command;
use crate::io::user_confirmation;
use log::debug;
use std::path::PathBuf;

pub fn cleanup(namespace: String, dir: PathBuf, no_verify: bool) -> Result<()> {
    debug!(
        "Received the following parameters: namespace: [{:?}], dir: [{:?}]",
        namespace, dir
    );
    let currently_released = find_currently_released(&namespace)?;
    let defined_releases = find_defined_releases(dir)?;

    let difference = currently_released.difference(&defined_releases);
    if difference.len() == 0 {
        println!("Nothing to cleanup");
        return Ok(())
    }

    let user_confirm = format!(
        "The following would be undeployed: {}: Proceed? [Y/N]",
        difference
    );
    if !no_verify && !user_confirmation(&user_confirm)? {
        return Ok(());
    }
    difference.undeploy(&namespace)?;

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

fn find_currently_released(namespace: &str) -> Result<Releases> {
    let releases = create_list_releases(namespace).output()?;

    releases.parse()
}
fn create_list_releases(namespace: &str) -> Command {
    let mut command = Command::new("helm");
    command
        .arg("list")
        .args(["--namespace", namespace])
        .args(["-o", "yaml"]);

    command
}

#[cfg(test)]
mod tests {
    use crate::actions::cleanup::action::create_list_releases;

    #[test]
    fn create_list_releases_correct_helm_command() {
        //given
        let namespace = "test";

        // when
        let result = create_list_releases(namespace);

        // then
        assert_eq!(result.get_program(), "helm");
        assert_eq!(
            result.get_args(),
            ["list", "--namespace", namespace, "-o", "yaml"]
        );
    }
}
