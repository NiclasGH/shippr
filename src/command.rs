use std::{ffi::OsStr, io::{stderr, stdout, Write}, process};

use tracing::info;

use crate::Result;

pub struct Command {
    command: process::Command,
}

impl Command {
    pub(crate) fn new(command: &str) -> Self {
        Self { command: process::Command::new(command) }
    }
    
    /// The advantage of AsRef vs &str
    /// AsRef: The callee can use &str or String and the method takes the ref when needed
    /// &str: The callee must borrow his String if he has one
    /// String: The callee must convert the &str to String himself
    /// Into<String>: The callee can pass &str or String
    pub(crate) fn arg<A: AsRef<OsStr>>(&mut self, arg: A) -> &mut Self {
        self.command.arg(arg);

        self
    }

    pub(crate) fn args<I, A>(&mut self, arg: I) -> &mut Self
    where 
        I: IntoIterator<Item = A>,
        A: AsRef<OsStr>
    {
        self.command.args(arg);

        self
    }

    /// No-value return
    pub fn execute(mut self) -> Result<()> {
        info!("Running command {:?}", self.command.get_program());
        info!("Command args: {:?}", self.command.get_args());

        let output = self.command.output()?;
        stdout().write_all(&output.stdout)?;
        stderr().write_all(&output.stderr)?;

        Ok(())
    }

    /// stdout return
    pub fn output(mut self) -> Result<String> {
        info!("Running command {:?}", self.command.get_program());
        info!("Command args: {:?}", self.command.get_args());

        let output = self.command.output()?;
        stderr().write_all(&output.stderr)?;

        Ok(String::from_utf8(output.stdout)?)
    }
}

#[cfg(test)]
impl Command {
    pub(crate) fn get_program(&self) -> &OsStr {
        self.command.get_program()
    }

    pub(crate) fn get_args(&self) -> Vec<&OsStr> {
        self.command.get_args().collect()
    }
}

