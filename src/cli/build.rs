use anyhow::Result;
use tracing::info;

use crate::core::{assets, builder, config, sitemap};

/// Selective rebuild options
#[derive(Debug, Clone, Default)]
pub struct RebuildOptions {
    pub page: Option<String>,
    pub post: Option<String>,
    pub category: Option<String>,
    pub tag: Option<String>,
}

impl RebuildOptions {
    pub fn is_selective(&self) -> bool {
        self.page.is_some() || self.post.is_some() || 
        self.category.is_some() || self.tag.is_some()
    }
}

/// Run the build command
pub async fn run(
    clean: bool,
    page: Option<String>,
    post: Option<String>,
    category: Option<String>,
    tag: Option<String>,
) -> Result<()> {
    info!("Starting build...");
    
    // Build rebuild options
    let rebuild_opts = RebuildOptions { page, post, category, tag };
    
    // Load configuration
    let config = config::load_config("explog.toml")?;
    info!("Loaded configuration for: {}", config.site.title);

    // Clean output directory if requested
    if clean {
        info!("Cleaning output directory...");
        std::fs::remove_dir_all(&config.build.output_dir).ok();
        crate::core::cache::BuildCache::clear()?;
    }

    // Check for selective rebuild
    if rebuild_opts.is_selective() {
        info!("Selective rebuild requested");
        
        if let Some(ref slug) = rebuild_opts.page {
            info!("Rebuilding page: {}", slug);
        }
        if let Some(ref slug) = rebuild_opts.post {
            info!("Rebuilding post: {}", slug);
        }
        if let Some(ref name) = rebuild_opts.category {
            info!("Rebuilding category: {}", name);
        }
        if let Some(ref name) = rebuild_opts.tag {
            info!("Rebuilding tag: {}", name);
        }
    }

    // Run the build pipeline (with incremental build support)
    let site = builder::build_site_selective(&config, clean, &rebuild_opts).await?;

    // Copy theme assets (always, in case theme changed)
    assets::copy_theme_assets(&config)?;

    // Generate sitemap and robots.txt
    info!("Generating sitemap.xml and robots.txt...");
    sitemap::generate_sitemap(
        &config,
        &site.posts,
        &site.pages,
        &site.categories,
        &site.tags,
        &config.build.output_dir,
    )?;

    // Generate search index
    info!("Generating search index...");
    crate::core::search::generate_search_index(
        &site.posts,
        &config.build.output_dir,
    )?;

    // Optimize images (convert to WebP)
    info!("Optimizing images...");
    let img_settings = crate::core::images::ImageOptSettings::default();
    crate::core::images::optimize_images(&config.build.output_dir, &img_settings)?;

    info!(
        "Build complete! Generated {} posts, {} pages",
        site.posts.len(),
        site.pages.len()
    );

    Ok(())
}
