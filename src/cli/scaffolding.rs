//! Scaffolding CLI module
//! 
//! Provides commands for creating new pages and content.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use tracing::info;

/// Create a new page with scaffolding
pub fn create_page(page_type: &str, name: &str, pages_dir: &str) -> Result<()> {
    match page_type.to_lowercase().as_str() {
        "customize" => create_customize_page(name, pages_dir),
        "embedded" => create_embedded_page(name, pages_dir),
        "standalone" => create_standalone_page(name, pages_dir),
        _ => {
            anyhow::bail!("Unknown page type: {}. Use: customize, embedded, standalone", page_type);
        }
    }
}

/// Create a customize (JSON-driven) page
fn create_customize_page(name: &str, pages_dir: &str) -> Result<()> {
    let page_dir = Path::new(pages_dir).join("customize").join(name);
    let assets_dir = page_dir.join("assets");
    
    fs::create_dir_all(&assets_dir)
        .with_context(|| format!("Failed to create directory: {:?}", assets_dir))?;
    
    let config = format!(r#"{{
  "title": "{}",
  "template": "grid",
  "columns": 3,
  "filter": true,
  "meta": {{
    "description": "Add your description here"
  }},
  "items": [
    {{
      "title": "Item 1",
      "image": "assets/item1.jpg",
      "category": "Category A",
      "description": "Description of item 1",
      "link": "/link/",
      "tags": ["tag1", "tag2"]
    }},
    {{
      "title": "Item 2",
      "image": "assets/item2.jpg",
      "category": "Category B",
      "description": "Description of item 2",
      "link": "/link/",
      "tags": ["tag3"]
    }}
  ]
}}"#, capitalize(name));
    
    fs::write(page_dir.join("config.json"), config)?;
    
    info!("Created customize page: {}", name);
    info!("  → {}/config.json", page_dir.display());
    info!("  → {}/assets/", page_dir.display());
    info!("  Add your images to assets/ and update config.json");
    
    Ok(())
}

/// Create an embedded (Markdown) page
fn create_embedded_page(name: &str, pages_dir: &str) -> Result<()> {
    let page_dir = Path::new(pages_dir).join("embedded").join(name);
    
    fs::create_dir_all(&page_dir)
        .with_context(|| format!("Failed to create directory: {:?}", page_dir))?;
    
    let title = capitalize(name);
    let content = format!(r#"---
title: {}
summary: Add your page summary here
---

# {}

Your content goes here. This page uses your theme's styling.

## Section 1

Add your content...

## Section 2

Add more content...
"#, title, title);
    
    fs::write(page_dir.join("index.md"), content)?;
    
    info!("Created embedded page: {}", name);
    info!("  → {}/index.md", page_dir.display());
    info!("  Edit index.md to add your content");
    
    Ok(())
}

/// Create a standalone (landing) page
fn create_standalone_page(name: &str, pages_dir: &str) -> Result<()> {
    let page_dir = Path::new(pages_dir).join("standalone").join(name);
    
    fs::create_dir_all(&page_dir)
        .with_context(|| format!("Failed to create directory: {:?}", page_dir))?;
    
    let title = capitalize(name);
    
    // Create HTML
    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{}</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <div class="container">
    <header>
      <a href="/" class="back-link">← Back to Blog</a>
    </header>
    
    <main>
      <h1>{}</h1>
      <p>Your standalone landing page content goes here.</p>
    </main>
    
    <footer>
      <p>&copy; 2024 Your Site</p>
    </footer>
  </div>
  
  <script src="script.js"></script>
</body>
</html>"#, title, title);
    
    // Create CSS
    let css = r#"* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: system-ui, sans-serif;
  background: #0f172a;
  color: #f8fafc;
  min-height: 100vh;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 24px;
}

header {
  margin-bottom: 48px;
}

.back-link {
  color: #94a3b8;
  text-decoration: none;
}

.back-link:hover {
  color: white;
}

main {
  text-align: center;
  padding: 80px 0;
}

h1 {
  font-size: 3rem;
  margin-bottom: 24px;
}

footer {
  text-align: center;
  padding: 24px 0;
  color: #64748b;
}
"#;
    
    // Create JS
    let js = r#"// Your custom scripts here
document.addEventListener('DOMContentLoaded', () => {
  console.log('Standalone page loaded');
});
"#;
    
    fs::write(page_dir.join("index.html"), html)?;
    fs::write(page_dir.join("style.css"), css)?;
    fs::write(page_dir.join("script.js"), js)?;
    
    info!("Created standalone page: {}", name);
    info!("  → {}/index.html", page_dir.display());
    info!("  → {}/style.css", page_dir.display());
    info!("  → {}/script.js", page_dir.display());
    info!("  This page will be copied directly to output without processing");
    
    Ok(())
}

/// Capitalize the first letter of a string
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(chars).collect(),
    }
}
