use anyhow::Result;
use std::collections::HashMap;
use tracing::info;

use crate::core::build_stage::{BuildContext, BuildStage};
use crate::models::site::{Category, Tag};

/// Stage 2: Build categories and tags from posts
pub struct TaxonomyBuilderStage;

impl BuildStage for TaxonomyBuilderStage {
    fn name(&self) -> &'static str {
        "TaxonomyBuilder"
    }
    
    fn priority(&self) -> u32 {
        20
    }
    
    fn execute(&self, context: &mut BuildContext) -> Result<()> {
        // Build categories
        context.categories = build_categories(&context.posts);
        info!("Built {} categories", context.categories.len());
        
        // Build tags
        context.tags = build_tags(&context.posts);
        info!("Built {} tags", context.tags.len());
        
        Ok(())
    }
}

fn build_categories(posts: &[crate::models::Post]) -> Vec<Category> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    
    for post in posts {
        for cat in &post.categories {
            *counts.entry(cat.clone()).or_insert(0) += 1;
        }
    }
    
    counts
        .into_iter()
        .map(|(name, count)| {
            let slug = slugify(&name);
            Category {
                name,
                slug: slug.clone(),
                url: format!("/category/{}/", slug),
                post_count: count,
            }
        })
        .collect()
}

fn build_tags(posts: &[crate::models::Post]) -> Vec<Tag> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    
    for post in posts {
        for tag in &post.tags {
            *counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }
    
    counts
        .into_iter()
        .map(|(name, count)| {
            let slug = slugify(&name);
            Tag {
                name,
                slug: slug.clone(),
                url: format!("/tag/{}/", slug),
                post_count: count,
            }
        })
        .collect()
}

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' {
                '-'
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
