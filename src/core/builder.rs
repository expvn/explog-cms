use anyhow::{bail, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

use crate::core::asset_validator;
use crate::core::cache::{self, BuildCache, ChangeSet};
use crate::core::feed;
use crate::core::minifier;
use crate::core::template::{NavLink, PostNavigation, RelatedPost, TemplateEngine};
use crate::core::{content, sitemap};
use crate::models::site::{Category, Site, Tag};
use crate::models::{Post, SiteConfig};
use crate::cli::build::RebuildOptions;

/// Build site with selective rebuild options
pub async fn build_site_selective(config: &SiteConfig, force_clean: bool, opts: &RebuildOptions) -> Result<Site> {
    // If selective rebuild, invalidate cache for specific items
    if opts.is_selective() {
        invalidate_selective_cache(opts)?;
    }
    
    // Call regular build
    build_site(config, force_clean).await
}

/// Invalidate cache entries for selective rebuild
/// This removes cache entries so they appear as "new" on next build
fn invalidate_selective_cache(opts: &RebuildOptions) -> Result<()> {
    use std::fs;
    
    // Load existing cache
    let cache_path = Path::new(".cache/content-hashes.json");
    if !cache_path.exists() {
        info!("No cache to invalidate");
        return Ok(());
    }
    
    let content = fs::read_to_string(cache_path)?;
    let mut cache: serde_json::Value = serde_json::from_str(&content)?;
    
    let mut modified = false;
    
    // Remove page entry from cache
    if let Some(ref slug) = opts.page {
        if let Some(pages) = cache.get_mut("pages") {
            if let Some(pages_obj) = pages.as_object_mut() {
                if pages_obj.remove(slug).is_some() {
                    info!("Removed page '{}' from cache - will rebuild", slug);
                    modified = true;
                } else {
                    info!("Page '{}' not in cache (will be built as new)", slug);
                }
            }
        }
    }
    
    // Remove post entry from cache  
    if let Some(ref slug) = opts.post {
        if let Some(posts) = cache.get_mut("posts") {
            if let Some(posts_obj) = posts.as_object_mut() {
                if posts_obj.remove(slug).is_some() {
                    info!("Removed post '{}' from cache - will rebuild", slug);
                    modified = true;
                } else {
                    info!("Post '{}' not in cache (will be built as new)", slug);
                }
            }
        }
    }
    
    // For category, mark home and affected category for rebuild
    if let Some(ref _name) = opts.category {
        info!("Category rebuild requested - invalidating home page");
        // Remove a field to force home rebuild (theme_hash trick)
        if let Some(theme_hash) = cache.get_mut("theme_hash") {
            *theme_hash = serde_json::Value::String(String::new());
            modified = true;
        }
    }
    
    // For tag, same approach  
    if let Some(ref _name) = opts.tag {
        info!("Tag rebuild requested - invalidating affected pages");
        if let Some(theme_hash) = cache.get_mut("theme_hash") {
            *theme_hash = serde_json::Value::String(String::new());
            modified = true;
        }
    }
    
    // Save modified cache
    if modified {
        let updated = serde_json::to_string_pretty(&cache)?;
        fs::write(cache_path, updated)?;
        info!("Cache updated for selective rebuild");
    }
    
    Ok(())
}

/// Build the entire site (with incremental build support)
pub async fn build_site(config: &SiteConfig, force_clean: bool) -> Result<Site> {
    // Load content
    info!("Loading posts from {}...", config.content.posts_dir);
    let posts = content::load_posts(&config.content.posts_dir)?;
    info!("Loaded {} posts", posts.len());

    info!("Loading pages from {}...", config.content.pages_dir);
    let pages = content::load_pages(&config.content.pages_dir)?;
    info!("Loaded {} pages", pages.len());

    // Validate assets if strict mode is enabled
    if config.build.strict_assets {
        info!("Validating assets (strict mode)...");
        let errors = asset_validator::validate_assets(&posts, &pages);
        if !errors.is_empty() {
            for error in &errors {
                warn!("  {}", error);
            }
            bail!("Strict asset validation failed: {} missing assets", errors.len());
        }
    }

    // Build categories and tags
    let categories = build_categories(&posts);
    let tags = build_tags(&posts);

    // Create site object
    let site = Site {
        config: config.site.clone(),
        posts: posts.clone(),
        pages: pages.clone(),
        categories: categories.clone(),
        tags: tags.clone(),
        authors: config.authors.clone(),
    };

    // Initialize template engine
    let theme_dir = format!("themes/{}", config.build.theme);
    let engine = TemplateEngine::new(&theme_dir)?;
    let output_dir = &config.build.output_dir;

    // Compute current hashes
    let theme_hash = cache::hash_directory(&theme_dir).unwrap_or_default();
    let config_hash = cache::hash_file("explog.toml").unwrap_or_default();

    // Determine what needs rebuilding
    let changes = if force_clean {
        info!("Force clean build requested");
        cache::BuildCache::clear()?;
        ChangeSet::full_rebuild()
    } else {
        let old_cache = BuildCache::load();
        cache::detect_changes(&old_cache, &posts, &pages, &theme_hash, &config_hash)
    };

    // Create output directory
    fs::create_dir_all(output_dir)?;

    // Stats
    let mut posts_rendered = 0;
    let mut pages_rendered = 0;
    let mut categories_rendered = 0;
    let mut tags_rendered = 0;

    // Full rebuild or incremental?
    if changes.is_full_rebuild {
        info!("Performing FULL rebuild...");
        
        // Pagination settings - read from theme config
        let posts_per_page = crate::core::theme_loader::load_theme_settings(&theme_dir)
            .map(|s| s.posts_per_page)
            .unwrap_or(12);
        let total_posts = posts.len();
        let total_pages = (total_posts + posts_per_page - 1) / posts_per_page;
        
        // Render paginated home pages
        for page_num in 1..=total_pages {
            let start = (page_num - 1) * posts_per_page;
            let end = (start + posts_per_page).min(total_posts);
            let page_posts: Vec<_> = posts[start..end].to_vec();
            
            let pagination = crate::core::template::Pagination::new(
                page_num,
                total_posts,
                posts_per_page,
                "",
            );
            
            let home_html = engine.render_home_page(&site, &page_posts, &pagination)?;
            
            if page_num == 1 {
                write_html(output_dir, "index.html", &home_html)?;
            } else {
                let page_path = format!("page/{}/index.html", page_num);
                write_html(output_dir, &page_path, &home_html)?;
            }
        }
        info!("Generated {} paginated home pages", total_pages);

        // Read related_posts_count from theme config
        let related_count = crate::core::theme_loader::load_theme_settings(&theme_dir)
            .map(|s| s.related_posts_count)
            .unwrap_or(4);
        
        // Render all posts with related posts and navigation
        for (idx, post) in posts.iter().enumerate() {
            let related = find_related_posts(post, &posts, related_count);
            let navigation = get_post_navigation(&posts, idx);
            let post_html = engine.render_post_with_context(&site, post, &related, &navigation)?;
            let post_path = format!("post/{}/index.html", post.slug);
            write_html(output_dir, &post_path, &post_html)?;
            
            // Copy post assets (images folder) if exists
            let post_source_dir = format!("{}/{}", config.content.posts_dir, post.slug);
            let post_assets_path = std::path::Path::new(&post_source_dir).join("images");
            if post_assets_path.exists() {
                let post_dest = std::path::Path::new(output_dir).join("post").join(&post.slug);
                copy_assets_only(&std::path::Path::new(&post_source_dir), &post_dest)?;
            }
            
            posts_rendered += 1;
        }

        // Render all pages by mode + type
        for page in &pages {
            use crate::models::{ContentType, PageMode};
            
            match page.mode {
                PageMode::Standalone => {
                    // Standalone: copy entire source folder as-is
                    if let Some(ref source_dir) = page.source_dir {
                        let source = std::path::Path::new(source_dir);
                        let dest = std::path::Path::new(output_dir).join(&page.slug);
                        copy_dir_recursive(source, &dest)?;
                        pages_rendered += 1;
                    }
                }
                PageMode::Embedded => {
                    // Embedded: render with theme layout
                    match page.content_type {
                        ContentType::Gallery => {
                            // Gallery pages use customize template
                            let page_html = engine.render_customize_page(&site, page)?;
                            let page_path = format!("{}/index.html", page.slug);
                            write_html(output_dir, &page_path, &page_html)?;
                            
                            // Copy gallery assets (covers)
                            if let Some(ref source_dir) = page.source_dir {
                                let source = std::path::Path::new(source_dir);
                                let dest = std::path::Path::new(output_dir).join(&page.slug);
                                copy_assets_only(source, &dest)?;
                            }
                        }
                        ContentType::Html => {
                            // HTML pages with embed config use iframe template
                            if page.embed.is_some() {
                                let page_html = engine.render_embed_page(&site, page)?;
                                let page_path = format!("{}/index.html", page.slug);
                                write_html(output_dir, &page_path, &page_html)?;
                                
                                // Copy ENTIRE source folder to embed/ subfolder for iframe
                                if let Some(ref source_dir) = page.source_dir {
                                    let source = std::path::Path::new(source_dir);
                                    let dest = std::path::Path::new(output_dir).join(&page.slug).join("embed");
                                    copy_dir_recursive(source, &dest)?;
                                }
                            } else {
                                // HTML fragments inject directly
                                let page_html = engine.render_page(&site, page)?;
                                let page_path = format!("{}/index.html", page.slug);
                                write_html(output_dir, &page_path, &page_html)?;
                            }
                        }
                        ContentType::Markdown => {
                            // Markdown uses standard page template
                            let page_html = engine.render_page(&site, page)?;
                            let page_path = format!("{}/index.html", page.slug);
                            write_html(output_dir, &page_path, &page_html)?;
                        }
                    }
                    pages_rendered += 1;
                }
            }
        }

        // Render all categories
        for category in &categories {
            render_category(&engine, &site, &posts, category, output_dir)?;
            categories_rendered += 1;
        }

        // Render all tags
        for tag in &tags {
            render_tag(&engine, &site, &posts, tag, output_dir)?;
            tags_rendered += 1;
        }

        // Copy all post assets
        for post in &posts {
            let post_dir = std::path::Path::new(&post.source_path).parent().unwrap_or(std::path::Path::new("."));
            crate::core::assets::copy_post_assets(post_dir, &post.slug, output_dir)?;
        }
    } else if changes.is_empty() {
        info!("No changes detected, nothing to rebuild!");
    } else {
        info!("Performing INCREMENTAL rebuild...");

        // Render home if needed
        if changes.rebuild_home {
            info!("  Rebuilding home page");
            let home_html = engine.render_home(&site, &posts)?;
            write_html(output_dir, "index.html", &home_html)?;
        }

        // Render changed posts with related posts and navigation
        for slug in &changes.posts_to_rebuild {
            if let Some((idx, post)) = posts.iter().enumerate().find(|(_, p)| &p.slug == slug) {
                info!("  Rebuilding post: {}", slug);
                let related_count = crate::core::theme_loader::load_theme_settings(&theme_dir)
                    .map(|s| s.related_posts_count)
                    .unwrap_or(4);
                let related = find_related_posts(post, &posts, related_count);
                let navigation = get_post_navigation(&posts, idx);
                let post_html = engine.render_post_with_context(&site, post, &related, &navigation)?;
                let post_path = format!("post/{}/index.html", post.slug);
                write_html(output_dir, &post_path, &post_html)?;
                
                // Copy assets for this post
                let post_dir = std::path::Path::new(&post.source_path).parent().unwrap_or(std::path::Path::new("."));
                crate::core::assets::copy_post_assets(post_dir, &post.slug, output_dir)?;
                
                posts_rendered += 1;
            }
        }

        // Render changed pages with proper mode/type handling
        for slug in &changes.pages_to_rebuild {
            if let Some(page) = pages.iter().find(|p| &p.slug == slug) {
                info!("  Rebuilding page: {}", slug);
                use crate::models::{ContentType, PageMode};
                
                match page.mode {
                    PageMode::Standalone => {
                        if let Some(ref source_dir) = page.source_dir {
                            let source = std::path::Path::new(source_dir);
                            let dest = std::path::Path::new(output_dir).join(&page.slug);
                            copy_dir_recursive(source, &dest)?;
                        }
                    }
                    PageMode::Embedded => {
                        match page.content_type {
                            ContentType::Gallery => {
                                let page_html = engine.render_customize_page(&site, page)?;
                                let page_path = format!("{}/index.html", page.slug);
                                write_html(output_dir, &page_path, &page_html)?;
                                
                                if let Some(ref source_dir) = page.source_dir {
                                    let source = std::path::Path::new(source_dir);
                                    let dest = std::path::Path::new(output_dir).join(&page.slug);
                                    copy_assets_only(source, &dest)?;
                                }
                            }
                            ContentType::Html => {
                                if page.embed.is_some() {
                                    let page_html = engine.render_embed_page(&site, page)?;
                                    let page_path = format!("{}/index.html", page.slug);
                                    write_html(output_dir, &page_path, &page_html)?;
                                    
                                    if let Some(ref source_dir) = page.source_dir {
                                        let source = std::path::Path::new(source_dir);
                                        let dest = std::path::Path::new(output_dir).join(&page.slug).join("embed");
                                        copy_dir_recursive(source, &dest)?;
                                    }
                                } else {
                                    let page_html = engine.render_page(&site, page)?;
                                    let page_path = format!("{}/index.html", page.slug);
                                    write_html(output_dir, &page_path, &page_html)?;
                                }
                            }
                            ContentType::Markdown => {
                                let page_html = engine.render_page(&site, page)?;
                                let page_path = format!("{}/index.html", page.slug);
                                write_html(output_dir, &page_path, &page_html)?;
                            }
                        }
                    }
                }
                pages_rendered += 1;
            }
        }

        // Render affected categories
        for cat_name in &changes.categories_to_rebuild {
            if let Some(category) = categories.iter().find(|c| &c.name == cat_name) {
                info!("  Rebuilding category: {}", cat_name);
                render_category(&engine, &site, &posts, category, output_dir)?;
                categories_rendered += 1;
            }
        }

        // Render affected tags
        for tag_name in &changes.tags_to_rebuild {
            if let Some(tag) = tags.iter().find(|t| &t.name == tag_name) {
                info!("  Rebuilding tag: {}", tag_name);
                render_tag(&engine, &site, &posts, tag, output_dir)?;
                tags_rendered += 1;
            }
        }
    }

    // Save updated cache
    let new_cache = cache::create_cache(&posts, &pages, theme_hash, config_hash);
    new_cache.save()?;

    // Generate sitemap and feeds
    if config.seo.generate_sitemap {
        sitemap::generate_sitemap(config, &posts, &pages, &categories, &tags, output_dir)?;
        info!("Generated sitemap.xml and robots.txt");
    }

    if config.seo.generate_rss {
        feed::generate_feeds(config, &posts, output_dir)?;
        info!("Generated RSS and Atom feeds");
    }

    // Copy and minify theme assets
    minifier::copy_and_minify_assets(&theme_dir, output_dir, config.build.minify)?;

    // Log stats
    if changes.is_full_rebuild {
        info!(
            "Full build complete! {} posts, {} pages, {} categories, {} tags",
            posts_rendered, pages_rendered, categories_rendered, tags_rendered
        );
    } else {
        info!(
            "Incremental build complete! {} posts, {} pages, {} categories, {} tags rebuilt",
            posts_rendered, pages_rendered, categories_rendered, tags_rendered
        );
    }

    Ok(site)
}

/// Render a category page
fn render_category(
    engine: &TemplateEngine,
    site: &Site,
    posts: &[crate::models::Post],
    category: &Category,
    output_dir: &str,
) -> Result<()> {
    let category_posts: Vec<_> = posts
        .iter()
        .filter(|p| p.categories.contains(&category.name))
        .cloned()
        .collect();
    let category_html = engine.render_category(site, &category.name, &category_posts)?;
    let category_path = format!("category/{}/index.html", category.slug);
    write_html(output_dir, &category_path, &category_html)
}

/// Render a tag page
fn render_tag(
    engine: &TemplateEngine,
    site: &Site,
    posts: &[crate::models::Post],
    tag: &Tag,
    output_dir: &str,
) -> Result<()> {
    let tag_posts: Vec<_> = posts
        .iter()
        .filter(|p| p.tags.contains(&tag.name))
        .cloned()
        .collect();
    let tag_html = engine.render_tag(site, &tag.name, &tag_posts)?;
    let tag_path = format!("tag/{}/index.html", tag.slug);
    write_html(output_dir, &tag_path, &tag_html)
}

/// Write HTML content to a file
fn write_html<P: AsRef<Path>>(output_dir: P, relative_path: &str, content: &str) -> Result<()> {
    let full_path = output_dir.as_ref().join(relative_path);

    // Create parent directories
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&full_path, content)?;
    Ok(())
}

/// Build category index from posts
fn build_categories(posts: &[crate::models::Post]) -> Vec<Category> {
    let mut category_counts: HashMap<String, usize> = HashMap::new();

    for post in posts {
        for category in &post.categories {
            *category_counts.entry(category.clone()).or_insert(0) += 1;
        }
    }

    category_counts
        .into_iter()
        .map(|(name, count)| {
            let slug = slugify(&name);
            Category {
                name: name.clone(),
                slug: slug.clone(),
                url: format!("/category/{}/", slug),
                post_count: count,
            }
        })
        .collect()
}

/// Build tag index from posts
fn build_tags(posts: &[crate::models::Post]) -> Vec<Tag> {
    let mut tag_counts: HashMap<String, usize> = HashMap::new();

    for post in posts {
        for tag in &post.tags {
            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }

    tag_counts
        .into_iter()
        .map(|(name, count)| {
            let slug = slugify(&name);
            Tag {
                name: name.clone(),
                slug: slug.clone(),
                url: format!("/tag/{}/", slug),
                post_count: count,
            }
        })
        .collect()
}

/// Convert text to URL-friendly slug
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

/// Get previous and next post navigation
fn get_post_navigation(posts: &[Post], current_index: usize) -> PostNavigation {
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

/// Recursively copy a directory
fn copy_dir_recursive<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    
    if !src.exists() {
        return Ok(());
    }
    
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

/// Copy only asset files (skip config and content files that are rendered)
fn copy_assets_only<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    
    if !src.exists() {
        return Ok(());
    }
    
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        // Skip content and config files (these are rendered, not copied)
        let filename = src_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if filename == "page.json" || filename == "index.html" || filename == "index.md" {
            continue;
        }
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}
