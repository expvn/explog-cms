use anyhow::Result;
use std::fs;
use tracing::info;

/// Run the clean command
pub fn run() -> Result<()> {
    // Remove public directory
    if fs::metadata("public").is_ok() {
        fs::remove_dir_all("public")?;
        info!("Removed public/ directory");
    }

    // Remove cache directory
    if fs::metadata(".cache").is_ok() {
        fs::remove_dir_all(".cache")?;
        info!("Removed .cache/ directory");
    }

    info!("Clean complete!");
    Ok(())
}
