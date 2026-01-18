//! Plugin System Module
//! 
//! Provides plugin manifest parsing, hook registration, and plugin lifecycle management.

#![allow(dead_code)]

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use tracing::{info, warn};

/// Plugin manifest parsed from plugin.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub plugin: PluginMeta,
    #[serde(default)]
    pub hooks: PluginHooks,
    #[serde(default)]
    pub config: HashMap<String, toml::Value>,
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMeta {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default = "default_api_version")]
    pub api_version: String,
    #[serde(default)]
    pub entry: Option<String>,
}

fn default_api_version() -> String {
    "0.2".to_string()
}

/// Plugin hooks configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginHooks {
    /// Script to run after content is loaded
    #[serde(default)]
    pub after_content_load: Option<String>,
    /// Script to run before rendering
    #[serde(default)]
    pub before_render: Option<String>,
    /// Script to run after build completes
    #[serde(default)]
    pub after_build: Option<String>,
    /// Script to run on dev server start
    #[serde(default)]
    pub on_dev_start: Option<String>,
}

/// Hook type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HookType {
    AfterContentLoad,
    BeforeRender,
    AfterBuild,
    OnDevStart,
}

impl std::fmt::Display for HookType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookType::AfterContentLoad => write!(f, "after_content_load"),
            HookType::BeforeRender => write!(f, "before_render"),
            HookType::AfterBuild => write!(f, "after_build"),
            HookType::OnDevStart => write!(f, "on_dev_start"),
        }
    }
}

/// Plugin registry manages all loaded plugins
#[derive(Debug, Default)]
pub struct PluginRegistry {
    plugins: Vec<LoadedPlugin>,
}

/// A loaded plugin with its manifest and path
#[derive(Debug, Clone)]
pub struct LoadedPlugin {
    pub manifest: PluginManifest,
    pub path: String,
    pub enabled: bool,
}

impl PluginRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }
    
    /// Load all plugins from the plugins directory
    pub fn load_from_dir(&mut self, plugins_dir: &str) -> Result<()> {
        let dir = Path::new(plugins_dir);
        if !dir.exists() {
            info!("No plugins directory found at {}", plugins_dir);
            return Ok(());
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                match self.load_plugin(&path) {
                    Ok(plugin) => {
                        info!("Loaded plugin: {} v{}", plugin.manifest.plugin.name, plugin.manifest.plugin.version);
                        self.plugins.push(plugin);
                    }
                    Err(e) => {
                        warn!("Failed to load plugin {:?}: {}", path, e);
                    }
                }
            }
        }
        
        info!("Loaded {} plugins", self.plugins.len());
        Ok(())
    }
    
    /// Load a single plugin from a directory
    fn load_plugin(&self, plugin_dir: &Path) -> Result<LoadedPlugin> {
        let manifest_path = plugin_dir.join("plugin.toml");
        
        if !manifest_path.exists() {
            anyhow::bail!("No plugin.toml found in {:?}", plugin_dir);
        }
        
        let content = fs::read_to_string(&manifest_path)
            .with_context(|| format!("Failed to read {}", manifest_path.display()))?;
        
        let manifest: PluginManifest = toml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", manifest_path.display()))?;
        
        Ok(LoadedPlugin {
            manifest,
            path: plugin_dir.to_string_lossy().to_string(),
            enabled: true,
        })
    }
    
    /// Register a plugin
    pub fn register(&mut self, plugin: LoadedPlugin) {
        self.plugins.push(plugin);
    }
    
    /// Get all loaded plugins
    pub fn plugins(&self) -> &[LoadedPlugin] {
        &self.plugins
    }
    
    /// Get plugins that have a specific hook
    pub fn get_plugins_with_hook(&self, hook: HookType) -> Vec<&LoadedPlugin> {
        self.plugins.iter()
            .filter(|p| p.enabled && p.has_hook(hook))
            .collect()
    }
    
    /// Execute a hook for all plugins that have it
    pub fn execute_hook(&self, hook: HookType, context: &HookContext) -> Result<()> {
        let plugins = self.get_plugins_with_hook(hook);
        
        if plugins.is_empty() {
            return Ok(());
        }
        
        info!("Executing hook '{}' for {} plugins", hook, plugins.len());
        
        for plugin in plugins {
            if let Err(e) = plugin.execute_hook(hook, context) {
                warn!("Plugin '{}' hook '{}' failed: {}", plugin.manifest.plugin.name, hook, e);
            }
        }
        
        Ok(())
    }
    
    /// Get plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&LoadedPlugin> {
        self.plugins.iter().find(|p| p.manifest.plugin.name == name)
    }
    
    /// Remove plugin by name
    pub fn remove_plugin(&mut self, name: &str) -> bool {
        let len_before = self.plugins.len();
        self.plugins.retain(|p| p.manifest.plugin.name != name);
        self.plugins.len() < len_before
    }
}

impl LoadedPlugin {
    /// Check if plugin has a specific hook
    pub fn has_hook(&self, hook: HookType) -> bool {
        match hook {
            HookType::AfterContentLoad => self.manifest.hooks.after_content_load.is_some(),
            HookType::BeforeRender => self.manifest.hooks.before_render.is_some(),
            HookType::AfterBuild => self.manifest.hooks.after_build.is_some(),
            HookType::OnDevStart => self.manifest.hooks.on_dev_start.is_some(),
        }
    }
    
    /// Execute a hook
    pub fn execute_hook(&self, hook: HookType, context: &HookContext) -> Result<()> {
        let script = match hook {
            HookType::AfterContentLoad => self.manifest.hooks.after_content_load.as_ref(),
            HookType::BeforeRender => self.manifest.hooks.before_render.as_ref(),
            HookType::AfterBuild => self.manifest.hooks.after_build.as_ref(),
            HookType::OnDevStart => self.manifest.hooks.on_dev_start.as_ref(),
        };
        
        let script = match script {
            Some(s) => s,
            None => return Ok(()),
        };
        
        let script_path = Path::new(&self.path).join(script);
        
        if !script_path.exists() {
            anyhow::bail!("Hook script not found: {}", script_path.display());
        }
        
        info!("Running hook '{}' for plugin '{}'", hook, self.manifest.plugin.name);
        
        // Execute the script
        let output = Command::new(&script_path)
            .env("EXPLOG_OUTPUT_DIR", &context.output_dir)
            .env("EXPLOG_CONTENT_DIR", &context.content_dir)
            .env("EXPLOG_THEME_DIR", &context.theme_dir)
            .env("EXPLOG_HOOK", hook.to_string())
            .current_dir(&self.path)
            .output()
            .with_context(|| format!("Failed to execute {}", script_path.display()))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Hook script failed: {}", stderr);
        }
        
        Ok(())
    }
}

/// Context passed to hooks
#[derive(Debug, Clone)]
pub struct HookContext {
    pub output_dir: String,
    pub content_dir: String,
    pub theme_dir: String,
}

impl Default for HookContext {
    fn default() -> Self {
        Self {
            output_dir: "public".to_string(),
            content_dir: "content".to_string(),
            theme_dir: "themes/default".to_string(),
        }
    }
}

/// Create a new plugin scaffold
pub fn create_plugin_scaffold(name: &str, plugins_dir: &str) -> Result<()> {
    let plugin_dir = Path::new(plugins_dir).join(name);
    
    if plugin_dir.exists() {
        anyhow::bail!("Plugin '{}' already exists", name);
    }
    
    fs::create_dir_all(&plugin_dir)?;
    
    // Create plugin.toml
    let manifest = format!(r#"[plugin]
name = "{}"
version = "0.1.0"
description = "Description of your plugin"
author = "Your Name"
api_version = "0.2"

[hooks]
# Uncomment hooks you want to use
# after_content_load = "scripts/after_content.sh"
# before_render = "scripts/before_render.sh"
after_build = "scripts/after_build.sh"
# on_dev_start = "scripts/on_dev.sh"

[config]
# Your plugin configuration
# example_option = "value"
"#, name);
    
    fs::write(plugin_dir.join("plugin.toml"), manifest)?;
    
    // Create scripts directory
    let scripts_dir = plugin_dir.join("scripts");
    fs::create_dir_all(&scripts_dir)?;
    
    // Create sample hook script
    #[cfg(windows)]
    let script_content = r#"@echo off
REM Plugin hook script
echo Running plugin hook: %EXPLOG_HOOK%
echo Output dir: %EXPLOG_OUTPUT_DIR%
"#;
    
    #[cfg(not(windows))]
    let script_content = r#"#!/bin/bash
# Plugin hook script
echo "Running plugin hook: $EXPLOG_HOOK"
echo "Output dir: $EXPLOG_OUTPUT_DIR"
"#;
    
    #[cfg(windows)]
    let script_name = "after_build.bat";
    #[cfg(not(windows))]
    let script_name = "after_build.sh";
    
    fs::write(scripts_dir.join(script_name), script_content)?;
    
    // Create README
    let readme = format!(r#"# {} Plugin

## Description

Add your plugin description here.

## Installation

Copy this folder to the `plugins/` directory in your Explog project.

## Configuration

Edit `plugin.toml` to configure the plugin.

## Hooks

This plugin uses the following hooks:
- `after_build`: Runs after the site build completes
"#, name);
    
    fs::write(plugin_dir.join("README.md"), readme)?;
    
    info!("Created plugin scaffold: {}", name);
    info!("  → {}/plugin.toml", plugin_dir.display());
    info!("  → {}/scripts/", plugin_dir.display());
    info!("  → {}/README.md", plugin_dir.display());
    
    Ok(())
}
