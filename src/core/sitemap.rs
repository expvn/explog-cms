use anyhow::Result;
use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;
use tracing::info;

use crate::models::{Page, Post, SiteConfig};
use crate::models::site::{Category, Tag};

/// Maximum URLs per sitemap file (Google limit is 50,000, we use 5,000 for safety)
const SITEMAP_CHUNK_SIZE: usize = 5000;

/// Generate XML sitemap for SEO with chunking support
pub fn generate_sitemap(
    config: &SiteConfig,
    posts: &[Post],
    pages: &[Page],
    categories: &[Category],
    tags: &[Tag],
    output_dir: &str,
) -> Result<()> {
    let base_url = config.site.base_url.trim_end_matches('/');
    let now = Utc::now();
    
    let mut urls = Vec::new();
    
    // Home page
    urls.push(SitemapUrl {
        loc: format!("{}/", base_url),
        lastmod: now,
        changefreq: "daily".to_string(),
        priority: 1.0,
    });
    
    // Posts
    for post in posts {
        urls.push(SitemapUrl {
            loc: format!("{}/post/{}/", base_url, post.slug),
            lastmod: parse_date(&post.date).unwrap_or(now),
            changefreq: "weekly".to_string(),
            priority: 0.8,
        });
    }
    
    // Pages
    for page in pages {
        urls.push(SitemapUrl {
            loc: format!("{}/{}/", base_url, page.slug),
            lastmod: now,
            changefreq: "monthly".to_string(),
            priority: 0.7,
        });
    }
    
    // Categories
    for category in categories {
        urls.push(SitemapUrl {
            loc: format!("{}/category/{}/", base_url, category.slug),
            lastmod: now,
            changefreq: "weekly".to_string(),
            priority: 0.6,
        });
    }
    
    // Tags
    for tag in tags {
        urls.push(SitemapUrl {
            loc: format!("{}/tag/{}/", base_url, tag.slug),
            lastmod: now,
            changefreq: "weekly".to_string(),
            priority: 0.5,
        });
    }
    
    // Check if we need to chunk
    if urls.len() > SITEMAP_CHUNK_SIZE {
        generate_chunked_sitemaps(&urls, base_url, output_dir, now)?;
    } else {
        // Single sitemap
        let xml = generate_sitemap_xml(&urls);
        let sitemap_path = Path::new(output_dir).join("sitemap.xml");
        fs::write(&sitemap_path, &xml)?;
    }
    
    // Generate robots.txt
    let robots = generate_robots_txt(base_url);
    let robots_path = Path::new(output_dir).join("robots.txt");
    fs::write(&robots_path, &robots)?;
    
    info!("Generated sitemap with {} URLs", urls.len());
    
    Ok(())
}

/// Generate chunked sitemaps with sitemap-index.xml
fn generate_chunked_sitemaps(
    urls: &[SitemapUrl],
    base_url: &str,
    output_dir: &str,
    now: DateTime<Utc>,
) -> Result<()> {
    let chunks: Vec<_> = urls.chunks(SITEMAP_CHUNK_SIZE).collect();
    let mut sitemap_files = Vec::new();
    
    for (i, chunk) in chunks.iter().enumerate() {
        let filename = format!("sitemap-{}.xml", i + 1);
        let xml = generate_sitemap_xml(chunk);
        let path = Path::new(output_dir).join(&filename);
        fs::write(&path, &xml)?;
        sitemap_files.push(filename);
    }
    
    // Generate sitemap index
    let index_xml = generate_sitemap_index(&sitemap_files, base_url, now);
    let index_path = Path::new(output_dir).join("sitemap.xml");
    fs::write(&index_path, &index_xml)?;
    
    info!("Generated {} sitemap chunks", sitemap_files.len());
    
    Ok(())
}

/// Generate sitemap index XML
fn generate_sitemap_index(files: &[String], base_url: &str, now: DateTime<Utc>) -> String {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#);

    for file in files {
        xml.push_str(&format!(
            r#"  <sitemap>
    <loc>{}/{}</loc>
    <lastmod>{}</lastmod>
  </sitemap>
"#,
            base_url,
            file,
            now.format("%Y-%m-%d")
        ));
    }

    xml.push_str("</sitemapindex>\n");
    xml
}

struct SitemapUrl {
    loc: String,
    lastmod: DateTime<Utc>,
    changefreq: String,
    priority: f32,
}

fn generate_sitemap_xml(urls: &[SitemapUrl]) -> String {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#);

    for url in urls {
        xml.push_str(&format!(
            r#"  <url>
    <loc>{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>{}</changefreq>
    <priority>{:.1}</priority>
  </url>
"#,
            escape_xml(&url.loc),
            url.lastmod.format("%Y-%m-%d"),
            url.changefreq,
            url.priority
        ));
    }

    xml.push_str("</urlset>\n");
    xml
}

fn generate_robots_txt(base_url: &str) -> String {
    format!(
        r#"User-agent: *
Allow: /

Sitemap: {}/sitemap.xml
"#,
        base_url
    )
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn parse_date(date_str: &str) -> Option<DateTime<Utc>> {
    // Parse YYYY-MM-DD format
    chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .ok()
        .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc())
}
