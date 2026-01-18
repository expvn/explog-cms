use serde::{Deserialize, Serialize};

/// Represents a blog post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    /// Post title
    pub title: String,

    /// URL-friendly slug
    pub slug: String,

    /// Publication date (YYYY-MM-DD)
    pub date: String,

    /// HTML content (rendered from Markdown)
    pub content: String,

    /// Raw markdown content
    #[serde(skip_serializing)]
    pub raw_content: String,

    /// Post summary/excerpt
    pub summary: Option<String>,

    /// Categories
    #[serde(default)]
    pub categories: Vec<String>,

    /// Tags
    #[serde(default)]
    pub tags: Vec<String>,

    /// Cover image path
    pub cover: Option<String>,

    /// Is featured post
    #[serde(default)]
    pub featured: bool,

    /// Is draft (not published)
    #[serde(default)]
    pub draft: bool,

    /// Author ID
    #[serde(default = "default_author")]
    pub author: String,

    /// SEO metadata
    pub seo: Option<SeoMeta>,

    /// Related post slugs
    #[serde(default)]
    pub related: Vec<String>,

    /// Relative URL path
    pub url: String,

    /// Source file path
    #[serde(skip)]
    pub source_path: String,

    /// Scheduled publish date (ISO 8601 format)
    /// Post won't be published until this date
    #[serde(default)]
    pub publish_date: Option<String>,

    /// Preview token for draft posts
    /// Access draft via /preview/{token}
    #[serde(default)]
    pub preview_token: Option<String>,
}

fn default_author() -> String {
    "admin".to_string()
}

/// SEO metadata for a post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoMeta {
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub noindex: bool,
}

/// Frontmatter for a post (parsed from YAML)
#[derive(Debug, Deserialize)]
pub struct PostFrontmatter {
    pub title: String,
    pub date: Option<String>,
    pub slug: Option<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub cover: Option<String>,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub draft: bool,
    #[serde(default = "default_author")]
    pub author: String,
    pub seo: Option<SeoMeta>,
    #[serde(default)]
    pub related: Vec<String>,
    /// Scheduled publish date (ISO 8601)
    pub publish_date: Option<String>,
    /// Preview token for drafts
    pub preview_token: Option<String>,
}
