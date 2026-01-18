use anyhow::Result;
use axum::Router;
use notify::{Event, RecursiveMode, Watcher};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use tokio::sync::broadcast;
use tower_http::services::ServeDir;
use tracing::info;

use crate::cli::build;

/// Debounce duration in milliseconds
const DEBOUNCE_MS: u64 = 300;

/// Run the development server with hot reload
pub async fn run(port: u16) -> Result<()> {
    // Initial build
    build::run(false, None, None, None, None).await?;

    // Setup file watcher
    let (tx, _rx) = broadcast::channel::<String>(16);
    let tx_clone = tx.clone();

    // Watch for file changes in content, themes, and config
    std::thread::spawn(move || {
        let (file_tx, file_rx) = channel::<Result<Event, notify::Error>>();
        let mut watcher = notify::recommended_watcher(file_tx).unwrap();

        // Watch directories
        watcher
            .watch(Path::new("content"), RecursiveMode::Recursive)
            .ok();
        watcher
            .watch(Path::new("themes"), RecursiveMode::Recursive)
            .ok();
        
        // Watch config file
        watcher
            .watch(Path::new("explog.toml"), RecursiveMode::NonRecursive)
            .ok();

        info!("Watching: content/, themes/, explog.toml");

        let mut last_event = Instant::now();
        
        loop {
            if let Ok(Ok(event)) = file_rx.recv() {
                // Debounce: skip if too soon after last event
                if last_event.elapsed() < Duration::from_millis(DEBOUNCE_MS) {
                    continue;
                }
                last_event = Instant::now();
                
                // Get the file that triggered the change
                let file_path = event.paths
                    .first()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                info!("Change detected: {}", file_path);
                let _ = tx_clone.send(file_path);
            }
        }
    });

    // Spawn rebuild task
    let tx_rebuild = tx.clone();
    tokio::spawn(async move {
        let mut rx = tx_rebuild.subscribe();
        loop {
            if let Ok(file) = rx.recv().await {
                info!("Rebuilding... (triggered by {})", file);
                match build::run(false, None, None, None, None).await {
                    Ok(_) => info!("Rebuild complete!"),
                    Err(e) => tracing::error!("Rebuild failed: {}", e),
                }
            }
        }
    });

    // Serve static files
    let app = Router::new().fallback_service(ServeDir::new("public"));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Dev server: http://localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

