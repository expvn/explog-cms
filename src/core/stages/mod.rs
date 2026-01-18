//! Build pipeline stages
//! 
//! Each stage is a self-contained unit of work in the build pipeline.

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
