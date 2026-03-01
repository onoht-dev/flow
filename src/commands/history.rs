use crate::storage::Storage;
use anyhow::Result;
use chrono::Utc;

pub fn run(limit: usize) -> Result<()> {
    let storage = Storage::default_location()?;
    let history = storage.load_history()?;

    println!("📊 Context History");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    if history.is_empty() {
        println!("No history yet.");
        println!();
        println!("Complete tasks with 'flow done' to build history.");
        return Ok(());
    }

    // Stats
    let total_entries = history.len();
    let today: Vec<_> = history
        .iter()
        .filter(|e| {
            e.completed_at.date_naive() == Utc::now().date_naive()
        })
        .collect();

    let total_minutes_today: i64 = today.iter().map(|e| e.duration_minutes).sum();
    let hours_today = total_minutes_today / 60;
    let mins_today = total_minutes_today % 60;

    println!("📈 Today: {} tasks, ~{}h {}m tracked", today.len(), hours_today, mins_today);
    println!("📚 Total: {} completed tasks", total_entries);
    println!();

    // Show entries
    let display_count = limit.min(history.len());
    println!("Recent {} entries:", display_count);
    println!();

    for entry in history.iter().take(display_count) {
        let time_ago = format_time_ago(&entry.completed_at);
        let duration = format_duration(entry.duration_minutes);

        println!("• {} [{}]", entry.context.note, duration);
        if let (Some(repo), Some(branch)) = (&entry.context.repo, &entry.context.branch) {
            println!("  └ {} ({}) • {}", repo, branch, time_ago);
        } else {
            println!("  └ {}", time_ago);
        }
        println!();
    }

    Ok(())
}

fn format_time_ago(dt: &chrono::DateTime<Utc>) -> String {
    let duration = Utc::now().signed_duration_since(*dt);

    if duration.num_minutes() < 1 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else {
        format!("{}d ago", duration.num_days())
    }
}

fn format_duration(minutes: i64) -> String {
    if minutes < 1 {
        "<1m".to_string()
    } else if minutes < 60 {
        format!("{}m", minutes)
    } else {
        let hours = minutes / 60;
        let mins = minutes % 60;
        if mins == 0 {
            format!("{}h", hours)
        } else {
            format!("{}h{}m", hours, mins)
        }
    }
}
