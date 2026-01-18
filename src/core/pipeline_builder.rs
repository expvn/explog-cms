//! Pipeline-based site builder
//! 
//! This is an alternative to the original builder that uses the stage-based
//! pipeline architecture for extensibility.

#![allow(dead_code)]

use anyhow::Result;
use std::fs;
use tracing::info;

use crate::core::build_stage::BuildContext;
use crate::core::cache::{self, BuildCache};
use crate::core::stages::create_default_pipeline;
use crate::models::site::Site;
use crate::models::SiteConfig;

/// Build the site using the pipeline architecture
pub async fn build_site_pipeline(config: &SiteConfig, force_clean: bool) -> Result<Site> {
    // Create build context
    let mut context = BuildContext::new(config.clone());
    
    // Determine if full rebuild is needed
    let theme_hash = cache::hash_directory(&context.theme_dir).unwrap_or_default();
    let config_hash = cache::hash_file("explog.toml").unwrap_or_default();
    
    if force_clean {
        info!("Force clean build requested");
        BuildCache::clear()?;
        context.is_full_rebuild = true;
        context.rebuild_home = true;
    } else {
        let old_cache = BuildCache::load();
        let changes = cache::detect_changes(
            &old_cache,
            &[], // Posts not loaded yet
            &[], // Pages not loaded yet
            &theme_hash,
            &config_hash,
        );
        
        // If theme or config changed, do full rebuild
        if changes.is_full_rebuild {
            context.is_full_rebuild = true;
            context.rebuild_home = true;
        } else {
            context.is_full_rebuild = old_cache.posts.is_empty(); // First build
            context.rebuild_home = changes.rebuild_home;
            context.posts_to_rebuild = changes.posts_to_rebuild.into_iter().collect();
            context.pages_to_rebuild = changes.pages_to_rebuild.into_iter().collect();
            context.categories_to_rebuild = changes.categories_to_rebuild.into_iter().collect();
            context.tags_to_rebuild = changes.tags_to_rebuild.into_iter().collect();
        }
    }
    
    // Create output directory
    fs::create_dir_all(&context.output_dir)?;
    
    // Create and execute pipeline
    let pipeline = create_default_pipeline();
    info!("Executing build pipeline with {} stages", pipeline.stage_count());
    
    pipeline.execute(&mut context)?;
    
    // Save updated cache
    let theme_hash = cache::hash_directory(&context.theme_dir).unwrap_or_default();
    let config_hash = cache::hash_file("explog.toml").unwrap_or_default();
    let new_cache = cache::create_cache(&context.posts, &context.pages, theme_hash, config_hash);
    new_cache.save()?;
    
    // Log stats
    let stats = &context.stats;
    if context.is_full_rebuild {
        info!(
            "Full build complete! {} posts, {} pages, {} categories, {} tags",
            stats.posts_rendered, stats.pages_rendered, 
            stats.categories_rendered, stats.tags_rendered
        );
    } else {
        info!(
            "Incremental build complete! {} posts, {} pages, {} categories, {} tags rebuilt",
            stats.posts_rendered, stats.pages_rendered,
            stats.categories_rendered, stats.tags_rendered
        );
    }
    
    // Return site object
    Ok(Site {
        config: context.config.site.clone(),
        posts: context.posts,
        pages: context.pages,
        categories: context.categories,
        tags: context.tags,
        authors: context.config.authors,
    })
}
