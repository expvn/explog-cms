//! Theme configuration loader
//! 
//! Parses theme.toml and provides layout/widget configuration to templates.

#![allow(dead_code)]

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Theme configuration parsed from theme.toml
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeConfig {
    pub theme: ThemeMeta,
    #[serde(default)]
    pub layout: HashMap<String, LayoutConfig>,
    #[serde(default)]
    pub widgets: WidgetsConfig,
    #[serde(default)]
    pub settings: ThemeSettings,
}

/// Theme metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeMeta {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub extends: String,
}

/// Layout configuration for a page type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayoutConfig {
    #[serde(default)]
    pub sections: Vec<String>,
    #[serde(default)]
    pub sidebar: bool,
    #[serde(default)]
    pub widgets: Vec<String>,
}

/// Widget configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WidgetsConfig {
    #[serde(default)]
    pub default: Vec<String>,
}

/// Theme-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    #[serde(default = "default_posts_per_page")]
    pub posts_per_page: usize,
    #[serde(default = "default_related_posts_count")]
    pub related_posts_count: usize,
    #[serde(default = "default_true")]
    pub show_reading_time: bool,
    #[serde(default = "default_true")]
    pub show_author: bool,
    #[serde(default = "default_true")]
    pub show_date: bool,
    #[serde(default = "default_true")]
    pub show_categories: bool,
    #[serde(default = "default_true")]
    pub show_tags: bool,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            posts_per_page: 12,
            related_posts_count: 4,
            show_reading_time: true,
            show_author: true,
            show_date: true,
            show_categories: true,
            show_tags: true,
        }
    }
}

fn default_posts_per_page() -> usize {
    12
}

fn default_related_posts_count() -> usize {
    4
}

fn default_true() -> bool {
    true
}

/// Load theme configuration from theme.toml
pub fn load_theme_config<P: AsRef<Path>>(theme_dir: P) -> Result<ThemeConfig> {
    let theme_dir = theme_dir.as_ref();
    let config_path = theme_dir.join("theme.toml");
    
    if !config_path.exists() {
        return Ok(ThemeConfig::default());
    }
    
    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read {}", config_path.display()))?;
    
    let config: ThemeConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", config_path.display()))?;
    
    Ok(config)
}

/// Get layout configuration for a specific page type
pub fn get_layout_config(theme_config: &ThemeConfig, page_type: &str) -> LayoutConfig {
    theme_config.layout.get(page_type)
        .cloned()
        .unwrap_or_else(|| {
            // Default layout if not specified
            LayoutConfig {
                sections: vec!["content".to_string()],
                sidebar: false,
                widgets: theme_config.widgets.default.clone(),
            }
        })
}

/// Get widgets for a specific page type
pub fn get_widgets(theme_config: &ThemeConfig, page_type: &str) -> Vec<String> {
    if let Some(layout) = theme_config.layout.get(page_type) {
        if !layout.widgets.is_empty() {
            return layout.widgets.clone();
        }
    }
    theme_config.widgets.default.clone()
}

/// Load theme settings from theme.toml (convenience function)
pub fn load_theme_settings<P: AsRef<Path>>(theme_dir: P) -> Result<ThemeSettings> {
    let config = load_theme_config(theme_dir)?;
    Ok(config.settings)
}
