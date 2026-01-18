use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Page, Post};
use crate::core::image_cdn::CdnConfig;

/// Site configuration loaded from explog.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub site: SiteInfo,
    pub build: BuildConfig,
    pub content: ContentConfig,
    #[serde(default)]
    pub seo: SeoConfig,
    #[serde(default)]
    pub cdn: CdnConfig,
    #[serde(default)]
    pub authors: HashMap<String, Author>,
}

/// Navigation menu item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub label: String,
    pub url: String,
    #[serde(default)]
    pub children: Vec<MenuItem>,
}

/// Author information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

/// Social media link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLink {
    pub platform: String,
    pub url: String,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfo {
    pub title: String,
    pub base_url: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub navigation: Vec<MenuItem>,
    #[serde(default)]
    pub socials: Vec<SocialLink>,
}

fn default_language() -> String {
    "en".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    #[serde(default = "default_true")]
    pub minify: bool,
    #[serde(default)]
    pub strict_assets: bool,
}

fn default_theme() -> String {
    "default".to_string()
}

fn default_output_dir() -> String {
    "public".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentConfig {
    #[serde(default = "default_posts_dir")]
    pub posts_dir: String,
    #[serde(default = "default_pages_dir")]
    pub pages_dir: String,
}

fn default_posts_dir() -> String {
    "content/posts".to_string()
}

fn default_pages_dir() -> String {
    "content/pages".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SeoConfig {
    #[serde(default = "default_true")]
    pub generate_sitemap: bool,
    #[serde(default = "default_true")]
    pub generate_rss: bool,
}

fn default_true() -> bool {
    true
}

/// Represents the entire built site
#[derive(Debug, Clone, Serialize)]
pub struct Site {
    /// Site configuration
    pub config: SiteInfo,

    /// All posts
    pub posts: Vec<Post>,

    /// All pages
    pub pages: Vec<Page>,

    /// Categories with post counts
    pub categories: Vec<Category>,

    /// Tags with post counts
    pub tags: Vec<Tag>,

    /// Authors lookup
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub authors: HashMap<String, Author>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Category {
    pub name: String,
    pub slug: String,
    pub url: String,
    pub post_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub name: String,
    pub slug: String,
    pub url: String,
    pub post_count: usize,
}

