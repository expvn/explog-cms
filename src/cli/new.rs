use anyhow::Result;
use chrono::Local;
use std::fs;
use std::path::Path;
use tracing::info;

/// Create a new post or page
pub fn run(content_type: &str, title: &str) -> Result<()> {
    let slug = slugify(title);
    let date = Local::now().format("%Y-%m-%d").to_string();

    let (dir, template) = match content_type {
        "post" => {
            let dir = format!("content/posts/{}", slug);
            let template = format!(
                r#"---
title: "{}"
date: {}
categories:
  - General
tags: []
draft: false
---

Write your content here...
"#,
                title, date
            );
            (dir, template)
        }
        "page" => {
            let dir = format!("content/pages/{}", slug);
            let template = format!(
                r#"---
title: "{}"
---

Write your page content here...
"#,
                title
            );
            (dir, template)
        }
        _ => return Err(anyhow::anyhow!("Invalid content type")),
    };

    // Create directory
    fs::create_dir_all(&dir)?;

    // Write index.md
    let file_path = Path::new(&dir).join("index.md");
    fs::write(&file_path, template)?;

    info!("Created {} at {}", content_type, file_path.display());
    Ok(())
}

/// Convert title to URL-friendly slug
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' {
                '-'
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
