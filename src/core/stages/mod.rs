//! Build Pipeline Stages Module
//!
//! This module implements a stage-based build pipeline for advanced build orchestration.
//! Each stage is a self-contained unit of work that can be composed into pipelines.
//!
//! ## Current Status
//! 
//! The stages are implemented but the `builder.rs` currently uses a direct build approach
//! for simplicity. This module is designed for future extensibility when:
//! - Plugin system needs to hook into specific build stages
//! - Custom pipelines are needed for specialized builds
//! - Parallel stage execution is implemented
//!
//! ## Stage Priority Order
//! 
//! | Stage | Priority | Description |
//! |-------|----------|-------------|
//! | ContentLoader | 10 | Load posts and pages |
//! | TaxonomyBuilder | 20 | Build categories/tags |
//! | RelatedPosts | 30 | Calculate related posts |
//! | Navigation | 35 | Prev/next navigation |
//! | TemplateRenderer | 40 | Render all templates |
//! | AssetProcessor | 50 | Copy and process assets |
//! | SeoGenerator | 60 | Sitemap, feeds, search |
//!
//! ## Future Considerations
//! 
//! - Async stage execution for I/O-bound operations
//! - Stage dependency graph for parallel execution
//! - Plugin-provided custom stages

mod content_loader;
mod taxonomy_builder;
mod related_posts;
mod navigation;
mod template_renderer;
mod asset_processor;
mod seo_generator;

pub use content_loader::ContentLoaderStage;
pub use taxonomy_builder::TaxonomyBuilderStage;
pub use related_posts::RelatedPostsStage;
pub use navigation::NavigationStage;
pub use template_renderer::TemplateRendererStage;
pub use asset_processor::AssetProcessorStage;
pub use seo_generator::SeoGeneratorStage;

use super::build_stage::BuildPipeline;

/// Create a default pipeline with all built-in stages
pub fn create_default_pipeline() -> BuildPipeline {
    let mut pipeline = BuildPipeline::new();
    
    // Stage 1: Load content (priority 10)
    pipeline.add_stage(ContentLoaderStage);
    
    // Stage 2: Build taxonomies (priority 20)
    pipeline.add_stage(TaxonomyBuilderStage);
    
    // Stage 3: Calculate related posts (priority 30)
    pipeline.add_stage(RelatedPostsStage);
    
    // Stage 4: Calculate navigation (priority 35)
    pipeline.add_stage(NavigationStage);
    
    // Stage 5: Render templates (priority 40)
    pipeline.add_stage(TemplateRendererStage);
    
    // Stage 6: Process assets (priority 50)
    pipeline.add_stage(AssetProcessorStage);
    
    // Stage 7: Generate SEO files (priority 60)
    pipeline.add_stage(SeoGeneratorStage);
    
    pipeline
}
