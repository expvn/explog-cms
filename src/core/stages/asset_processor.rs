use anyhow::Result;
use tracing::info;

use crate::core::build_stage::{BuildContext, BuildStage};
use crate::core::{assets, images, minifier};

/// Stage 6: Process and copy assets
pub struct AssetProcessorStage;

impl BuildStage for AssetProcessorStage {
    fn name(&self) -> &'static str {
        "AssetProcessor"
    }
    
    fn priority(&self) -> u32 {
        50
    }
    
    fn execute(&self, context: &mut BuildContext) -> Result<()> {
        let output_dir = &context.output_dir;
        let theme_dir = &context.theme_dir;
        let minify = context.config.build.minify;
        
        // Copy and optionally minify theme assets
        minifier::copy_and_minify_assets(theme_dir, output_dir, minify)?;
        info!("Copied theme assets (minify={})", minify);
        
        // Copy post assets
        for post in &context.posts {
            let post_dir = std::path::Path::new(&post.source_path).parent().unwrap_or(std::path::Path::new("."));
            assets::copy_post_assets(post_dir, &post.slug, output_dir)?;
        }
        info!("Copied assets for {} posts", context.posts.len());
        
        // Optimize images (WebP conversion)
        let img_settings = images::ImageOptSettings::default();
        let stats = images::optimize_images(output_dir, &img_settings)?;
        context.stats.images_optimized = stats.processed;
        
        if stats.processed > 0 {
            info!("Optimized {} images", stats.processed);
        }
        
        Ok(())
    }
    
    fn run_on_incremental(&self) -> bool {
        true // Always process assets
    }
}
