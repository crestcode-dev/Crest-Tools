use anyhow::{bail, Context, Result};
use std::env;
use std::path::Path;
use std::process::{Command, ExitStatus, Output, Stdio};

fn command_exists(cmd: &str) -> bool {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let full_path = Path::new(&path).join(cmd);
            if full_path.is_file() {
                return true;
            }
        }
    }
    false
}

#[derive(Debug, Clone)]
pub enum ApkBackend {
    Native,
    ProotDistro(String),
}

impl ApkBackend {
    pub fn detect() -> Result<Self> {
        // 1. Check if native apk exists in PATH
        if command_exists("apk") {
            return Ok(ApkBackend::Native);
        }

        // 2. Fallback check for proot-distro alpine
        if command_exists("proot-distro") {
            return Ok(ApkBackend::ProotDistro("alpine".to_string()));
        }

        bail!("Neither 'apk' nor 'proot-distro' found! Ensure you are in Alpine Linux or have proot-distro installed.");
    }

    pub fn prepare_cmd(&self, args: &[&str]) -> Command {
        match self {
            ApkBackend::Native => {
                let mut cmd = Command::new("apk");
                cmd.args(args);
                cmd
            }
            ApkBackend::ProotDistro(container) => {
                let mut cmd = Command::new("proot-distro");
                cmd.args(["login", container.as_str(), "--", "apk"]);
                cmd.args(args);
                cmd
            }
        }
    }

    pub fn execute_output(&self, args: &[&str]) -> Result<Output> {
        let mut cmd = self.prepare_cmd(args);
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        let output = cmd.output().context("Failed to execute apk command")?;
        Ok(output)
    }

    pub fn execute_interactive(&self, args: &[&str]) -> Result<ExitStatus> {
        let mut cmd = self.prepare_cmd(args);
        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        let status = cmd.status().context("Failed to run apk command")?;
        Ok(status)
    }
}
