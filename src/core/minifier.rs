//! CSS and JavaScript minification module
//!
//! Provides functions to minify CSS and JS assets for production builds.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use tracing::info;
use walkdir::WalkDir;

/// Minify CSS content by removing whitespace and comments
pub fn minify_css(content: &str) -> Result<String> {
    use minifier::css::minify;
    minify(content)
        .map(|m| m.to_string())
        .map_err(|e| anyhow::anyhow!("CSS minification failed: {}", e))
}

/// Minify JavaScript content by removing whitespace and comments
pub fn minify_js(content: &str) -> Result<String> {
    use minifier::js::minify;
    Ok(minify(content).to_string())
}

/// Copy and optionally minify theme assets to output directory
pub fn copy_and_minify_assets(
    theme_dir: &str,
    output_dir: &str,
    minify_enabled: bool,
) -> Result<()> {
    let assets_dir = Path::new(theme_dir).join("assets");
    let output_assets = Path::new(output_dir).join("assets");

    if !assets_dir.exists() {
        return Ok(());
    }

    fs::create_dir_all(&output_assets)?;

    for entry in WalkDir::new(&assets_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let relative = path.strip_prefix(&assets_dir)?;
        let dest_path = output_assets.join(relative);

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else if path.is_file() {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            if minify_enabled {
                match extension {
                    "css" => {
                        let content = fs::read_to_string(path)
                            .with_context(|| format!("Failed to read CSS: {}", path.display()))?;
                        let minified = minify_css(&content)?;
                        fs::write(&dest_path, minified)?;
                        info!("Minified CSS: {}", relative.display());
                    }
                    "js" => {
                        let content = fs::read_to_string(path)
                            .with_context(|| format!("Failed to read JS: {}", path.display()))?;
                        let minified = minify_js(&content)?;
                        fs::write(&dest_path, minified)?;
                        info!("Minified JS: {}", relative.display());
                    }
                    _ => {
                        // Copy non-minifiable files as-is
                        fs::copy(path, &dest_path)?;
                    }
                }
            } else {
                // Copy files without minification
                fs::copy(path, &dest_path)?;
            }
        }
    }

    info!(
        "Copied theme assets to {} (minify: {})",
        output_assets.display(),
        minify_enabled
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minify_css() {
        let css = "body {\n    color: red;\n}";
        let result = minify_css(css).unwrap();
        assert!(!result.contains('\n'));
    }

    #[test]
    fn test_minify_js() {
        let js = "function hello() {\n    console.log('world');\n}";
        let result = minify_js(js).unwrap();
        // JS minifier removes unnecessary whitespace
        assert!(result.len() <= js.len());
    }
}
