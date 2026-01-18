use anyhow::Result;
use tracing::info;

use crate::core::build_stage::{BuildContext, BuildStage};
use crate::core::content;

/// Stage 1: Load content from disk
pub struct ContentLoaderStage;

impl BuildStage for ContentLoaderStage {
    fn name(&self) -> &'static str {
        "ContentLoader"
    }
    
    fn priority(&self) -> u32 {
        10
    }
    
    fn execute(&self, context: &mut BuildContext) -> Result<()> {
        // Load posts
        info!("Loading posts from {}...", context.config.content.posts_dir);
        context.posts = content::load_posts(&context.config.content.posts_dir)?;
        info!("Loaded {} posts", context.posts.len());
        
        // Load pages
        info!("Loading pages from {}...", context.config.content.pages_dir);
        context.pages = content::load_pages(&context.config.content.pages_dir)?;
        info!("Loaded {} pages", context.pages.len());
        
        Ok(())
    }
    
    fn run_on_incremental(&self) -> bool {
        true // Always need to load content
    }
}
