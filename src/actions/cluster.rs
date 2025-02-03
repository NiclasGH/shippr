use std::{io::{self, Write}, process::Command};

use crate::Result;

use tracing::{debug, info};

pub fn set_cluster(name: String) -> Result<()> {
    debug!("Recieved the following parameters: name: [{:?}]", name);

    let mut command = Command::new("kubectl");
    command
        .arg("config")
        .arg("use-context")
        .arg(&name);

    let program = command.get_program();
    info!("Running command {:?}", program);

    let output = command.output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use assert_cmd::Command;

    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn uses_name_for_kubectl_cmd() -> TestResult {
        // when
        let assert = Command::cargo_bin("shippr")?
            .arg("cluster")
            .arg("test-cluster")
            .assert();
        
        // then
        assert.success();

        Ok(())
    }
}
