//! Scheduling Module
//! 
//! Handles scheduled publishing and draft preview functionality.

#![allow(dead_code)]

use chrono::{DateTime, Utc};
use rand::Rng;
use tracing::info;

use crate::models::Post;

/// Check if a post should be published based on its publish_date
pub fn is_published(post: &Post) -> bool {
    // If draft, not published
    if post.draft {
        return false;
    }
    
    // If no publish_date, it's immediately published
    let publish_date = match &post.publish_date {
        Some(date) => date,
        None => return true,
    };
    
    // Parse the publish date
    let scheduled_time = match parse_datetime(publish_date) {
        Ok(dt) => dt,
        Err(_) => {
            // If can't parse, treat as published
            return true;
        }
    };
    
    // Check if current time is past the scheduled time
    Utc::now() >= scheduled_time
}

/// Filter posts to only include those that should be visible
pub fn filter_published_posts(posts: Vec<Post>, include_scheduled: bool) -> Vec<Post> {
    if include_scheduled {
        // Include all non-draft posts (ignore publish_date)
        posts.into_iter()
            .filter(|p| !p.draft)
            .collect()
    } else {
        // Only include posts that are published now
        posts.into_iter()
            .filter(|p| is_published(p))
            .collect()
    }
}

/// Get scheduled posts (publish_date in future)
pub fn get_scheduled_posts(posts: &[Post]) -> Vec<&Post> {
    let now = Utc::now();
    
    posts.iter()
        .filter(|p| !p.draft)
        .filter(|p| {
            if let Some(ref date) = p.publish_date {
                if let Ok(dt) = parse_datetime(date) {
                    return dt > now;
                }
            }
            false
        })
        .collect()
}

/// Generate a preview token for a draft post
pub fn generate_preview_token() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 16] = rng.gen();
    hex::encode(bytes)
}

/// Verify a preview token matches
pub fn verify_preview_token(post: &Post, token: &str) -> bool {
    match &post.preview_token {
        Some(t) => t == token,
        None => false,
    }
}

/// Get draft posts with preview tokens
pub fn get_previewable_drafts(posts: &[Post]) -> Vec<(&Post, &str)> {
    posts.iter()
        .filter(|p| p.draft)
        .filter_map(|p| {
            p.preview_token.as_ref().map(|token| (p, token.as_str()))
        })
        .collect()
}

/// Find a draft post by its preview token
pub fn find_by_preview_token<'a>(posts: &'a [Post], token: &str) -> Option<&'a Post> {
    posts.iter()
        .filter(|p| p.draft)
        .find(|p| {
            p.preview_token.as_ref().map_or(false, |t| t == token)
        })
}

/// Parse a datetime string in various formats
fn parse_datetime(s: &str) -> anyhow::Result<DateTime<Utc>> {
    // Try ISO 8601 with timezone
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }
    
    // Try ISO 8601 without timezone (assume UTC)
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Ok(DateTime::from_naive_utc_and_offset(dt, Utc));
    }
    
    // Try date only (midnight UTC)
    if let Ok(date) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        let dt = date.and_hms_opt(0, 0, 0).unwrap();
        return Ok(DateTime::from_naive_utc_and_offset(dt, Utc));
    }
    
    anyhow::bail!("Invalid date format: {}", s)
}

/// Print scheduling summary
pub fn print_scheduling_summary(posts: &[Post]) {
    let total = posts.len();
    let published: Vec<_> = posts.iter().filter(|p| is_published(p)).collect();
    let scheduled = get_scheduled_posts(posts);
    let drafts: Vec<_> = posts.iter().filter(|p| p.draft).collect();
    
    info!("Content summary:");
    info!("  Total posts: {}", total);
    info!("  Published: {}", published.len());
    info!("  Scheduled: {}", scheduled.len());
    info!("  Drafts: {}", drafts.len());
    
    if !scheduled.is_empty() {
        info!("Scheduled posts:");
        for post in scheduled {
            info!("  - {} (publish: {})", post.title, post.publish_date.as_ref().unwrap_or(&"unknown".into()));
        }
    }
}
