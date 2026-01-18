#![allow(dead_code)]

use anyhow::Result;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use crate::models::site::{Category, Tag};
use crate::models::{Page, Post, SiteConfig};
use crate::core::cache::BuildCache;

/// Build context shared across all stages
pub struct BuildContext {
    /// Site configuration
    pub config: SiteConfig,
    
    /// All loaded posts
    pub posts: Vec<Post>,
    
    /// All loaded pages
    pub pages: Vec<Page>,
    
    /// Built categories
    pub categories: Vec<Category>,
    
    /// Built tags
    pub tags: Vec<Tag>,
    
    /// Build cache for incremental builds
    pub cache: BuildCache,
    
    /// Output directory
    pub output_dir: String,
    
    /// Theme directory
    pub theme_dir: String,
    
    /// Whether this is a full rebuild
    pub is_full_rebuild: bool,
    
    /// Posts that need rebuilding (for incremental)
    pub posts_to_rebuild: Vec<String>,
    
    /// Pages that need rebuilding (for incremental)
    pub pages_to_rebuild: Vec<String>,
    
    /// Categories that need rebuilding
    pub categories_to_rebuild: Vec<String>,
    
    /// Tags that need rebuilding
    pub tags_to_rebuild: Vec<String>,
    
    /// Whether home page needs rebuild
    pub rebuild_home: bool,
    
    /// Extensible data store for stages to share data
    pub data: HashMap<String, Box<dyn Any + Send + Sync>>,
    
    /// Build statistics
    pub stats: BuildStats,
}

/// Build statistics
#[derive(Debug, Default, Clone)]
pub struct BuildStats {
    pub posts_rendered: usize,
    pub pages_rendered: usize,
    pub categories_rendered: usize,
    pub tags_rendered: usize,
    pub images_optimized: usize,
    pub assets_copied: usize,
}

impl BuildContext {
    /// Create a new build context
    pub fn new(config: SiteConfig) -> Self {
        let output_dir = config.build.output_dir.clone();
        let theme_dir = format!("themes/{}", config.build.theme);
        
        Self {
            config,
            posts: Vec::new(),
            pages: Vec::new(),
            categories: Vec::new(),
            tags: Vec::new(),
            cache: BuildCache::new(),
            output_dir,
            theme_dir,
            is_full_rebuild: false,
            posts_to_rebuild: Vec::new(),
            pages_to_rebuild: Vec::new(),
            categories_to_rebuild: Vec::new(),
            tags_to_rebuild: Vec::new(),
            rebuild_home: false,
            data: HashMap::new(),
            stats: BuildStats::default(),
        }
    }
    
    /// Store data for sharing between stages
    pub fn set_data<T: Any + Send + Sync>(&mut self, key: &str, value: T) {
        self.data.insert(key.to_string(), Box::new(value));
    }
    
    /// Retrieve shared data
    pub fn get_data<T: Any + Send + Sync + Clone>(&self, key: &str) -> Option<T> {
        self.data.get(key)
            .and_then(|v| v.downcast_ref::<T>())
            .cloned()
    }
}

/// A stage in the build pipeline
pub trait BuildStage: Send + Sync {
    /// Name of the stage (for logging)
    fn name(&self) -> &'static str;
    
    /// Priority for ordering (lower = earlier)
    fn priority(&self) -> u32;
    
    /// Execute the stage
    fn execute(&self, context: &mut BuildContext) -> Result<()>;
    
    /// Whether this stage should run on incremental builds
    fn run_on_incremental(&self) -> bool {
        true
    }
}

/// Build pipeline that executes stages in order
pub struct BuildPipeline {
    stages: Vec<Arc<dyn BuildStage>>,
}

impl BuildPipeline {
    /// Create a new empty pipeline
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }
    
    /// Add a stage to the pipeline
    pub fn add_stage<S: BuildStage + 'static>(&mut self, stage: S) {
        self.stages.push(Arc::new(stage));
        self.stages.sort_by_key(|s| s.priority());
    }
    
    /// Remove a stage by name
    pub fn remove_stage(&mut self, name: &str) {
        self.stages.retain(|s| s.name() != name);
    }
    
    /// Execute all stages in order
    pub fn execute(&self, context: &mut BuildContext) -> Result<()> {
        use tracing::info;
        
        for stage in &self.stages {
            // Skip stages that don't run on incremental
            if !context.is_full_rebuild && !stage.run_on_incremental() {
                continue;
            }
            
            info!("Running stage: {}", stage.name());
            stage.execute(context)?;
        }
        
        Ok(())
    }
    
    /// Get stage count
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
}

impl Default for BuildPipeline {
    fn default() -> Self {
        Self::new()
    }
}
