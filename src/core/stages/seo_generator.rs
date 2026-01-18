use anyhow::Result;
use tracing::info;

use crate::core::build_stage::{BuildContext, BuildStage};
use crate::core::{feed, search, sitemap};

/// Stage 7: Generate SEO files (sitemap, feeds, search index)
pub struct SeoGeneratorStage;

impl BuildStage for SeoGeneratorStage {
    fn name(&self) -> &'static str {
        "SeoGenerator"
    }
    
    fn priority(&self) -> u32 {
        60
    }
    
    fn execute(&self, context: &mut BuildContext) -> Result<()> {
        let config = &context.config;
        let posts = &context.posts;
        let pages = &context.pages;
        let categories = &context.categories;
        let tags = &context.tags;
        let output_dir = &context.output_dir;
        
        // Generate sitemap
        if config.seo.generate_sitemap {
            sitemap::generate_sitemap(config, posts, pages, categories, tags, output_dir)?;
            info!("Generated sitemap.xml and robots.txt");
        }
        
        // Generate RSS/Atom feeds
        if config.seo.generate_rss {
            feed::generate_feeds(config, posts, output_dir)?;
            info!("Generated RSS and Atom feeds");
        }
        
        // Generate search index
        search::generate_search_index(posts, output_dir)?;
        info!("Generated sharded search index");
        
        Ok(())
    }
    
    fn run_on_incremental(&self) -> bool {
        true // Always regenerate SEO files
    }
}
