use serde::{Deserialize, Serialize};

/// Content type - what the page content is
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    #[default]
    Markdown,
    Html,
    Gallery,
}

/// Page mode - how to display the page
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PageMode {
    #[default]
    Embedded,   // Uses theme layout (header/footer)
    Standalone, // Independent, no theme
}

/// Represents a static page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// Page title
    pub title: String,

    /// URL-friendly slug
    pub slug: String,

    /// HTML content (rendered from Markdown or raw HTML)
    pub content: String,

    /// Relative URL path
    pub url: String,

    /// Content type (markdown, html, gallery)
    #[serde(default)]
    pub content_type: ContentType,

    /// Display mode (embedded, standalone)
    #[serde(default)]
    pub mode: PageMode,

    /// Template name override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    /// Description/summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Embed config (for html type with iframe)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<EmbedConfig>,

    /// Gallery items (for gallery type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<GalleryItem>>,

    /// Source directory path
    #[serde(skip)]
    pub source_dir: Option<String>,

    /// Source file path
    #[serde(skip)]
    pub source_path: String,
}

/// Gallery item for gallery pages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalleryItem {
    pub cover: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub url: String,
}

/// Embed config for HTML pages with iframe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedConfig {
    #[serde(default)]
    pub src: Option<String>,
    #[serde(default)]
    pub width: Option<String>,
    #[serde(default)]
    pub height: Option<String>,
}

/// Config for page.json files
#[derive(Debug, Deserialize)]
pub struct PageJsonConfig {
    pub title: String,
    
    #[serde(default)]
    pub description: Option<String>,
    
    /// Content type: markdown, html, gallery
    #[serde(rename = "type", default)]
    pub content_type: Option<String>,
    
    /// Display mode: embedded, standalone
    #[serde(default)]
    pub mode: Option<String>,
    
    #[serde(default)]
    pub template: Option<String>,
    
    #[serde(default)]
    pub embed: Option<EmbedConfig>,
    
    /// Gallery items
    #[serde(default)]
    pub items: Option<Vec<GalleryItem>>,
    
    #[serde(default)]
    pub slug: Option<String>,
}
