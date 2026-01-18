use anyhow::{Context, Result};
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::models::page::Page;
use crate::models::post::{Post, PostFrontmatter};

/// Parse a markdown file with YAML frontmatter
pub fn parse_markdown_file<P: AsRef<Path>>(path: P) -> Result<(String, String)> {
    let path = path.as_ref();
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    parse_markdown(&content)
}

/// Parse markdown content with YAML frontmatter
/// Returns (frontmatter_yaml, html_content)
pub fn parse_markdown(content: &str) -> Result<(String, String)> {
    let content = content.trim();

    // Check for frontmatter delimiter
    if !content.starts_with("---") {
        // No frontmatter, just convert markdown to HTML
        let html = markdown_to_html(content);
        return Ok((String::new(), html));
    }

    // Find the closing delimiter
    let rest = &content[3..];
    let end_pos = rest
        .find("\n---")
        .context("Invalid frontmatter: missing closing ---")?;

    let frontmatter = rest[..end_pos].trim().to_string();
    let markdown_content = rest[end_pos + 4..].trim();
    let html = markdown_to_html(markdown_content);

    Ok((frontmatter, html))
}

/// Convert bare URLs to markdown links before processing
/// Only converts URLs that are NOT inside:
/// - Markdown link syntax [text](url)
/// - Code blocks (``` ... ```)
/// - Inline code (`...`)
fn linkify_urls(text: &str) -> String {
    use regex::Regex;
    
    // Simple URL regex - matches http/https URLs
    let url_regex = Regex::new(r"(https?://[^\s<>\[\]`]+)").unwrap();
    
    let mut result = String::new();
    let mut last_end = 0;
    
    for cap in url_regex.captures_iter(text) {
        let m = cap.get(1).unwrap();
        let url = m.as_str();
        let start = m.start();
        
        // Check if this URL is already inside a markdown link
        let before = &text[..start];
        let is_in_link = before.ends_with("](") || 
                         before.ends_with("\"") ||
                         before.ends_with("'");
        
        // Check if inside code block (count unmatched ```)
        let code_block_count = before.matches("```").count();
        let is_in_code_block = code_block_count % 2 == 1;
        
        // Check if inside inline code (count unmatched `)
        // But exclude ``` which we already handled
        let before_no_blocks = before.replace("```", "___");
        let backtick_count = before_no_blocks.matches('`').count();
        let is_in_inline_code = backtick_count % 2 == 1;
        
        if is_in_link || is_in_code_block || is_in_inline_code {
            // Keep as-is
            result.push_str(&text[last_end..m.end()]);
        } else {
            // Add text before URL
            result.push_str(&text[last_end..start]);
            // Convert to markdown link
            result.push_str(&format!("[{}]({})", url, url));
        }
        last_end = m.end();
    }
    
    // Add remaining text
    result.push_str(&text[last_end..]);
    result
}

/// Convert markdown to HTML with Obsidian-like features
pub fn markdown_to_html(markdown: &str) -> String {
    use pulldown_cmark::Event;
    
    // Pre-process: convert bare URLs to markdown links
    let linkified = linkify_urls(markdown);
    
    // Pre-process: handle callouts before pulldown_cmark
    let preprocessed = preprocess_callouts(&linkified);
    
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(&preprocessed, options);
    
    // Map SoftBreak to HardBreak so single newlines become <br>
    let parser = parser.map(|event| match event {
        Event::SoftBreak => Event::HardBreak,
        _ => event,
    });
    
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Post-process: enhance HTML output
    let html_output = postprocess_images(&html_output);
    let html_output = postprocess_youtube(&html_output);
    let html_output = postprocess_callouts(&html_output);
    let html_output = postprocess_line_breaks(&html_output);

    html_output
}

/// Pre-process callouts (> [!NOTE], > [!WARNING], etc.)
fn preprocess_callouts(markdown: &str) -> String {
    use regex::Regex;
    
    // Match Obsidian-style callouts: > [!TYPE]
    let callout_regex = Regex::new(r"(?m)^>\s*\[!(NOTE|WARNING|TIP|IMPORTANT|CAUTION|INFO)\]\s*\n((?:>.*\n?)*)").unwrap();
    
    callout_regex.replace_all(markdown, |caps: &regex::Captures| {
        let callout_type = caps.get(1).map(|m| m.as_str()).unwrap_or("NOTE");
        let content = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        
        // Remove leading > from each line
        let clean_content: String = content
            .lines()
            .map(|line| line.trim_start_matches('>').trim())
            .collect::<Vec<_>>()
            .join("\n");
        
        format!(
            "{{{{CALLOUT_START:{}}}}}{}{{{{CALLOUT_END}}}}\n\n",
            callout_type.to_lowercase(),
            clean_content
        )
    }).to_string()
}

/// Post-process: center images
fn postprocess_images(html: &str) -> String {
    use regex::Regex;
    
    // Wrap standalone <img> tags in centered figure
    let img_regex = Regex::new(r#"<p>\s*(<img[^>]*>)\s*</p>"#).unwrap();
    
    img_regex.replace_all(html, |caps: &regex::Captures| {
        let img_tag = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        format!(
            r#"<figure class="image-center">{}</figure>"#,
            img_tag
        )
    }).to_string()
}

/// Post-process: convert YouTube URLs to embeds
fn postprocess_youtube(html: &str) -> String {
    use regex::Regex;
    
    // Match YouTube URLs in img tags or links
    let youtube_regex = Regex::new(
        r#"<img[^>]*src="(?:https?://)?(?:www\.)?(?:youtube\.com/watch\?v=|youtu\.be/)([a-zA-Z0-9_-]{11})[^"]*"[^>]*>"#
    ).unwrap();
    
    let html = youtube_regex.replace_all(html, |caps: &regex::Captures| {
        let video_id = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        format!(
            r#"<div class="video-container"><iframe src="https://www.youtube.com/embed/{}" frameborder="0" allowfullscreen loading="lazy"></iframe></div>"#,
            video_id
        )
    }).to_string();
    
    // Also handle YouTube links in anchor tags
    let link_regex = Regex::new(
        r#"<a[^>]*href="(?:https?://)?(?:www\.)?(?:youtube\.com/watch\?v=|youtu\.be/)([a-zA-Z0-9_-]{11})[^"]*"[^>]*>[^<]*</a>"#
    ).unwrap();
    
    link_regex.replace_all(&html, |caps: &regex::Captures| {
        let video_id = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        format!(
            r#"<div class="video-container"><iframe src="https://www.youtube.com/embed/{}" frameborder="0" allowfullscreen loading="lazy"></iframe></div>"#,
            video_id
        )
    }).to_string()
}

/// Post-process: convert callout markers to styled divs
fn postprocess_callouts(html: &str) -> String {
    use regex::Regex;
    
    let callout_regex = Regex::new(
        r"\{\{CALLOUT_START:(\w+)\}\}([\s\S]*?)\{\{CALLOUT_END\}\}"
    ).unwrap();
    
    callout_regex.replace_all(html, |caps: &regex::Captures| {
        let callout_type = caps.get(1).map(|m| m.as_str()).unwrap_or("note");
        let content = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        
        let icon = match callout_type {
            "note" => "📝",
            "warning" => "⚠️",
            "tip" => "💡",
            "important" => "❗",
            "caution" => "🚨",
            "info" => "ℹ️",
            _ => "📝",
        };
        
        let title = match callout_type {
            "note" => "Note",
            "warning" => "Warning",
            "tip" => "Tip",
            "important" => "Important",
            "caution" => "Caution",
            "info" => "Info",
            _ => "Note",
        };
        
        format!(
            r#"<div class="callout callout-{}"><div class="callout-title">{} {}</div><div class="callout-content">{}</div></div>"#,
            callout_type, icon, title, content.trim()
        )
    }).to_string()
}

/// Post-process: handle line breaks (single newline → <br>)
fn postprocess_line_breaks(html: &str) -> String {
    // In markdown, two spaces at end of line = <br>
    // pulldown_cmark handles this, but we can also convert single newlines in paragraph
    // For now, keep pulldown_cmark behavior - it's standard CommonMark
    html.to_string()
}

/// Load all posts from content directory (parallel)
pub fn load_posts<P: AsRef<Path>>(posts_dir: P) -> Result<Vec<Post>> {
    use rayon::prelude::*;
    
    let posts_dir = posts_dir.as_ref();

    if !posts_dir.exists() {
        return Ok(Vec::new());
    }

    // Collect all index.md paths first
    let paths: Vec<_> = WalkDir::new(posts_dir)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().file_name().map(|f| f == "index.md").unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect();

    // Parse files in parallel
    let mut posts: Vec<Post> = paths
        .par_iter()
        .filter_map(|path| parse_post(path).ok())
        .filter(|p| !p.draft)
        .collect();

    // Sort by date (newest first)
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(posts)
}

/// Parse a single post from its index.md file
fn parse_post<P: AsRef<Path>>(path: P) -> Result<Post> {
    let path = path.as_ref();
    let (frontmatter_yaml, content_html) = parse_markdown_file(path)?;

    let frontmatter: PostFrontmatter = serde_yaml::from_str(&frontmatter_yaml)
        .with_context(|| format!("Failed to parse frontmatter in: {}", path.display()))?;

    // Extract slug from parent directory name
    let slug = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("untitled")
        .to_string();

    // Read raw content for summary extraction
    let raw_content = fs::read_to_string(path)?;
    let raw_markdown = raw_content
        .split("\n---")
        .nth(1)
        .map(|s| s.trim_start_matches('-').trim())
        .unwrap_or("");

    // Generate summary if not provided
    let summary = frontmatter.summary.or_else(|| {
        let text: String = raw_markdown.chars().take(200).collect();
        if text.len() >= 200 {
            Some(format!("{}...", text))
        } else if !text.is_empty() {
            Some(text)
        } else {
            None
        }
    });

    Ok(Post {
        title: frontmatter.title,
        slug: frontmatter.slug.unwrap_or_else(|| slug.clone()),
        date: frontmatter.date.unwrap_or_else(|| "1970-01-01".to_string()),
        content: content_html,
        raw_content: raw_markdown.to_string(),
        summary,
        categories: frontmatter.categories,
        tags: frontmatter.tags,
        // Rewrite cover path to public URL
        cover: frontmatter.cover.map(|c| format!("/media/{}/{}", slug, c)),
        featured: frontmatter.featured,
        draft: frontmatter.draft,
        author: frontmatter.author,
        seo: frontmatter.seo,
        related: frontmatter.related,
        url: format!("/post/{}/", slug),
        source_path: path.to_string_lossy().to_string(),
        publish_date: frontmatter.publish_date,
        preview_token: frontmatter.preview_token,
    })
}

/// Load all pages from content directory
/// Flat structure: content/pages/{page-name}/page.json
/// page.json type + mode fields determine processing
pub fn load_pages<P: AsRef<Path>>(pages_dir: P) -> Result<Vec<Page>> {
    use crate::models::page::{ContentType, PageMode, PageJsonConfig};
    
    let pages_dir = pages_dir.as_ref();

    if !pages_dir.exists() {
        return Ok(Vec::new());
    }

    let mut pages: Vec<Page> = Vec::new();

    // Scan all subdirectories in content/pages/
    for entry in fs::read_dir(pages_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if !path.is_dir() {
            continue;
        }

        let slug = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("untitled")
            .to_string();

        // page.json is REQUIRED
        let json_path = path.join("page.json");
        if !json_path.exists() {
            tracing::warn!("Skipping page {} - no page.json found", slug);
            continue;
        }

        let json_content = fs::read_to_string(&json_path)?;
        let config: PageJsonConfig = serde_json::from_str(&json_content)
            .with_context(|| format!("Failed to parse page.json in: {}", path.display()))?;

        let final_slug = config.slug.clone().unwrap_or_else(|| slug.clone());
        
        // Parse content type (default: markdown)
        let type_str = config.content_type.clone().unwrap_or_else(|| "markdown".to_string());
        let content_type = match type_str.as_str() {
            "markdown" => ContentType::Markdown,
            "html" => ContentType::Html,
            "gallery" => ContentType::Gallery,
            // Legacy support
            "webgl" => ContentType::Html,
            "standalone" => ContentType::Html,
            _ => ContentType::Markdown,
        };

        // Parse mode (default: embedded, legacy standalone type → standalone mode)
        let mode_str = config.mode.clone().unwrap_or_else(|| {
            // Legacy: standalone type implies standalone mode
            if type_str == "standalone" { "standalone".to_string() } 
            else { "embedded".to_string() }
        });
        let mode = match mode_str.as_str() {
            "embedded" => PageMode::Embedded,
            "standalone" => PageMode::Standalone,
            _ => PageMode::Embedded,
        };

        // Load content based on content type
        let content = match content_type {
            ContentType::Markdown => {
                let md_path = path.join("index.md");
                if md_path.exists() {
                    let (_, content_html) = parse_markdown_file(&md_path)?;
                    content_html
                } else {
                    String::new()
                }
            }
            ContentType::Html => {
                let html_path = path.join("index.html");
                if html_path.exists() {
                    fs::read_to_string(&html_path)?
                } else {
                    String::new()
                }
            }
            ContentType::Gallery => {
                // Gallery content comes from template rendering
                String::new()
            }
        };

        pages.push(Page {
            title: config.title.clone(),
            slug: final_slug.clone(),
            content,
            url: format!("/{}/", final_slug),
            content_type,
            mode,
            template: config.template.clone(),
            description: config.description.clone(),
            embed: config.embed.clone(),
            items: config.items.clone(),
            source_dir: Some(path.to_string_lossy().to_string()),
            source_path: json_path.to_string_lossy().to_string(),
        });
    }

    Ok(pages)
}
