use crate::{Result, command::Command};

pub fn list_clusters() -> Result<()> {
    create_list_clusters().execute()?;

    Ok(())
}

fn create_list_clusters() -> Command {
    let mut command = Command::new("kubectl");
    command.arg("config").arg("get-contexts");

    command
}

#[cfg(test)]
mod test {
    use super::create_list_clusters;

    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn correct_kubectl_command() -> TestResult {
        // given

        // when
        let command = create_list_clusters();

        // then
        assert_eq!(command.get_program(), "kubectl");
        assert_eq!(command.get_args(), &["config", "get-contexts"]);

        Ok(())
    }
}
