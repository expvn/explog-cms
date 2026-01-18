use anyhow::Result;
use tracing::info;

use crate::core::build_stage::{BuildContext, BuildStage};
use crate::core::template::{NavLink, PostNavigation};

/// Stage 4: Calculate prev/next navigation for posts
pub struct NavigationStage;

impl BuildStage for NavigationStage {
    fn name(&self) -> &'static str {
        "Navigation"
    }
    
    fn priority(&self) -> u32 {
        35
    }
    
    fn execute(&self, context: &mut BuildContext) -> Result<()> {
        let posts = context.posts.clone();
        let mut nav_map: Vec<(String, PostNavigation)> = Vec::new();
        
        for (idx, post) in posts.iter().enumerate() {
            let navigation = get_post_navigation(&posts, idx);
            nav_map.push((post.slug.clone(), navigation));
        }
        
        // Store in context for template renderer
        context.set_data("post_navigation", nav_map);
        info!("Calculated navigation for {} posts", posts.len());
        
        Ok(())
    }
}

/// Get previous and next post navigation
fn get_post_navigation(posts: &[crate::models::Post], current_index: usize) -> PostNavigation {
    let prev = if current_index < posts.len() - 1 {
        let p = &posts[current_index + 1];
        Some(NavLink {
            title: p.title.clone(),
            url: p.url.clone(),
        })
    } else {
        None
    };
    
    let next = if current_index > 0 {
        let p = &posts[current_index - 1];
        Some(NavLink {
            title: p.title.clone(),
            url: p.url.clone(),
        })
    } else {
        None
    };
    
    PostNavigation { prev, next }
}
