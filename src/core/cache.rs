use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use tracing::info;

const CACHE_VERSION: u32 = 1;
const CACHE_FILE: &str = ".cache/content-hashes.json";

/// Cache for incremental builds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildCache {
    pub version: u32,
    pub posts: HashMap<String, PostCacheEntry>,
    pub pages: HashMap<String, PageCacheEntry>,
    pub theme_hash: String,
    pub config_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCacheEntry {
    pub hash: String,
    pub title: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub featured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageCacheEntry {
    pub hash: String,
    pub title: String,
}

impl BuildCache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self {
            version: CACHE_VERSION,
            posts: HashMap::new(),
            pages: HashMap::new(),
            theme_hash: String::new(),
            config_hash: String::new(),
        }
    }

    /// Load cache from disk, or return empty cache if not found
    pub fn load() -> Self {
        if let Ok(content) = fs::read_to_string(CACHE_FILE) {
            if let Ok(cache) = serde_json::from_str::<BuildCache>(&content) {
                if cache.version == CACHE_VERSION {
                    info!("Loaded build cache with {} posts, {} pages", 
                          cache.posts.len(), cache.pages.len());
                    return cache;
                }
                info!("Cache version mismatch, starting fresh");
            }
        }
        info!("No valid cache found, starting fresh build");
        Self::new()
    }

    /// Save cache to disk
    pub fn save(&self) -> Result<()> {
        let cache_dir = Path::new(".cache");
        fs::create_dir_all(cache_dir)?;
        
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize cache")?;
        fs::write(CACHE_FILE, content)
            .context("Failed to write cache file")?;
        
        info!("Saved build cache");
        Ok(())
    }

    /// Clear the cache
    pub fn clear() -> Result<()> {
        if Path::new(CACHE_FILE).exists() {
            fs::remove_file(CACHE_FILE)?;
        }
        Ok(())
    }
}

/// Result of comparing old and new content
#[derive(Debug, Default)]
pub struct ChangeSet {
    /// Posts that need to be rebuilt
    pub posts_to_rebuild: HashSet<String>,
    /// Pages that need to be rebuilt
    pub pages_to_rebuild: HashSet<String>,
    /// Categories that need to be rebuilt
    pub categories_to_rebuild: HashSet<String>,
    /// Tags that need to be rebuilt
    pub tags_to_rebuild: HashSet<String>,
    /// Whether home page needs rebuild
    pub rebuild_home: bool,
    /// Whether this is a full rebuild
    pub is_full_rebuild: bool,
}

impl ChangeSet {
    pub fn full_rebuild() -> Self {
        Self {
            is_full_rebuild: true,
            rebuild_home: true,
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.is_full_rebuild 
            && self.posts_to_rebuild.is_empty()
            && self.pages_to_rebuild.is_empty()
            && self.categories_to_rebuild.is_empty()
            && self.tags_to_rebuild.is_empty()
            && !self.rebuild_home
    }
}

/// Compute SHA-256 hash of file content
pub fn hash_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let content = fs::read(path.as_ref())
        .with_context(|| format!("Failed to read file: {}", path.as_ref().display()))?;
    Ok(hash_bytes(&content))
}

/// Compute SHA-256 hash of bytes
pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Compute hash of a directory (all files combined)
pub fn hash_directory<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(String::new());
    }

    let mut hasher = Sha256::new();
    let mut entries: Vec<_> = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();
    
    // Sort for deterministic hashing
    entries.sort_by(|a, b| a.path().cmp(b.path()));
    
    for entry in entries {
        let content = fs::read(entry.path())?;
        hasher.update(&content);
    }
    
    Ok(format!("{:x}", hasher.finalize()))
}

/// Detect changes between old cache and current posts/pages
pub fn detect_changes(
    old_cache: &BuildCache,
    posts: &[crate::models::Post],
    pages: &[crate::models::Page],
    theme_hash: &str,
    config_hash: &str,
) -> ChangeSet {
    let mut changes = ChangeSet::default();

    // If theme or config changed, full rebuild
    if old_cache.theme_hash != theme_hash || old_cache.config_hash != config_hash {
        info!("Theme or config changed, doing full rebuild");
        return ChangeSet::full_rebuild();
    }

    // Check each post
    for post in posts {
        let post_hash = hash_bytes(post.raw_content.as_bytes());
        
        if let Some(cached) = old_cache.posts.get(&post.slug) {
            // Post exists in cache
            if cached.hash != post_hash {
                // Content changed
                info!("Post changed: {}", post.slug);
                changes.posts_to_rebuild.insert(post.slug.clone());
                
                // If title changed, also rebuild listings
                if cached.title != post.title {
                    changes.rebuild_home = true;
                    for cat in &post.categories {
                        changes.categories_to_rebuild.insert(cat.clone());
                    }
                }

                // If categories changed, rebuild old and new categories
                let old_cats: HashSet<_> = cached.categories.iter().collect();
                let new_cats: HashSet<_> = post.categories.iter().collect();
                if old_cats != new_cats {
                    changes.rebuild_home = true;
                    for cat in old_cats.union(&new_cats) {
                        changes.categories_to_rebuild.insert((*cat).clone());
                    }
                }

                // Same for tags
                let old_tags: HashSet<_> = cached.tags.iter().collect();
                let new_tags: HashSet<_> = post.tags.iter().collect();
                if old_tags != new_tags {
                    for tag in old_tags.union(&new_tags) {
                        changes.tags_to_rebuild.insert((*tag).clone());
                    }
                }

                // If featured status changed
                if cached.featured != post.featured {
                    changes.rebuild_home = true;
                }
            }
        } else {
            // New post
            info!("New post: {}", post.slug);
            changes.posts_to_rebuild.insert(post.slug.clone());
            changes.rebuild_home = true;
            for cat in &post.categories {
                changes.categories_to_rebuild.insert(cat.clone());
            }
            for tag in &post.tags {
                changes.tags_to_rebuild.insert(tag.clone());
            }
        }
    }

    // Check for deleted posts
    for (slug, cached) in &old_cache.posts {
        if !posts.iter().any(|p| &p.slug == slug) {
            info!("Post deleted: {}", slug);
            changes.rebuild_home = true;
            for cat in &cached.categories {
                changes.categories_to_rebuild.insert(cat.clone());
            }
            for tag in &cached.tags {
                changes.tags_to_rebuild.insert(tag.clone());
            }
        }
    }

    // Check each page
    for page in pages {
        let page_hash = hash_bytes(page.content.as_bytes());
        
        if let Some(cached) = old_cache.pages.get(&page.slug) {
            if cached.hash != page_hash {
                info!("Page changed: {}", page.slug);
                changes.pages_to_rebuild.insert(page.slug.clone());
            }
        } else {
            info!("New page: {}", page.slug);
            changes.pages_to_rebuild.insert(page.slug.clone());
        }
    }

    changes
}

/// Create new cache from current content
pub fn create_cache(
    posts: &[crate::models::Post],
    pages: &[crate::models::Page],
    theme_hash: String,
    config_hash: String,
) -> BuildCache {
    let mut cache = BuildCache::new();
    cache.theme_hash = theme_hash;
    cache.config_hash = config_hash;

    for post in posts {
        cache.posts.insert(post.slug.clone(), PostCacheEntry {
            hash: hash_bytes(post.raw_content.as_bytes()),
            title: post.title.clone(),
            categories: post.categories.clone(),
            tags: post.tags.clone(),
            featured: post.featured,
        });
    }

    for page in pages {
        cache.pages.insert(page.slug.clone(), PageCacheEntry {
            hash: hash_bytes(page.content.as_bytes()),
            title: page.title.clone(),
        });
    }

    cache
}
