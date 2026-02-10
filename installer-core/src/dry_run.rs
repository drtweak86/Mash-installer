use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct DryRunEntry {
    pub phase: String,
    pub action: String,
    pub detail: Option<String>,
}

pub struct DryRunLog {
    entries: RefCell<Vec<DryRunEntry>>,
}

impl DryRunLog {
    pub fn new() -> Self {
        Self {
            entries: RefCell::new(Vec::new()),
        }
    }

    pub fn record(
        &self,
        phase: impl Into<String>,
        action: impl Into<String>,
        detail: Option<String>,
    ) {
        let entry = DryRunEntry {
            phase: phase.into(),
            action: action.into(),
            detail,
        };
        self.entries.borrow_mut().push(entry);
    }

    pub fn entries(&self) -> Vec<DryRunEntry> {
        self.entries.borrow().clone()
    }
}

pub fn print_summary(log: &DryRunLog) {
    let entries = log.entries();
    println!();
    println!("──── Dry-run summary ────────────────────────────");
    if entries.is_empty() {
        println!("  No dry-run actions were recorded.");
    } else {
        for (idx, entry) in entries.iter().enumerate() {
            println!("  {}. [{}] {}", idx + 1, entry.phase, entry.action);
            if let Some(detail) = &entry.detail {
                println!("     {}", detail);
            }
        }
    }
    println!("  No resources were modified during dry run.");
    println!("───────────────────────────────────────────────");
    println!();
}
