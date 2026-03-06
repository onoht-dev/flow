use crate::storage::Storage;
use anyhow::Result;
use chrono::Utc;

pub fn run(limit: usize, search: Option<String>, repo: Option<String>) -> Result<()> {
    let storage = Storage::default_location()?;
    let mut history = storage.load_history()?;

    // Apply filters
    if let Some(search_term) = &search {
        history.retain(|e| {
            e.context
                .note
                .to_lowercase()
                .contains(&search_term.to_lowercase())
        });
    }

    if let Some(repo_name) = &repo {
        history.retain(|e| {
            e.context
                .repo
                .as_ref()
                .map(|r| r.to_lowercase().contains(&repo_name.to_lowercase()))
                .unwrap_or(false)
        });
    }

    println!("📊 Context History");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    if history.is_empty() {
        if search.is_some() || repo.is_some() {
            println!("No matching history entries found.");
            println!();
            println!("Try adjusting your filter criteria.");
        } else {
            println!("No history yet.");
            println!();
            println!("Complete tasks with 'flow done' to build history.");
        }
        return Ok(());
    }

    // Stats
    let total_entries = history.len();
    let today: Vec<_> = history
        .iter()
        .filter(|e| e.completed_at.date_naive() == Utc::now().date_naive())
        .collect();

    let total_minutes_today: i64 = today.iter().map(|e| e.duration_minutes).sum();
    let hours_today = total_minutes_today / 60;
    let mins_today = total_minutes_today % 60;

    println!(
        "📈 Today: {} tasks, ~{}h {}m tracked",
        today.len(),
        hours_today,
        mins_today
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::storage::HistoryEntry;

    fn create_test_entry(note: &str, repo: Option<&str>, branch: Option<&str>) -> HistoryEntry {
        let mut context = Context::new(note.to_string());
        context.repo = repo.map(|r| r.to_string());
        context.branch = branch.map(|b| b.to_string());

        HistoryEntry {
            context,
            completed_at: Utc::now(),
            duration_minutes: 30,
        }
    }

    #[test]
    fn test_filter_by_search_case_insensitive() {
        let entries = [
            create_test_entry("Fix authentication bug", Some("my-repo"), Some("main")),
            create_test_entry("Add feature for AUTH flow", Some("other-repo"), Some("dev")),
            create_test_entry("Update documentation", Some("my-repo"), Some("docs")),
        ];

        let filtered: Vec<_> = entries
            .iter()
            .filter(|e| {
                e.context
                    .note
                    .to_lowercase()
                    .contains(&"auth".to_lowercase())
            })
            .collect();

        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_by_repo_case_insensitive() {
        let entries = [
            create_test_entry("Task 1", Some("My-Repo"), Some("main")),
            create_test_entry("Task 2", Some("my-repo"), Some("dev")),
            create_test_entry("Task 3", Some("other-repo"), Some("main")),
        ];

        let filtered: Vec<_> = entries
            .iter()
            .filter(|e| {
                e.context
                    .repo
                    .as_ref()
                    .map(|r| r.to_lowercase().contains(&"my-repo".to_lowercase()))
                    .unwrap_or(false)
            })
            .collect();

        assert_eq!(filtered.len(), 2);
    }

    #[test]
    #[allow(clippy::manual_retain)]
    fn test_filter_combined() {
        let entries = [
            create_test_entry("Fix auth bug", Some("my-repo"), Some("main")),
            create_test_entry("Fix auth bug", Some("other-repo"), Some("dev")),
            create_test_entry("Update docs", Some("my-repo"), Some("docs")),
        ];

        let mut filtered: Vec<_> = entries
            .into_iter()
            .filter(|e| {
                e.context
                    .note
                    .to_lowercase()
                    .contains(&"auth".to_lowercase())
            })
            .collect();

        filtered.retain(|e| {
            e.context
                .repo
                .as_ref()
                .map(|r| r.to_lowercase().contains(&"my-repo".to_lowercase()))
                .unwrap_or(false)
        });

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].context.note, "Fix auth bug");
    }
}
