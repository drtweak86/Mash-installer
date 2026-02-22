use crate::cmd;
use anyhow::{anyhow, Context, Result};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::path::Path;
use std::process::{Command, Output};
use std::time::Duration;

/// Abstraction over system APIs that are hard to test directly.
pub trait SystemOps {
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn command_output(&self, cmd: &mut Command) -> Result<Output>;
    fn connect(&self, host: &str, port: u16, timeout: Duration) -> Result<TcpStream>;
    fn detect_root_fstype(&self) -> Result<String>;
}

/// Real implementation of `SystemOps` that delegates to the OS.
pub struct RealSystem;

impl SystemOps for RealSystem {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
    }

    fn command_output(&self, cmd: &mut Command) -> Result<Output> {
        cmd::run(cmd)
    }

    fn connect(&self, host: &str, port: u16, timeout: Duration) -> Result<TcpStream> {
        let mut addrs: Vec<SocketAddr> = (host, port)
            .to_socket_addrs()
            .with_context(|| format!("resolving address {}:{}", host, port))?
            .collect();
        let addr = addrs
            .pop()
            .ok_or_else(|| anyhow!("no socket address available for {host}:{port}"))?;
        TcpStream::connect_timeout(&addr, timeout)
            .with_context(|| format!("connecting to {}:{}", host, port))
    }

    fn detect_root_fstype(&self) -> Result<String> {
        let mut cmd = Command::new("findmnt");
        cmd.args(["-n", "-o", "FSTYPE", "/"]);
        let output = cmd::run(&mut cmd)?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}
