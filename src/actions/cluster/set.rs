use crate::{Result, command::Command};

use tracing::debug;

pub fn set_cluster(name: &str) -> Result<()> {
    debug!("Received the following parameters: name: [{:?}]", name);
    create_set_cluster(name).execute()?;

    Ok(())
}

fn create_set_cluster(name: &str) -> Command {
    let mut command = Command::new("kubectl");
    command.arg("config").arg("use-context").arg(name);

    command
}

#[cfg(test)]
mod test {
    use super::create_set_cluster;

    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn uses_name_for_kubectl_cmd() -> TestResult {
        // given
        let name = "testName";

        // when
        let command = create_set_cluster(name);

        // then
        assert_eq!(command.get_program(), "kubectl");
        assert_eq!(command.get_args(), &["config", "use-context", "testName"]);

        Ok(())
    }
}
