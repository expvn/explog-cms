use anyhow::{Context, Result};
use serde::Serialize;
use std::path::Path;
use tera::{Context as TeraContext, Tera};

use crate::models::{Page, Post, Site, SiteConfig};

/// Pagination info for templates
#[derive(Debug, Clone, Serialize)]
pub struct Pagination {
    pub current_page: usize,
    pub total_pages: usize,
    pub total_posts: usize,
    pub per_page: usize,
    pub has_prev: bool,
    pub has_next: bool,
    pub prev_url: Option<String>,
    pub next_url: Option<String>,
}

impl Pagination {
    pub fn new(current_page: usize, total_posts: usize, per_page: usize, base_url: &str) -> Self {
        let total_pages = (total_posts + per_page - 1) / per_page;
        let has_prev = current_page > 1;
        let has_next = current_page < total_pages;
        
        let prev_url = if has_prev {
            if current_page == 2 {
                Some(format!("{}/", base_url))
            } else {
                Some(format!("{}/page/{}/", base_url, current_page - 1))
            }
        } else {
            None
        };
        
        let next_url = if has_next {
            Some(format!("{}/page/{}/", base_url, current_page + 1))
        } else {
            None
        };
        
        Self {
            current_page,
            total_pages,
            total_posts,
            per_page,
            has_prev,
            has_next,
            prev_url,
            next_url,
        }
    }
}

/// Related post info for templates
#[derive(Debug, Clone, Serialize)]
pub struct RelatedPost {
    pub title: String,
    pub slug: String,
    pub url: String,
    pub cover: Option<String>,
    pub date: String,
}

/// Navigation link for prev/next
#[derive(Debug, Clone, Serialize)]
pub struct NavLink {
    pub title: String,
    pub url: String,
}

/// Post navigation (prev/next posts)
#[derive(Debug, Clone, Serialize)]
pub struct PostNavigation {
    pub prev: Option<NavLink>,
    pub next: Option<NavLink>,
}

/// Template engine wrapper around Tera
pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    /// Create a new template engine from theme directory with optional inheritance
    pub fn new<P: AsRef<Path>>(theme_dir: P) -> Result<Self> {
        let theme_dir = theme_dir.as_ref();
        
        // Check for theme.toml to get parent theme
        let theme_config_path = theme_dir.join("theme.toml");
        let mut tera = if theme_config_path.exists() {
            Self::load_with_inheritance(theme_dir)?
        } else {
            let pattern = theme_dir.join("layouts/**/*.html");
            let pattern_str = pattern.to_string_lossy();
            Tera::new(&pattern_str)
                .with_context(|| format!("Failed to load templates from: {}", theme_dir.display()))?
        };
        
        // Register custom filters
        tera.register_filter("slugify", Self::filter_slugify);
        
        Ok(Self { tera })
    }
    
    /// Load theme with inheritance from parent
    fn load_with_inheritance(theme_dir: &Path) -> Result<Tera> {
        use std::fs;
        
        let theme_config_path = theme_dir.join("theme.toml");
        let config_content = fs::read_to_string(&theme_config_path)?;
        
        #[derive(serde::Deserialize, Default)]
        struct ThemeMeta {
            #[serde(default)]
            extends: Option<String>,
        }
        
        #[derive(serde::Deserialize, Default)]
        struct ThemeConfig {
            #[serde(default)]
            theme: ThemeMeta,
        }
        
        let theme_config: ThemeConfig = toml::from_str(&config_content)?;
        
        // Load parent theme first if extends is set
        let mut tera = if let Some(parent_name) = theme_config.theme.extends.filter(|s| !s.is_empty()) {
            let parent_dir = theme_dir.parent().unwrap().join(&parent_name);
            let parent_pattern = parent_dir.join("layouts/**/*.html");
            Tera::new(&parent_pattern.to_string_lossy())
                .with_context(|| format!("Failed to load parent theme: {}", parent_name))?
        } else {
            Tera::default()
        };
        
        // Load child theme templates (override parent)
        let child_pattern = theme_dir.join("layouts/**/*.html");
        let child_tera = Tera::new(&child_pattern.to_string_lossy())
            .with_context(|| format!("Failed to load child theme from: {}", theme_dir.display()))?;
        
        tera.extend(&child_tera)?;
        
        Ok(tera)
    }
    
    /// Slugify filter for templates
    fn filter_slugify(value: &tera::Value, _args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let s = tera::try_get_value!("slugify", "value", String, value);
        let slug = s.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");
        Ok(tera::Value::String(slug))
    }

    /// Render the home page (first page)
    pub fn render_home(&self, site: &Site, posts: &[Post]) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", &site.config);
        context.insert("posts", posts);
        context.insert("categories", &site.categories);
        context.insert("tags", &site.tags);

        self.tera
            .render("home.html", &context)
            .context("Failed to render home template")
    }

    /// Render a paginated home page
    pub fn render_home_page(
        &self,
        site: &Site,
        posts: &[Post],
        pagination: &Pagination,
    ) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", &site.config);
        context.insert("posts", posts);
        context.insert("categories", &site.categories);
        context.insert("tags", &site.tags);
        context.insert("pagination", pagination);

        self.tera
            .render("home.html", &context)
            .context("Failed to render home template")
    }

    /// Render a single post (API method - kept for plugin/extension use)
    #[allow(dead_code)]
    pub fn render_post(&self, site: &Site, post: &Post) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", &site.config);
        context.insert("post", post);
        context.insert("title", &post.title);
        context.insert("content", &post.content);

        self.tera
            .render("post.html", &context)
            .context("Failed to render post template")
    }

    /// Render a post with related posts and navigation context
    pub fn render_post_with_context(
        &self,
        site: &Site,
        post: &Post,
        related: &[RelatedPost],
        navigation: &PostNavigation,
    ) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", &site.config);
        context.insert("post", post);
        context.insert("title", &post.title);
        context.insert("content", &post.content);
        context.insert("related", related);
        context.insert("navigation", navigation);

        self.tera
            .render("post.html", &context)
            .context("Failed to render post template")
    }

    /// Render a static page
    pub fn render_page(&self, site: &Site, page: &Page) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", &site.config);
        context.insert("page", page);
        context.insert("title", &page.title);
        context.insert("content", &page.content);

        self.tera
            .render("page.html", &context)
            .context("Failed to render page template")
    }

    /// Render an embedded HTML page (WebGL, iframe) with theme layout
    pub fn render_embed_page(&self, site: &Site, page: &Page) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", &site.config);
        context.insert("page", page);
        context.insert("title", &page.title);

        // Use page-embed.html template, fallback to page.html
        if self.tera.get_template_names().any(|n| n == "page-embed.html") {
            self.tera
                .render("page-embed.html", &context)
                .context("Failed to render embed page template")
        } else {
            // Fallback: inject content directly
            context.insert("content", &page.content);
            self.tera
                .render("page.html", &context)
                .context("Failed to render embed page (fallback to page.html)")
        }
    }

    /// Render a customize page (gallery with template)
    pub fn render_customize_page(&self, site: &Site, page: &Page) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", &site.config);
        context.insert("page", page);
        context.insert("title", &page.title);
        context.insert("items", &page.items);
        
        if let Some(ref desc) = page.description {
            context.insert("description", desc);
        }

        // Try custom template first, then fallback to customize/default.html, then page.html
        let template_name = page.template
            .as_ref()
            .map(|t| format!("customize/{}.html", t))
            .unwrap_or_else(|| "customize/default.html".to_string());

        if self.tera.get_template_names().any(|n| n == template_name) {
            self.tera
                .render(&template_name, &context)
                .context("Failed to render customize template")
        } else if self.tera.get_template_names().any(|n| n == "customize/default.html") {
            self.tera
                .render("customize/default.html", &context)
                .context("Failed to render customize default template")
        } else {
            // Ultimate fallback to page.html
            self.tera
                .render("page.html", &context)
                .context("Failed to render customize page (using page.html fallback)")
        }
    }

    /// Render a category listing page
    pub fn render_category(
        &self,
        site: &Site,
        category_name: &str,
        posts: &[Post],
    ) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", &site.config);
        context.insert("category", category_name);
        context.insert("posts", posts);
        context.insert("title", &format!("Category: {}", category_name));

        self.tera
            .render("category.html", &context)
            .context("Failed to render category template")
    }

    /// Render a category page with pagination (API method - kept for plugin/extension use)
    #[allow(dead_code)]
    pub fn render_category_page(
        &self,
        site: &Site,
        category_name: &str,
        posts: &[Post],
        pagination: &Pagination,
    ) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", &site.config);
        context.insert("category", category_name);
        context.insert("posts", posts);
        context.insert("title", &format!("Category: {}", category_name));
        context.insert("pagination", pagination);

        self.tera
            .render("category.html", &context)
            .context("Failed to render category template")
    }

    /// Render a tag listing page
    pub fn render_tag(
        &self,
        site: &Site,
        tag_name: &str,
        posts: &[Post],
    ) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", &site.config);
        context.insert("tag", tag_name);
        context.insert("posts", posts);
        context.insert("title", &format!("Tag: {}", tag_name));

        // Use category.html as fallback if tag.html doesn't exist
        if self.tera.get_template_names().any(|n| n == "tag.html") {
            self.tera
                .render("tag.html", &context)
                .context("Failed to render tag template")
        } else {
            // Fallback to category template
            context.insert("category", tag_name);
            self.tera
                .render("category.html", &context)
                .context("Failed to render tag template (using category fallback)")
        }
    }
}

/// Create template context from site config
#[allow(dead_code)]
pub fn create_site_context(config: &SiteConfig) -> TeraContext {
    let mut context = TeraContext::new();
    context.insert("site", &config.site);
    context
}

