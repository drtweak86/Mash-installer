use anyhow::{Context, Result};
use std::error::Error;
use std::fmt;
use std::process::{Command, Output};

/// Runs a command and provides detailed errors when it fails.
pub fn run(cmd: &mut Command) -> Result<Output> {
    let desc = describe_command(cmd);
    let output = cmd
        .output()
        .with_context(|| format!("running command: {desc}"))?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(CommandExecutionError::new(desc, output).into())
    }
}

fn describe_command(cmd: &Command) -> String {
    let mut parts = Vec::new();
    parts.push(cmd.get_program().to_string_lossy().into_owned());
    for arg in cmd.get_args() {
        parts.push(arg.to_string_lossy().into_owned());
    }
    parts.join(" ")
}

#[derive(Debug)]
pub struct CommandExecutionError {
    pub command: String,
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl CommandExecutionError {
    fn new(command: String, output: Output) -> Self {
        Self {
            command,
            status: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}

impl fmt::Display for CommandExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "command '{}' failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
            self.command, self.status, self.stdout, self.stderr
        )
    }
}

impl Error for CommandExecutionError {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn describe_command_includes_arguments() {
        let mut cmd = Command::new("echo");
        cmd.args(["hello", "world"]);
        assert_eq!(describe_command(&cmd), "echo hello world");
    }

    #[test]
    fn run_returns_output_on_success() -> Result<()> {
        let mut cmd = Command::new("echo");
        cmd.arg("ok");
        let output = run(&mut cmd)?;
        assert!(output.status.success());
        Ok(())
    }

    #[test]
    fn run_includes_stdout_and_stderr_on_failure() {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg("echo out; echo err >&2; exit 1");
        let err = run(&mut cmd).unwrap_err();
        let err = err
            .downcast_ref::<CommandExecutionError>()
            .expect("expected command error");
        assert!(err.stdout.contains("out"));
        assert!(err.stderr.contains("err"));
    }
}
