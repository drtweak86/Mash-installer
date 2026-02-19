//! UI components for progress tracking and phase observation

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use installer_core::{PhaseEvent, PhaseObserver};
use std::time::Duration;

pub struct CliPhaseObserver {
    mp: MultiProgress,
    overall: ProgressBar,
    spinner: Option<ProgressBar>,
}

impl CliPhaseObserver {
    pub fn new() -> Self {
        let mp = MultiProgress::new();
        let overall = mp.add(ProgressBar::new(0));
        overall.set_style(
            ProgressStyle::with_template(
                "{spinner:.cyan} [{bar:30.green/dim}] {pos}/{len} phases  {percent}%  ETA {eta}  [{elapsed}]",
            )
            .unwrap()
            .progress_chars("━╸─")
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
        );
        overall.enable_steady_tick(Duration::from_millis(200));

        Self {
            mp,
            overall,
            spinner: None,
        }
    }

    fn finish_spinner(&mut self, prefix: &'static str, msg: &str) {
        if let Some(pb) = self.spinner.take() {
            pb.set_style(ProgressStyle::with_template("{prefix} {msg}").unwrap());
            pb.set_prefix(prefix);
            pb.finish_with_message(msg.to_string());
        }
    }

    fn start_spinner(&mut self, msg: &str) {
        self.spinner = Some(
            self.mp
                .insert_before(&self.overall, ProgressBar::new_spinner()),
        );
        if let Some(pb) = &self.spinner {
            pb.set_style(
                ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {msg}")
                    .unwrap()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
            );
            pb.set_message(msg.to_string());
            pb.enable_steady_tick(Duration::from_millis(120));
        }
    }

    pub fn finish(&mut self) {
        self.finish_spinner(" ", "");
        self.overall.finish_and_clear();
    }
}

impl PhaseObserver for CliPhaseObserver {
    fn on_event(&mut self, event: PhaseEvent) {
        match event {
            PhaseEvent::Total { total } => self.overall.set_length(total as u64),
            PhaseEvent::Started {
                index,
                total,
                phase,
            } => {
                self.finish_spinner(" ", "");
                let display = format!("Phase {}/{} · {}", index, total, phase);
                self.start_spinner(&display);
            }
            PhaseEvent::Completed { description, .. } => {
                self.finish_spinner("✓", &description);
                self.overall.inc(1);
            }
            PhaseEvent::Failed { error, .. } => {
                let message = format!("Phase FAILED: {error}");
                self.finish_spinner("✗", &message);
                self.overall.inc(1);
            }
            PhaseEvent::Skipped { phase, .. } => {
                self.finish_spinner("–", &phase);
                self.overall.inc(1);
            }
        }
    }
}

pub fn print_banner() {
    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║       mash-setup · mega installer            ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();
}
