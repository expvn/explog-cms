use anyhow::Result;
use std::fs;
use std::path::Path;
use tracing::info;

use crate::core::build_stage::{BuildContext, BuildStage};
use crate::core::template::{Pagination, PostNavigation, RelatedPost, TemplateEngine};
use crate::models::site::Site;

/// Stage 5: Render all templates to HTML
pub struct TemplateRendererStage;

impl BuildStage for TemplateRendererStage {
    fn name(&self) -> &'static str {
        "TemplateRenderer"
    }
    
    fn priority(&self) -> u32 {
        40
    }
    
    fn execute(&self, context: &mut BuildContext) -> Result<()> {
        let engine = TemplateEngine::new(&context.theme_dir)?;
        
        // Create Site object for templates
        let site = Site {
            config: context.config.site.clone(),
            posts: context.posts.clone(),
            pages: context.pages.clone(),
            categories: context.categories.clone(),
            tags: context.tags.clone(),
            authors: context.config.authors.clone(),
        };
        
        // Get related posts and navigation from context
        let related_map: Vec<(String, Vec<RelatedPost>)> = context
            .get_data("related_posts")
            .unwrap_or_default();
        let nav_map: Vec<(String, PostNavigation)> = context
            .get_data("post_navigation")
            .unwrap_or_default();
        
        if context.is_full_rebuild {
            self.full_render(&engine, &site, context, &related_map, &nav_map)?;
        } else {
            self.incremental_render(&engine, &site, context, &related_map, &nav_map)?;
        }
        
        Ok(())
    }
}

impl TemplateRendererStage {
    fn full_render(
        &self,
        engine: &TemplateEngine,
        site: &Site,
        context: &mut BuildContext,
        related_map: &[(String, Vec<RelatedPost>)],
        nav_map: &[(String, PostNavigation)],
    ) -> Result<()> {
        let posts = context.posts.clone();
        let pages = context.pages.clone();
        let output_dir = context.output_dir.clone();
        let categories = context.categories.clone();
        let tags = context.tags.clone();
        
        // Pagination settings - read from theme config
        let posts_per_page = crate::core::theme_loader::load_theme_settings(&context.theme_dir)
            .map(|s| s.posts_per_page)
            .unwrap_or(12);
        let total_posts = posts.len();
        let total_pages = (total_posts + posts_per_page - 1) / posts_per_page;
        
        // Render paginated home pages
        for page_num in 1..=total_pages {
            let start = (page_num - 1) * posts_per_page;
            let end = (start + posts_per_page).min(total_posts);
            let page_posts: Vec<_> = posts[start..end].to_vec();
            
            let pagination = Pagination::new(page_num, total_posts, posts_per_page, "");
            let home_html = engine.render_home_page(site, &page_posts, &pagination)?;
            
            if page_num == 1 {
                write_html(&output_dir, "index.html", &home_html)?;
            } else {
                let page_path = format!("page/{}/index.html", page_num);
                write_html(&output_dir, &page_path, &home_html)?;
            }
        }
        info!("Generated {} paginated home pages", total_pages);
        
        // Render all posts
        for (_idx, post) in posts.iter().enumerate() {
            let related = related_map.iter()
                .find(|(slug, _)| slug == &post.slug)
                .map(|(_, r)| r.clone())
                .unwrap_or_default();
            let navigation = nav_map.iter()
                .find(|(slug, _)| slug == &post.slug)
                .map(|(_, n)| n.clone())
                .unwrap_or_else(|| PostNavigation { prev: None, next: None });
            
            let post_html = engine.render_post_with_context(site, post, &related, &navigation)?;
            let post_path = format!("post/{}/index.html", post.slug);
            write_html(&output_dir, &post_path, &post_html)?;
            context.stats.posts_rendered += 1;
        }
        
        // Render all pages
        for page in &pages {
            let page_html = engine.render_page(site, page)?;
            let page_path = format!("{}/index.html", page.slug);
            write_html(&output_dir, &page_path, &page_html)?;
            context.stats.pages_rendered += 1;
        }
        
        // Render categories
        for category in &categories {
            self.render_category(engine, site, &posts, category, &output_dir)?;
            context.stats.categories_rendered += 1;
        }
        
        // Render tags
        for tag in &tags {
            self.render_tag(engine, site, &posts, tag, &output_dir)?;
            context.stats.tags_rendered += 1;
        }
        
        Ok(())
    }
    
    fn incremental_render(
        &self,
        engine: &TemplateEngine,
        site: &Site,
        context: &mut BuildContext,
        related_map: &[(String, Vec<RelatedPost>)],
        nav_map: &[(String, PostNavigation)],
    ) -> Result<()> {
        let posts = context.posts.clone();
        let pages = context.pages.clone();
        let output_dir = context.output_dir.clone();
        let posts_to_rebuild = context.posts_to_rebuild.clone();
        let pages_to_rebuild = context.pages_to_rebuild.clone();
        let categories_to_rebuild = context.categories_to_rebuild.clone();
        let tags_to_rebuild = context.tags_to_rebuild.clone();
        let categories = context.categories.clone();
        let tags = context.tags.clone();
        
        // Render home if needed
        if context.rebuild_home {
            info!("  Rebuilding home page");
            let home_html = engine.render_home(site, &posts)?;
            write_html(&output_dir, "index.html", &home_html)?;
        }
        
        // Render changed posts
        for slug in &posts_to_rebuild {
            if let Some((_idx, post)) = posts.iter().enumerate().find(|(_, p)| &p.slug == slug) {
                info!("  Rebuilding post: {}", slug);
                let related = related_map.iter()
                    .find(|(s, _)| s == slug)
                    .map(|(_, r)| r.clone())
                    .unwrap_or_default();
                let navigation = nav_map.iter()
                    .find(|(s, _)| s == slug)
                    .map(|(_, n)| n.clone())
                    .unwrap_or_else(|| PostNavigation { prev: None, next: None });
                
                let post_html = engine.render_post_with_context(site, post, &related, &navigation)?;
                let post_path = format!("post/{}/index.html", post.slug);
                write_html(&output_dir, &post_path, &post_html)?;
                context.stats.posts_rendered += 1;
            }
        }
        
        // Render changed pages
        for slug in &pages_to_rebuild {
            if let Some(page) = pages.iter().find(|p| &p.slug == slug) {
                info!("  Rebuilding page: {}", slug);
                let page_html = engine.render_page(site, page)?;
                let page_path = format!("{}/index.html", page.slug);
                write_html(&output_dir, &page_path, &page_html)?;
                context.stats.pages_rendered += 1;
            }
        }
        
        // Render affected categories
        for cat_name in &categories_to_rebuild {
            if let Some(category) = categories.iter().find(|c| &c.name == cat_name) {
                info!("  Rebuilding category: {}", cat_name);
                self.render_category(engine, site, &posts, category, &output_dir)?;
                context.stats.categories_rendered += 1;
            }
        }
        
        // Render affected tags
        for tag_name in &tags_to_rebuild {
            if let Some(tag) = tags.iter().find(|t| &t.name == tag_name) {
                info!("  Rebuilding tag: {}", tag_name);
                self.render_tag(engine, site, &posts, tag, &output_dir)?;
                context.stats.tags_rendered += 1;
            }
        }
        
        Ok(())
    }
    
    fn render_category(
        &self,
        engine: &TemplateEngine,
        site: &Site,
        posts: &[crate::models::Post],
        category: &crate::models::site::Category,
        output_dir: &str,
    ) -> Result<()> {
        let cat_posts: Vec<_> = posts
            .iter()
            .filter(|p| p.categories.contains(&category.name))
            .cloned()
            .collect();
        
        let html = engine.render_category(site, &category.name, &cat_posts)?;
        let path = format!("category/{}/index.html", category.slug);
        write_html(output_dir, &path, &html)
    }
    
    fn render_tag(
        &self,
        engine: &TemplateEngine,
        site: &Site,
        posts: &[crate::models::Post],
        tag: &crate::models::site::Tag,
        output_dir: &str,
    ) -> Result<()> {
        let tag_posts: Vec<_> = posts
            .iter()
            .filter(|p| p.tags.contains(&tag.name))
            .cloned()
            .collect();
        
        let html = engine.render_tag(site, &tag.name, &tag_posts)?;
        let path = format!("tag/{}/index.html", tag.slug);
        write_html(output_dir, &path, &html)
    }
}

fn write_html(output_dir: &str, path: &str, content: &str) -> Result<()> {
    let full_path = Path::new(output_dir).join(path);
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&full_path, content)?;
    Ok(())
}
