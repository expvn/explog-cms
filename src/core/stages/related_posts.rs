use anyhow::Result;
use tracing::info;

use crate::core::build_stage::{BuildContext, BuildStage};
use crate::core::template::RelatedPost;
use crate::models::Post;

/// Stage 3: Calculate related posts for each post
pub struct RelatedPostsStage;

impl BuildStage for RelatedPostsStage {
    fn name(&self) -> &'static str {
        "RelatedPosts"
    }
    
    fn priority(&self) -> u32 {
        30
    }
    
    fn execute(&self, context: &mut BuildContext) -> Result<()> {
        let posts = context.posts.clone();
        let mut related_map: Vec<(String, Vec<RelatedPost>)> = Vec::new();
        
        // Read related_posts_count from theme config
        let related_count = crate::core::theme_loader::load_theme_settings(&context.theme_dir)
            .map(|s| s.related_posts_count)
            .unwrap_or(4);
        
        for post in &posts {
            let related = find_related_posts(post, &posts, related_count);
            related_map.push((post.slug.clone(), related));
        }
        
        // Store in context for template renderer
        context.set_data("related_posts", related_map);
        info!("Calculated related posts for {} posts", posts.len());
        
        Ok(())
    }
}

/// Find related posts based on category and tag matches
fn find_related_posts(current: &Post, all_posts: &[Post], max_count: usize) -> Vec<RelatedPost> {
    let mut scored: Vec<_> = all_posts
        .iter()
        .filter(|p| p.slug != current.slug)
        .map(|p| {
            let mut score = 0;
            
            // Score based on shared categories
            for cat in &current.categories {
                if p.categories.contains(cat) {
                    score += 3;
                }
            }
            
            // Score based on shared tags
            for tag in &current.tags {
                if p.tags.contains(tag) {
                    score += 2;
                }
            }
            
            // Score based on explicit related slugs
            if current.related.contains(&p.slug) {
                score += 10;
            }
            
            (p, score)
        })
        .filter(|(_, score)| *score > 0)
        .collect();
    
    // Sort by score descending
    scored.sort_by(|a, b| b.1.cmp(&a.1));
    
    scored
        .into_iter()
        .take(max_count)
        .map(|(p, _)| RelatedPost {
            title: p.title.clone(),
            slug: p.slug.clone(),
            url: p.url.clone(),
            cover: p.cover.clone(),
            date: p.date.clone(),
        })
        .collect()
}
