//! UI components for progress tracking and phase observation

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use installer_core::{PhaseEvent, PhaseObserver};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct CliPhaseObserver {
    mp: MultiProgress,
    overall: ProgressBar,
    spinner: Option<ProgressBar>,
    message_updater: Option<Arc<Mutex<bool>>>, // Signal to stop message updates
}

impl CliPhaseObserver {
    pub fn new() -> Self {
        let mp = MultiProgress::new();
        let overall = mp.add(ProgressBar::new(0));

        // Use consistent single-width characters (emoji caused terminal width panics)
        overall.set_style(
            ProgressStyle::with_template(
                "{spinner:.cyan} [{bar:30}] {pos}/{len} phases  {percent}%  elapsed: {elapsed_precise}",
            )
            .unwrap()
            .progress_chars("━━╾─") // Single-width box-drawing chars
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
        );
        overall.enable_steady_tick(Duration::from_millis(200));

        Self {
            mp,
            overall,
            spinner: None,
            message_updater: None,
        }
    }

    fn finish_spinner(&mut self, prefix: &'static str, msg: &str) {
        // Signal message updater thread to stop
        if let Some(updater) = self.message_updater.take() {
            if let Ok(mut stop) = updater.lock() {
                *stop = true;
            }
        }

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
            pb.enable_steady_tick(Duration::from_millis(120));

            // Start rotating funny messages for known slow phases
            if let Some(messages) = get_funny_messages(msg) {
                let stop_flag = Arc::new(Mutex::new(false));
                self.message_updater = Some(stop_flag.clone());
                let pb_clone = pb.clone();

                thread::spawn(move || {
                    let mut idx = 0;
                    loop {
                        // Check if we should stop
                        if let Ok(should_stop) = stop_flag.lock() {
                            if *should_stop {
                                break;
                            }
                        }

                        pb_clone.set_message(messages[idx].to_string());
                        idx = (idx + 1) % messages.len();

                        thread::sleep(Duration::from_secs(3));
                    }
                });
            } else {
                // No funny messages, just show the phase as-is
                pb.set_message(msg.to_string());
            }
        }
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
            PhaseEvent::Warning { message } => {
                self.mp.suspend(|| {
                    eprintln!();
                    eprintln!("WARNING: {message}");
                    eprintln!();
                });
            }
        }
    }

    fn confirm(&mut self, prompt: &str) -> bool {
        self.mp.suspend(|| {
            eprint!("{prompt} ");
            let _ = io::stderr().flush();
            let mut response = String::new();
            if io::stdin().read_line(&mut response).is_err() {
                return false;
            }
            let response = response.trim().to_lowercase();
            response == "y" || response == "yes"
        })
    }

    fn sudo_password(&mut self) -> anyhow::Result<String> {
        self.mp
            .suspend(|| read_password_crossterm("Enter sudo password: "))
    }
}

fn read_password_crossterm(prompt: &str) -> anyhow::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    enable_raw_mode()?;
    let mut password = String::new();
    let res = (|| -> anyhow::Result<String> {
        loop {
            if let Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                ..
            }) = event::read()?
            {
                if kind != KeyEventKind::Press {
                    continue;
                }
                match code {
                    KeyCode::Enter => break,
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                        anyhow::bail!("Interrupted by user");
                    }
                    KeyCode::Char(c) => password.push(c),
                    KeyCode::Backspace => {
                        password.pop();
                    }
                    KeyCode::Esc => {
                        password.clear();
                        break;
                    }
                    _ => {}
                }
            }
        }
        Ok(password)
    })();

    let _ = disable_raw_mode();
    println!();
    res
}

pub fn print_banner() {
    println!(
        r#"
  __  __    _    ____  _   _ 
 |  \/  |  / \  / ___|| | | |
 | |\/| | / _ \ \___ \| |_| |
 | |  | |/ ___ \ ___) |  _  |
 |_|  |_/_/   \_\____/|_| |_|
"#
    );
    println!(" ╔══════════════════════════════════════════════╗");
    println!(" ║       mash-setup · mega installer            ║");
    println!(" ╚══════════════════════════════════════════════╝");
    println!();
}

/// Get rotating funny messages for known slow phases
fn get_funny_messages(msg: &str) -> Option<Vec<String>> {
    let lower = msg.to_lowercase();
    let base = msg.to_string();

    if lower.contains("rust") && lower.contains("toolchain") {
        Some(vec![
            format!(
                "{} · compiling the compiler that compiles compilers 🦀",
                base
            ),
            format!("{} · teaching crabs to code 🦀", base),
            format!("{} · still faster than npm install ⚡", base),
            format!("{} · rustup is doing rust things 🔧", base),
            format!("{} · adding memory safety to your life ✨", base),
            format!("{} · borrowing time from the borrow checker 📚", base),
        ])
    } else if lower.contains("docker") {
        Some(vec![
            format!("{} · containerizing all the things 📦", base),
            format!("{} · docker-ception in progress 🐋", base),
            format!("{} · installing whale technology 🐳", base),
            format!("{} · because it works on my container 🎯", base),
            format!(
                "{} · downloading the entire internet (jk, just docker) 🌐",
                base
            ),
        ])
    } else if lower.contains("buildroot") {
        Some(vec![
            format!("{} · building roots and taking names 🌱", base),
            format!("{} · cross-compiling your dreams ⚙️", base),
            format!("{} · embedded systems go brrrr 🚀", base),
            format!("{} · making tiny linux distributions 🐧", base),
        ])
    } else if lower.contains("system packages") || lower.contains("pkg") {
        Some(vec![
            format!("{} · apt-get install coffee ☕", base),
            format!("{} · downloading the dependencies of dependencies 📦", base),
            format!("{} · pacman is eating dots... wait, wrong pacman 👾", base),
            format!("{} · just resolving some dependencies, nbd 🔄", base),
            format!("{} · installing 1000 ways to open a text editor 📝", base),
        ])
    } else if lower.contains("git") || lower.contains("github") {
        Some(vec![
            format!("{} · git gud 🎮", base),
            format!("{} · cloning like Dolly the sheep 🐑", base),
            format!("{} · octocats incoming 🐙", base),
            format!("{} · distributed version controlling your life 🌿", base),
        ])
    } else if lower.contains("font") {
        Some(vec![
            format!("{} · making text look pretty ✨", base),
            format!("{} · Comic Sans NOT included (you're welcome) 😌", base),
            format!("{} · installing all the ligatures →→→ 🎨", base),
        ])
    } else {
        None // No funny messages for this phase, will show default
    }
}
