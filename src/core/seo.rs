//! SEO Module - Public API for template SEO injection
//! These functions are intentionally public for plugin/extension use
#![allow(dead_code)]

use serde::Serialize;
use crate::models::{Post, Page};
use crate::models::site::SiteConfig;

/// SEO metadata for templates
#[derive(Debug, Clone, Serialize)]
pub struct SeoMeta {
    pub canonical_url: String,
    pub og_title: String,
    pub og_description: String,
    pub og_type: String,
    pub og_image: Option<String>,
    pub structured_data: String,
}

/// Generate SEO metadata for a post
pub fn generate_post_seo(post: &Post, config: &SiteConfig) -> SeoMeta {
    let canonical_url = format!("/post/{}/", post.slug);
    let og_image = post.cover.clone().map(|c| {
        if c.starts_with("http") {
            c
        } else {
            format!("{}/post/{}/{}", config.site.base_url, post.slug, c)
        }
    });
    
    SeoMeta {
        canonical_url: canonical_url.clone(),
        og_title: post.title.clone(),
        og_description: post.summary.clone().unwrap_or_default(),
        og_type: "article".to_string(),
        og_image,
        structured_data: generate_article_jsonld(post, config),
    }
}

/// Generate SEO metadata for a page
pub fn generate_page_seo(page: &Page, config: &SiteConfig) -> SeoMeta {
    let canonical_url = format!("/{}/", page.slug);
    
    SeoMeta {
        canonical_url,
        og_title: page.title.clone(),
        og_description: page.description.clone().unwrap_or_default(),
        og_type: "website".to_string(),
        og_image: None,
        structured_data: generate_website_jsonld(config),
    }
}

/// Generate SEO metadata for homepage
pub fn generate_home_seo(config: &SiteConfig) -> SeoMeta {
    SeoMeta {
        canonical_url: "/".to_string(),
        og_title: config.site.title.clone(),
        og_description: config.site.description.clone(),
        og_type: "website".to_string(),
        og_image: Some(format!("{}/assets/og-default.png", config.site.base_url)),
        structured_data: generate_website_jsonld(config),
    }
}

/// Generate Article schema JSON-LD for blog posts
pub fn generate_article_jsonld(post: &Post, config: &SiteConfig) -> String {
    let image_url = post.cover.clone().map(|c| {
        if c.starts_with("http") {
            c
        } else {
            format!("{}/post/{}/{}", config.site.base_url, post.slug, c)
        }
    }).unwrap_or_else(|| format!("{}/assets/og-default.png", config.site.base_url));
    
    let author_name = if post.author.is_empty() { "Admin".to_string() } else { post.author.clone() };
    
    format!(r#"<script type="application/ld+json">
{{
  "@context": "https://schema.org",
  "@type": "BlogPosting",
  "headline": "{}",
  "description": "{}",
  "image": "{}",
  "author": {{
    "@type": "Person",
    "name": "{}"
  }},
  "publisher": {{
    "@type": "Organization",
    "name": "{}",
    "logo": {{
      "@type": "ImageObject",
      "url": "{}/assets/favicon.ico"
    }}
  }},
  "datePublished": "{}",
  "dateModified": "{}",
  "mainEntityOfPage": {{
    "@type": "WebPage",
    "@id": "{}/post/{}/"
  }}
}}
</script>"#,
        escape_json(&post.title),
        escape_json(&post.summary.clone().unwrap_or_default()),
        image_url,
        escape_json(&author_name),
        escape_json(&config.site.title),
        config.site.base_url,
        post.date,
        post.date,
        config.site.base_url,
        post.slug
    )
}

/// Generate Website schema JSON-LD
pub fn generate_website_jsonld(config: &SiteConfig) -> String {
    format!(r#"<script type="application/ld+json">
{{
  "@context": "https://schema.org",
  "@type": "WebSite",
  "name": "{}",
  "description": "{}",
  "url": "{}",
  "potentialAction": {{
    "@type": "SearchAction",
    "target": "{}/search?q={{search_term_string}}",
    "query-input": "required name=search_term_string"
  }}
}}
</script>"#,
        escape_json(&config.site.title),
        escape_json(&config.site.description),
        config.site.base_url,
        config.site.base_url
    )
}

/// Generate BreadcrumbList schema JSON-LD
#[allow(dead_code)]
pub fn generate_breadcrumb_jsonld(breadcrumbs: &[(&str, &str)], config: &SiteConfig) -> String {
    let items: Vec<String> = breadcrumbs.iter().enumerate().map(|(i, (name, url))| {
        format!(r#"{{
      "@type": "ListItem",
      "position": {},
      "name": "{}",
      "item": "{}{}"
    }}"#, i + 1, escape_json(name), config.site.base_url, url)
    }).collect();
    
    format!(r#"<script type="application/ld+json">
{{
  "@context": "https://schema.org",
  "@type": "BreadcrumbList",
  "itemListElement": [
    {}
  ]
}}
</script>"#, items.join(",\n    "))
}

/// Escape special characters for JSON
fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_escape_json() {
        assert_eq!(escape_json("Hello \"World\""), "Hello \\\"World\\\"");
    }
}
