//! Plugin CLI module
//! 
//! Provides CLI commands for plugin management.

use anyhow::Result;
use std::fs;
use std::path::Path;
use tracing::info;

use crate::core::plugin_system::{self, PluginRegistry};

/// List all installed plugins
pub fn list_plugins(plugins_dir: &str) -> Result<()> {
    let mut registry = PluginRegistry::new();
    registry.load_from_dir(plugins_dir)?;
    
    let plugins = registry.plugins();
    
    if plugins.is_empty() {
        println!("No plugins installed.");
        println!("Create a new plugin with: explog plugin new <name>");
        return Ok(());
    }
    
    println!("Installed plugins:\n");
    println!("{:<20} {:<10} {:<40}", "NAME", "VERSION", "DESCRIPTION");
    println!("{}", "-".repeat(70));
    
    for plugin in plugins {
        println!("{:<20} {:<10} {:<40}", 
            plugin.manifest.plugin.name,
            plugin.manifest.plugin.version,
            plugin.manifest.plugin.description.chars().take(40).collect::<String>()
        );
    }
    
    println!("\nTotal: {} plugin(s)", plugins.len());
    
    Ok(())
}

/// Show details of a specific plugin
pub fn show_plugin(name: &str, plugins_dir: &str) -> Result<()> {
    let mut registry = PluginRegistry::new();
    registry.load_from_dir(plugins_dir)?;
    
    match registry.get_plugin(name) {
        Some(plugin) => {
            println!("Plugin: {}\n", plugin.manifest.plugin.name);
            println!("Version:     {}", plugin.manifest.plugin.version);
            println!("Description: {}", plugin.manifest.plugin.description);
            println!("Author:      {}", plugin.manifest.plugin.author);
            println!("API Version: {}", plugin.manifest.plugin.api_version);
            println!("Path:        {}", plugin.path);
            
            println!("\nHooks:");
            if let Some(ref hook) = plugin.manifest.hooks.after_content_load {
                println!("  after_content_load: {}", hook);
            }
            if let Some(ref hook) = plugin.manifest.hooks.before_render {
                println!("  before_render: {}", hook);
            }
            if let Some(ref hook) = plugin.manifest.hooks.after_build {
                println!("  after_build: {}", hook);
            }
            if let Some(ref hook) = plugin.manifest.hooks.on_dev_start {
                println!("  on_dev_start: {}", hook);
            }
            
            if !plugin.manifest.config.is_empty() {
                println!("\nConfiguration:");
                for (key, value) in &plugin.manifest.config {
                    println!("  {}: {:?}", key, value);
                }
            }
        }
        None => {
            println!("Plugin '{}' not found.", name);
        }
    }
    
    Ok(())
}

/// Create a new plugin scaffold
pub fn new_plugin(name: &str, plugins_dir: &str) -> Result<()> {
    // Create plugins directory if it doesn't exist
    fs::create_dir_all(plugins_dir)?;
    
    plugin_system::create_plugin_scaffold(name, plugins_dir)?;
    
    println!("\nPlugin '{}' created successfully!", name);
    println!("\nNext steps:");
    println!("1. Edit plugins/{}/plugin.toml to configure hooks", name);
    println!("2. Write your hook scripts in plugins/{}/scripts/", name);
    println!("3. Test with: explog build");
    
    Ok(())
}

/// Remove a plugin
pub fn remove_plugin(name: &str, plugins_dir: &str) -> Result<()> {
    let plugin_dir = Path::new(plugins_dir).join(name);
    
    if !plugin_dir.exists() {
        println!("Plugin '{}' not found.", name);
        return Ok(());
    }
    
    fs::remove_dir_all(&plugin_dir)?;
    info!("Removed plugin: {}", name);
    println!("Plugin '{}' removed.", name);
    
    Ok(())
}
