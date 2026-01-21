//! Asset validation for strict mode
//! 
//! Validates that all assets referenced in Markdown content exist on disk.

use anyhow::Result;
use regex::Regex;
use std::path::Path;
use tracing::{info, warn};

use crate::models::{Page, Post};

/// Asset validation error
#[derive(Debug, Clone)]
pub struct AssetError {
    pub source_file: String,
    pub asset_path: String,
    pub error_type: AssetErrorType,
}

#[derive(Debug, Clone)]
pub enum AssetErrorType {
    NotFound,
    InvalidPath,
}

impl std::fmt::Display for AssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error_type {
            AssetErrorType::NotFound => {
                write!(f, "Asset not found: '{}' in '{}'", self.asset_path, self.source_file)
            }
            AssetErrorType::InvalidPath => {
                write!(f, "Invalid asset path: '{}' in '{}'", self.asset_path, self.source_file)
            }
        }
    }
}

/// Validate all assets referenced in posts and pages
pub fn validate_assets(posts: &[Post], pages: &[Page]) -> Vec<AssetError> {
    let mut errors = Vec::new();
    
    // Regex patterns for asset references
    let img_regex = Regex::new(r#"!\[.*?\]\((\.?/[^)]+)\)"#).unwrap();
    let link_regex = Regex::new(r#"\[.*?\]\((\.?/[^)]+\.(jpg|jpeg|png|gif|webp|svg|pdf|zip))\)"#).unwrap();
    let html_img_regex = Regex::new(r#"<img[^>]+src=["']([^"']+)["']"#).unwrap();
    
    // Validate posts
    for post in posts {
        let post_dir = Path::new(&post.source_path).parent().unwrap_or(Path::new("."));
        let content = &post.content;
        
        // Check Markdown images
        for cap in img_regex.captures_iter(content) {
            let asset_path = &cap[1];
            if let Err(e) = check_asset(post_dir, asset_path) {
                errors.push(AssetError {
                    source_file: post.source_path.clone(),
                    asset_path: asset_path.to_string(),
                    error_type: e,
                });
            }
        }
        
        // Check link assets (PDFs, images in links)
        for cap in link_regex.captures_iter(content) {
            let asset_path = &cap[1];
            if let Err(e) = check_asset(post_dir, asset_path) {
                errors.push(AssetError {
                    source_file: post.source_path.clone(),
                    asset_path: asset_path.to_string(),
                    error_type: e,
                });
            }
        }
        
        // Check HTML img tags
        for cap in html_img_regex.captures_iter(content) {
            let asset_path = &cap[1];
            // Skip external URLs
            if asset_path.starts_with("http://") || asset_path.starts_with("https://") {
                continue;
            }
            if let Err(e) = check_asset(post_dir, asset_path) {
                errors.push(AssetError {
                    source_file: post.source_path.clone(),
                    asset_path: asset_path.to_string(),
                    error_type: e,
                });
            }
        }
        
        // Check cover image if specified
        if let Some(ref cover) = post.cover {
            if !cover.starts_with("http://") && !cover.starts_with("https://") {
                if let Err(e) = check_asset(post_dir, cover) {
                    errors.push(AssetError {
                        source_file: post.source_path.clone(),
                        asset_path: cover.clone(),
                        error_type: e,
                    });
                }
            }
        }
    }
    
    // Validate pages
    for page in pages {
        let page_dir = Path::new(&page.source_path).parent().unwrap_or(Path::new("."));
        let content = &page.content;
        
        for cap in img_regex.captures_iter(content) {
            let asset_path = &cap[1];
            if let Err(e) = check_asset(page_dir, asset_path) {
                errors.push(AssetError {
                    source_file: page.source_path.clone(),
                    asset_path: asset_path.to_string(),
                    error_type: e,
                });
            }
        }
    }
    
    if errors.is_empty() {
        info!("Asset validation passed: all referenced assets exist");
    } else {
        warn!("Asset validation found {} errors", errors.len());
    }
    
    errors
}

/// Check if an asset exists relative to the source directory
fn check_asset(source_dir: &Path, asset_path: &str) -> Result<(), AssetErrorType> {
    // Normalize the path
    let normalized = asset_path
        .trim_start_matches("./")
        .trim_start_matches('/');
    
    if normalized.is_empty() {
        return Err(AssetErrorType::InvalidPath);
    }
    
    // Check relative to source directory
    let full_path = source_dir.join(normalized);
    if full_path.exists() {
        return Ok(());
    }
    
    // Check in content root
    let content_path = Path::new("content").join(normalized);
    if content_path.exists() {
        return Ok(());
    }
    
    // Check in public/media (for already processed assets)
    let public_path = Path::new("public").join(normalized);
    if public_path.exists() {
        return Ok(());
    }
    
    Err(AssetErrorType::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_path() {
        let source_dir = Path::new(".");
        assert!(check_asset(source_dir, "./Cargo.toml").is_ok());
    }
}
