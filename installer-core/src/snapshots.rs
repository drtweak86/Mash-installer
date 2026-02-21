use anyhow::Result;
use std::process::Command;
use crate::{PhaseContext, cmd, system::SystemOps, system::RealSystem};

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    let fstype = RealSystem.detect_root_fstype()?;
    ctx.record_action(format!("STATION_01: ROOT_FS IDENTIFIED: {}", fstype.to_uppercase()));

    if fstype == "btrfs" {
        if command_exists("timeshift") {
            ctx.run_or_record("Snapshot", "Create Btrfs snapshot via Timeshift", Some("MASH pre-install".into()), |_| {
                let mut cmd = Command::new("sudo");
                cmd.args(["timeshift", "--create", "--comments", "MASH pre-install"]);
                cmd::run(&mut cmd)?;
                Ok(())
            })?;
        } else {
            ctx.record_warning("Btrfs detected but Timeshift is missing. Skipping snapshot.");
        }
    } else {
        ctx.record_action(format!("FS {} does not support native snapshots via this station.", fstype));
    }
    
    Ok(())
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
