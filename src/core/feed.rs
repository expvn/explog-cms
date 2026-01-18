//! RSS and Atom feed generation module
//!
//! Generates RSS 2.0 and Atom 1.0 feeds for blog posts.

use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use std::fs;
use std::path::Path;
use tracing::info;

use crate::models::{Post, SiteConfig};

/// Maximum number of posts to include in feeds
const FEED_ITEM_LIMIT: usize = 20;

/// Generate RSS 2.0 feed
pub fn generate_rss(config: &SiteConfig, posts: &[Post], output_dir: &str) -> Result<()> {
    let base_url = config.site.base_url.trim_end_matches('/');
    let now = Utc::now();
    
    let items: Vec<String> = posts
        .iter()
        .take(FEED_ITEM_LIMIT)
        .map(|post| {
            let pub_date = parse_date(&post.date)
                .map(|d| d.format("%a, %d %b %Y %H:%M:%S +0000").to_string())
                .unwrap_or_else(|| now.format("%a, %d %b %Y %H:%M:%S +0000").to_string());
            
            let description = post.summary.clone().unwrap_or_default();
            
            format!(
                r#"    <item>
      <title>{}</title>
      <link>{}{}</link>
      <guid>{}{}</guid>
      <pubDate>{}</pubDate>
      <description><![CDATA[{}]]></description>
    </item>"#,
                escape_xml(&post.title),
                base_url,
                post.url,
                base_url,
                post.url,
                pub_date,
                description
            )
        })
        .collect();
    
    let rss = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>{}</title>
    <link>{}/</link>
    <description>{}</description>
    <language>{}</language>
    <lastBuildDate>{}</lastBuildDate>
    <atom:link href="{}/rss.xml" rel="self" type="application/rss+xml"/>
{}
  </channel>
</rss>
"#,
        escape_xml(&config.site.title),
        base_url,
        escape_xml(&config.site.description),
        &config.site.language,
        now.format("%a, %d %b %Y %H:%M:%S +0000"),
        base_url,
        items.join("\n")
    );
    
    let rss_path = Path::new(output_dir).join("rss.xml");
    fs::write(&rss_path, &rss)?;
    info!("Generated RSS feed: {}", rss_path.display());
    
    Ok(())
}

/// Generate Atom 1.0 feed
pub fn generate_atom(config: &SiteConfig, posts: &[Post], output_dir: &str) -> Result<()> {
    let base_url = config.site.base_url.trim_end_matches('/');
    let now = Utc::now();
    
    let entries: Vec<String> = posts
        .iter()
        .take(FEED_ITEM_LIMIT)
        .map(|post| {
            let updated = parse_date(&post.date)
                .map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string())
                .unwrap_or_else(|| now.format("%Y-%m-%dT%H:%M:%SZ").to_string());
            
            let summary = post.summary.clone().unwrap_or_default();
            
            format!(
                r#"  <entry>
    <title>{}</title>
    <link href="{}{}" rel="alternate" type="text/html"/>
    <id>{}{}</id>
    <updated>{}</updated>
    <summary type="html"><![CDATA[{}]]></summary>
    <author>
      <name>{}</name>
    </author>
  </entry>"#,
                escape_xml(&post.title),
                base_url,
                post.url,
                base_url,
                post.url,
                updated,
                summary,
                escape_xml(&post.author)
            )
        })
        .collect();
    
    let atom = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>{}</title>
  <link href="{}/atom.xml" rel="self" type="application/atom+xml"/>
  <link href="{}/"/>
  <id>{}/</id>
  <updated>{}</updated>
  <subtitle>{}</subtitle>
{}
</feed>
"#,
        escape_xml(&config.site.title),
        base_url,
        base_url,
        base_url,
        now.format("%Y-%m-%dT%H:%M:%SZ"),
        escape_xml(&config.site.description),
        entries.join("\n")
    );
    
    let atom_path = Path::new(output_dir).join("atom.xml");
    fs::write(&atom_path, &atom)?;
    info!("Generated Atom feed: {}", atom_path.display());
    
    Ok(())
}

/// Generate both RSS and Atom feeds
pub fn generate_feeds(config: &SiteConfig, posts: &[Post], output_dir: &str) -> Result<()> {
    if config.seo.generate_rss {
        generate_rss(config, posts, output_dir)?;
        generate_atom(config, posts, output_dir)?;
    }
    Ok(())
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn parse_date(date_str: &str) -> Option<DateTime<Utc>> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .ok()
        .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc())
}
