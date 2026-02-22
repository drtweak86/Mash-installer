use crate::{cmd, package_manager, PhaseContext};
use anyhow::Result;
use std::process::Command;

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    let selections = &ctx.options.software_plan.selections;

    // Check if any AI agents are selected in the "AI Spirits" category
    let mut selected_agents = Vec::new();
    if let Some(pick) = selections.get("AI Spirits") {
        selected_agents.push(*pick);
    } else {
        // Check all selections just in case
        for (cat, pick) in selections {
            if cat == &"AI Spirits" || *pick == "Claude" || *pick == "Gemini" || *pick == "Vibe" {
                selected_agents.push(*pick);
            }
        }
    }

    if selected_agents.is_empty() {
        return Ok(());
    }

    // Ensure nodejs/npm are present for these tools
    ctx.record_action("Ensuring Node.js environment for AI spirits");
    package_manager::ensure_packages(ctx.platform.driver, &["nodejs", "npm"], ctx.options.dry_run)?;

    for agent in selected_agents {
        match agent {
            "Claude" => install_claude(ctx)?,
            "Gemini" => install_gemini(ctx)?,
            "Vibe" => install_vibe(ctx)?,
            _ => {}
        }
    }

    Ok(())
}

fn install_claude(ctx: &mut PhaseContext) -> Result<()> {
    ctx.run_or_record(
        "AI Spirits",
        "Install Claude Code",
        Some("@anthropic-ai/claude-code".into()),
        |_| {
            let mut cmd = Command::new("sudo");
            cmd.args(["npm", "install", "-g", "@anthropic-ai/claude-code"]);
            cmd::run(&mut cmd)?;
            Ok(())
        },
    )
}

fn install_gemini(ctx: &mut PhaseContext) -> Result<()> {
    ctx.run_or_record(
        "AI Spirits",
        "Install Gemini CLI",
        Some("@google/gemini-cli".into()),
        |_| {
            let mut cmd = Command::new("sudo");
            cmd.args(["npm", "install", "-g", "@google/gemini-cli"]);
            cmd::run(&mut cmd)?;
            Ok(())
        },
    )
}

fn install_vibe(ctx: &mut PhaseContext) -> Result<()> {
    ctx.run_or_record(
        "AI Spirits",
        "Install Mistral Vibe",
        Some("@mistral-ai/vibe".into()),
        |_| {
            let mut cmd = Command::new("sudo");
            cmd.args(["npm", "install", "-g", "@mistral-ai/vibe"]);
            cmd::run(&mut cmd)?;
            Ok(())
        },
    )
}
