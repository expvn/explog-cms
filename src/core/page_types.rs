//! Page Types Module
//! 
//! Handles three types of pages:
//! - Customize: JSON-driven grid layouts
//! - Embedded: Markdown pages using theme (current behavior)
//! - Standalone: Independent landing pages with custom HTML/CSS/JS

#![allow(dead_code)]

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::info;

/// Page type enumeration
#[derive(Debug, Clone)]
pub enum PageType {
    Customize(CustomizePage),
    Embedded(EmbeddedPage),
    Standalone(StandalonePage),
}

/// Customize page - JSON-driven grid layouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizePage {
    pub slug: String,
    pub title: String,
    #[serde(default = "default_template")]
    pub template: String,
    #[serde(default = "default_columns")]
    pub columns: u32,
    #[serde(default)]
    pub filter: bool,
    #[serde(default)]
    pub items: Vec<CustomizeItem>,
    #[serde(default)]
    pub meta: CustomizeMeta,
}

fn default_template() -> String {
    "grid".to_string()
}

fn default_columns() -> u32 {
    3
}

/// Item in a customize grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizeItem {
    pub title: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Metadata for customize page
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomizeMeta {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub cover: Option<String>,
}

/// Embedded page - uses theme (standard behavior)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedPage {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub source_path: String,
    #[serde(default)]
    pub cover: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
}

/// Standalone page - independent landing page
#[derive(Debug, Clone)]
pub struct StandalonePage {
    pub slug: String,
    pub source_dir: String,
    pub files: Vec<String>,
}

/// Detect page type from directory
pub fn detect_page_type(page_dir: &Path) -> Result<PageType> {
    let slug = page_dir.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    // Check for config.json (Customize page)
    let config_path = page_dir.join("config.json");
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {}", config_path.display()))?;
        let mut page: CustomizePage = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse {}", config_path.display()))?;
        page.slug = slug;
        return Ok(PageType::Customize(page));
    }
    
    // Check for index.html (Standalone page)
    let html_path = page_dir.join("index.html");
    if html_path.exists() {
        let files = collect_files(page_dir)?;
        return Ok(PageType::Standalone(StandalonePage {
            slug,
            source_dir: page_dir.to_string_lossy().to_string(),
            files,
        }));
    }
    
    // Check for index.md (Embedded page - default)
    let md_path = page_dir.join("index.md");
    if md_path.exists() {
        let content = fs::read_to_string(&md_path)?;
        let (frontmatter, body) = parse_frontmatter(&content)?;
        
        return Ok(PageType::Embedded(EmbeddedPage {
            slug: slug.clone(),
            title: frontmatter.get("title").cloned().unwrap_or_else(|| slug.clone()),
            content: render_markdown(&body),
            source_path: md_path.to_string_lossy().to_string(),
            cover: frontmatter.get("cover").cloned(),
            summary: frontmatter.get("summary").cloned(),
        }));
    }
    
    // Default to empty embedded page
    Ok(PageType::Embedded(EmbeddedPage {
        slug: slug.clone(),
        title: slug,
        content: String::new(),
        source_path: page_dir.to_string_lossy().to_string(),
        cover: None,
        summary: None,
    }))
}

/// Collect all files in a directory
fn collect_files(dir: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            files.push(path.to_string_lossy().to_string());
        } else if path.is_dir() {
            files.extend(collect_files(&path)?);
        }
    }
    
    Ok(files)
}

/// Parse YAML frontmatter from markdown
fn parse_frontmatter(content: &str) -> Result<(std::collections::HashMap<String, String>, String)> {
    use std::collections::HashMap;
    
    if content.starts_with("---") {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() >= 3 {
            let yaml_str = parts[1].trim();
            let body = parts[2].trim();
            
            let map: HashMap<String, String> = serde_yaml::from_str(yaml_str)
                .unwrap_or_default();
            
            return Ok((map, body.to_string()));
        }
    }
    
    Ok((std::collections::HashMap::new(), content.to_string()))
}

/// Render markdown to HTML
fn render_markdown(content: &str) -> String {
    use pulldown_cmark::{html, Parser};
    
    let parser = Parser::new(content);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Load all pages from content/pages directory
pub fn load_all_pages(pages_dir: &str) -> Result<Vec<PageType>> {
    let customize_dir = Path::new(pages_dir).join("customize");
    let embedded_dir = Path::new(pages_dir).join("embedded");
    let standalone_dir = Path::new(pages_dir).join("standalone");
    
    let mut pages = Vec::new();
    
    // Load customize pages
    if customize_dir.exists() {
        for entry in fs::read_dir(&customize_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                match detect_page_type(&entry.path()) {
                    Ok(page) => pages.push(page),
                    Err(e) => tracing::warn!("Failed to load page {:?}: {}", entry.path(), e),
                }
            }
        }
        info!("Loaded {} customize pages", pages.iter().filter(|p| matches!(p, PageType::Customize(_))).count());
    }
    
    // Load embedded pages
    if embedded_dir.exists() {
        for entry in fs::read_dir(&embedded_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                match detect_page_type(&entry.path()) {
                    Ok(page) => pages.push(page),
                    Err(e) => tracing::warn!("Failed to load page {:?}: {}", entry.path(), e),
                }
            }
        }
        info!("Loaded {} embedded pages", pages.iter().filter(|p| matches!(p, PageType::Embedded(_))).count());
    }
    
    // Load standalone pages
    if standalone_dir.exists() {
        for entry in fs::read_dir(&standalone_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                match detect_page_type(&entry.path()) {
                    Ok(page) => pages.push(page),
                    Err(e) => tracing::warn!("Failed to load page {:?}: {}", entry.path(), e),
                }
            }
        }
        info!("Loaded {} standalone pages", pages.iter().filter(|p| matches!(p, PageType::Standalone(_))).count());
    }
    
    // Also load from root pages directory (backward compatibility)
    let root_pages = Path::new(pages_dir);
    if root_pages.exists() {
        for entry in fs::read_dir(root_pages)? {
            let entry = entry?;
            let path = entry.path();
            // Skip the special directories
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name != "customize" && name != "embedded" && name != "standalone" {
                    match detect_page_type(&path) {
                        Ok(page) => pages.push(page),
                        Err(e) => tracing::warn!("Failed to load page {:?}: {}", path, e),
                    }
                }
            }
        }
    }
    
    Ok(pages)
}
