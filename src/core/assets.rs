use anyhow::Result;
use std::fs;
use std::path::Path;
use tracing::info;
use walkdir::WalkDir;

use crate::models::SiteConfig;

/// Copy theme assets to output directory
/// Order: shared assets first, then theme-specific assets (theme can override shared)
pub fn copy_theme_assets(config: &SiteConfig) -> Result<()> {
    let output_assets = format!("{}/assets", config.build.output_dir);
    
    // 1. Copy shared assets first (base styles for all themes)
    let shared_assets = "themes/shared";
    if Path::new(shared_assets).exists() {
        copy_dir_recursive(shared_assets, &output_assets)?;
        info!("Copied shared assets to {}", output_assets);
    }
    
    // 2. Copy theme-specific assets (can override shared)
    let theme_assets = format!("themes/{}/assets", config.build.theme);
    if Path::new(&theme_assets).exists() {
        copy_dir_recursive(&theme_assets, &output_assets)?;
        info!("Copied theme assets to {}", output_assets);
    }

    Ok(())
}

/// Copy post-specific assets (images, attachments)
pub fn copy_post_assets<P: AsRef<Path>>(
    post_dir: P,
    slug: &str,
    output_dir: &str,
) -> Result<()> {
    let post_dir = post_dir.as_ref();
    let media_dir = format!("{}/media/{}", output_dir, slug);

    for entry in WalkDir::new(post_dir)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let relative = path.strip_prefix(post_dir).unwrap_or(path);

        // Skip index.md
        if relative.to_string_lossy().contains("index.md") {
            continue;
        }

        // Copy files
        if path.is_file() {
            let dest = Path::new(&media_dir).join(relative);
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &dest)?;
        }
    }

    Ok(())
}

/// Recursively copy a directory
fn copy_dir_recursive<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    fs::create_dir_all(dst)?;

    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let relative = path.strip_prefix(src)?;
        let dest_path = dst.join(relative);

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &dest_path)?;
        }
    }

    Ok(())
}
