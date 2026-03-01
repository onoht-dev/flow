use crate::storage::Storage;
use anyhow::Result;

pub fn run() -> Result<()> {
    let storage = Storage::default_location()?;

    match storage.load_context()? {
        None => {
            println!("No current context to complete.");
        }
        Some(context) => {
            // Archive to history before clearing
            storage.append_to_history(&context)?;

            println!("✅ Task completed: \"{}\"", context.note);
            storage.clear_context()?;
            println!();
            println!("Context archived to history. Ready for a new task!");
            println!("Use 'flow note \"...\"' to start tracking a new context.");
        }
    }

    Ok(())
}
