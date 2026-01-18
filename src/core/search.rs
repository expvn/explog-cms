use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::info;

use crate::models::Post;

/// Search result item
#[derive(Debug, Clone, Serialize)]
pub struct SearchItem {
    pub title: String,
    pub slug: String,
    pub url: String,
    pub summary: Option<String>,
    pub date: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

/// Head index with popular/recent posts
#[derive(Debug, Serialize)]
pub struct HeadIndex {
    pub version: u32,
    pub total_posts: usize,
    pub items: Vec<SearchItem>,
    pub shards: Vec<String>,
}

/// Shard index for specific letter/category
#[derive(Debug, Serialize)]
pub struct ShardIndex {
    pub shard_type: String,
    pub shard_key: String,
    pub items: Vec<SearchItem>,
}

/// Generate sharded search indices
pub fn generate_search_index(
    posts: &[Post],
    output_dir: &str,
) -> Result<()> {
    let search_dir = Path::new(output_dir).join("api").join("search");
    fs::create_dir_all(&search_dir)?;
    fs::create_dir_all(search_dir.join("shards"))?;

    // Convert posts to search items
    let items: Vec<SearchItem> = posts
        .iter()
        .map(|p| SearchItem {
            title: p.title.clone(),
            slug: p.slug.clone(),
            url: p.url.clone(),
            summary: p.summary.clone(),
            date: p.date.clone(),
            categories: p.categories.clone(),
            tags: p.tags.clone(),
        })
        .collect();

    // 1. Generate head index (top 100 recent posts)
    let head_items: Vec<_> = items.iter().take(100).cloned().collect();
    
    // Collect shard names
    let mut shard_names = Vec::new();

    // 2. Generate alphabet shards (a-z, 0-9, other)
    let mut alpha_shards: HashMap<char, Vec<SearchItem>> = HashMap::new();
    
    for item in &items {
        let first_char = item.title
            .chars()
            .next()
            .unwrap_or('_')
            .to_lowercase()
            .next()
            .unwrap_or('_');
        
        let key = if first_char.is_ascii_alphabetic() {
            first_char
        } else if first_char.is_ascii_digit() {
            '0' // Group all numbers under '0'
        } else {
            '_' // Other characters
        };
        
        alpha_shards
            .entry(key)
            .or_insert_with(Vec::new)
            .push(item.clone());
    }

    // Write alphabet shards
    for (key, shard_items) in &alpha_shards {
        let shard_name = format!("alpha-{}", key);
        shard_names.push(shard_name.clone());
        
        let shard = ShardIndex {
            shard_type: "alpha".to_string(),
            shard_key: key.to_string(),
            items: shard_items.clone(),
        };
        
        let shard_path = search_dir.join("shards").join(format!("{}.json", shard_name));
        let json = serde_json::to_string(&shard)?;
        fs::write(shard_path, json)?;
    }

    // 3. Generate category shards
    let mut category_shards: HashMap<String, Vec<SearchItem>> = HashMap::new();
    
    for item in &items {
        for cat in &item.categories {
            category_shards
                .entry(cat.to_lowercase().replace(' ', "-"))
                .or_insert_with(Vec::new)
                .push(item.clone());
        }
    }

    // Write category shards
    for (key, shard_items) in &category_shards {
        let shard_name = format!("cat-{}", key);
        shard_names.push(shard_name.clone());
        
        let shard = ShardIndex {
            shard_type: "category".to_string(),
            shard_key: key.clone(),
            items: shard_items.clone(),
        };
        
        let shard_path = search_dir.join("shards").join(format!("{}.json", shard_name));
        let json = serde_json::to_string(&shard)?;
        fs::write(shard_path, json)?;
    }

    // 4. Generate year shards
    let mut year_shards: HashMap<String, Vec<SearchItem>> = HashMap::new();
    
    for item in &items {
        let year = item.date.split('-').next().unwrap_or("unknown").to_string();
        year_shards
            .entry(year)
            .or_insert_with(Vec::new)
            .push(item.clone());
    }

    // Write year shards
    for (key, shard_items) in &year_shards {
        let shard_name = format!("year-{}", key);
        shard_names.push(shard_name.clone());
        
        let shard = ShardIndex {
            shard_type: "year".to_string(),
            shard_key: key.clone(),
            items: shard_items.clone(),
        };
        
        let shard_path = search_dir.join("shards").join(format!("{}.json", shard_name));
        let json = serde_json::to_string(&shard)?;
        fs::write(shard_path, json)?;
    }

    // Write head index
    shard_names.sort();
    let head = HeadIndex {
        version: 1,
        total_posts: items.len(),
        items: head_items,
        shards: shard_names.clone(),
    };
    
    let head_path = search_dir.join("index.json");
    let json = serde_json::to_string_pretty(&head)?;
    fs::write(head_path, json)?;

    info!(
        "Generated search index: {} total posts, {} shards",
        items.len(),
        shard_names.len()
    );

    Ok(())
}
