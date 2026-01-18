use anyhow::Result;
use serde::Serialize;
use std::fs;
use std::path::Path;
use tracing::info;

use crate::models::{Post, Page};
use crate::models::site::SiteConfig;

/// SEO issue severity
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Single SEO issue
#[derive(Debug, Clone, Serialize)]
pub struct SeoIssue {
    pub severity: Severity,
    pub category: String,
    pub message: String,
    pub url: String,
    pub suggestion: String,
}

/// SEO analysis for a single page/post
#[derive(Debug, Clone, Serialize)]
pub struct PageAnalysis {
    pub url: String,
    pub title: String,
    pub title_length: usize,
    pub description_length: usize,
    pub has_h1: bool,
    pub image_count: usize,
    pub images_without_alt: usize,
    pub word_count: usize,
    pub issues: Vec<SeoIssue>,
    pub score: u32,
}

/// Overall SEO report
#[derive(Debug, Clone, Serialize)]
pub struct SeoReport {
    pub generated_at: String,
    pub site_url: String,
    pub total_pages: usize,
    pub total_posts: usize,
    pub total_issues: usize,
    pub errors: usize,
    pub warnings: usize,
    pub average_score: u32,
    pub pages: Vec<PageAnalysis>,
    pub posts: Vec<PageAnalysis>,
}

/// Analyze a post for SEO issues
pub fn analyze_post(post: &Post, _config: &SiteConfig) -> PageAnalysis {
    let mut issues = Vec::new();
    let url = format!("/post/{}/", post.slug);
    
    // Title analysis
    let title_length = post.title.chars().count();
    if title_length < 30 {
        issues.push(SeoIssue {
            severity: Severity::Warning,
            category: "Title".to_string(),
            message: format!("Title too short: {} chars", title_length),
            url: url.clone(),
            suggestion: "Titles should be 50-60 characters for optimal display".to_string(),
        });
    } else if title_length > 60 {
        issues.push(SeoIssue {
            severity: Severity::Warning,
            category: "Title".to_string(),
            message: format!("Title too long: {} chars", title_length),
            url: url.clone(),
            suggestion: "Titles over 60 chars may be truncated in search results".to_string(),
        });
    }
    
    // Description analysis
    let description = post.summary.clone().unwrap_or_default();
    let description_length = description.chars().count();
    if description_length == 0 {
        issues.push(SeoIssue {
            severity: Severity::Error,
            category: "Meta Description".to_string(),
            message: "Missing meta description".to_string(),
            url: url.clone(),
            suggestion: "Add a summary field in frontmatter (150-160 chars recommended)".to_string(),
        });
    } else if description_length < 120 {
        issues.push(SeoIssue {
            severity: Severity::Warning,
            category: "Meta Description".to_string(),
            message: format!("Description too short: {} chars", description_length),
            url: url.clone(),
            suggestion: "Meta descriptions should be 150-160 characters".to_string(),
        });
    } else if description_length > 160 {
        issues.push(SeoIssue {
            severity: Severity::Info,
            category: "Meta Description".to_string(),
            message: format!("Description might be truncated: {} chars", description_length),
            url: url.clone(),
            suggestion: "Keep under 160 characters for full display".to_string(),
        });
    }
    
    // Cover image check
    if post.cover.is_none() {
        issues.push(SeoIssue {
            severity: Severity::Warning,
            category: "Open Graph".to_string(),
            message: "Missing cover image".to_string(),
            url: url.clone(),
            suggestion: "Add a cover image for better social sharing".to_string(),
        });
    }
    
    // Content analysis
    let word_count = post.content.split_whitespace().count();
    if word_count < 300 {
        issues.push(SeoIssue {
            severity: Severity::Warning,
            category: "Content".to_string(),
            message: format!("Content too short: {} words", word_count),
            url: url.clone(),
            suggestion: "Posts with 300+ words typically rank better".to_string(),
        });
    }
    
    // Count images and check for alt text patterns
    let image_count = post.content.matches("![").count();
    let images_without_alt = post.content.matches("![]").count();
    if images_without_alt > 0 {
        issues.push(SeoIssue {
            severity: Severity::Error,
            category: "Accessibility".to_string(),
            message: format!("{} images missing alt text", images_without_alt),
            url: url.clone(),
            suggestion: "Add descriptive alt text to all images".to_string(),
        });
    }
    
    // Calculate score
    let base_score = 100u32;
    let error_penalty = issues.iter().filter(|i| i.severity == Severity::Error).count() as u32 * 15;
    let warning_penalty = issues.iter().filter(|i| i.severity == Severity::Warning).count() as u32 * 5;
    let score = base_score.saturating_sub(error_penalty).saturating_sub(warning_penalty);
    
    PageAnalysis {
        url,
        title: post.title.clone(),
        title_length,
        description_length,
        has_h1: true, // Assuming markdown renders H1
        image_count,
        images_without_alt,
        word_count,
        issues,
        score,
    }
}

/// Analyze a page for SEO issues
pub fn analyze_page(page: &Page, _config: &SiteConfig) -> PageAnalysis {
    let mut issues = Vec::new();
    let url = format!("/{}/", page.slug);
    
    // Title analysis
    let title_length = page.title.chars().count();
    if title_length < 30 {
        issues.push(SeoIssue {
            severity: Severity::Warning,
            category: "Title".to_string(),
            message: format!("Title too short: {} chars", title_length),
            url: url.clone(),
            suggestion: "Titles should be 50-60 characters".to_string(),
        });
    }
    
    // Description analysis
    let description = page.description.clone().unwrap_or_default();
    let description_length = description.chars().count();
    if description_length == 0 {
        issues.push(SeoIssue {
            severity: Severity::Warning,
            category: "Meta Description".to_string(),
            message: "Missing page description".to_string(),
            url: url.clone(),
            suggestion: "Add description field in page.json".to_string(),
        });
    }
    
    let score = 100u32.saturating_sub(issues.len() as u32 * 10);
    
    PageAnalysis {
        url,
        title: page.title.clone(),
        title_length,
        description_length,
        has_h1: true,
        image_count: 0,
        images_without_alt: 0,
        word_count: 0,
        issues,
        score,
    }
}

/// Generate full SEO report
pub fn generate_report(
    posts: &[Post],
    pages: &[Page],
    config: &SiteConfig,
) -> SeoReport {
    // Use current time string directly
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let generated_at = format!("Timestamp: {}", now);
    
    let post_analyses: Vec<PageAnalysis> = posts.iter()
        .map(|p| analyze_post(p, config))
        .collect();
    
    let page_analyses: Vec<PageAnalysis> = pages.iter()
        .map(|p| analyze_page(p, config))
        .collect();
    
    let all_issues: Vec<&SeoIssue> = post_analyses.iter()
        .flat_map(|a| &a.issues)
        .chain(page_analyses.iter().flat_map(|a| &a.issues))
        .collect();
    
    let total_score: u32 = post_analyses.iter().map(|a| a.score).sum::<u32>()
        + page_analyses.iter().map(|a| a.score).sum::<u32>();
    let total_items = post_analyses.len() + page_analyses.len();
    let average_score = if total_items > 0 { total_score / total_items as u32 } else { 0 };
    
    SeoReport {
        generated_at,
        site_url: config.site.base_url.clone(),
        total_pages: page_analyses.len(),
        total_posts: post_analyses.len(),
        total_issues: all_issues.len(),
        errors: all_issues.iter().filter(|i| i.severity == Severity::Error).count(),
        warnings: all_issues.iter().filter(|i| i.severity == Severity::Warning).count(),
        average_score,
        pages: page_analyses,
        posts: post_analyses,
    }
}

/// Write SEO report to directory
pub fn write_report(report: &SeoReport, output_dir: &str) -> Result<()> {
    let output_path = Path::new(output_dir);
    fs::create_dir_all(output_path)?;
    
    // Write JSON report
    let json_path = output_path.join("report.json");
    let json = serde_json::to_string_pretty(report)?;
    fs::write(&json_path, json)?;
    info!("Generated SEO report: {}", json_path.display());
    
    // Write HTML dashboard
    let html_path = output_path.join("index.html");
    let html = generate_dashboard_html(report);
    fs::write(&html_path, html)?;
    info!("Generated SEO dashboard: {}", html_path.display());
    
    Ok(())
}

/// Generate HTML dashboard
fn generate_dashboard_html(report: &SeoReport) -> String {
    let errors_class = if report.errors > 0 { "error" } else { "success" };
    let warnings_class = if report.warnings > 0 { "warning" } else { "success" };
    let score_class = if report.average_score >= 80 { "success" } else if report.average_score >= 60 { "warning" } else { "error" };
    
    let post_rows: String = report.posts.iter().map(|p| {
        let score_class = if p.score >= 80 { "success" } else if p.score >= 60 { "warning" } else { "error" };
        format!(r#"<tr>
            <td><a href="{}">{}</a></td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td class="{}">{}</td>
        </tr>"#, p.url, html_escape(&p.title), p.title_length, p.description_length, p.issues.len(), score_class, p.score)
    }).collect();
    
    let page_rows: String = report.pages.iter().map(|p| {
        let score_class = if p.score >= 80 { "success" } else if p.score >= 60 { "warning" } else { "error" };
        format!(r#"<tr>
            <td><a href="{}">{}</a></td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td class="{}">{}</td>
        </tr>"#, p.url, html_escape(&p.title), p.title_length, p.description_length, p.issues.len(), score_class, p.score)
    }).collect();
    
    let all_issues: Vec<&SeoIssue> = report.posts.iter()
        .flat_map(|a| &a.issues)
        .chain(report.pages.iter().flat_map(|a| &a.issues))
        .collect();
    
    let issue_rows: String = all_issues.iter().map(|i| {
        let severity_class = match i.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        };
        format!(r#"<tr class="{}">
            <td>{:?}</td>
            <td>{}</td>
            <td>{}</td>
            <td><code>{}</code></td>
            <td>{}</td>
        </tr>"#, severity_class, i.severity, html_escape(&i.category), html_escape(&i.message), i.url, html_escape(&i.suggestion))
    }).collect();
    
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SEO Report - {}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #f5f5f5; color: #333; }}
        .container {{ max-width: 1200px; margin: 0 auto; padding: 20px; }}
        h1 {{ margin-bottom: 10px; }}
        .subtitle {{ color: #666; margin-bottom: 30px; }}
        .cards {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin-bottom: 30px; }}
        .card {{ background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); text-align: center; }}
        .card-value {{ font-size: 2.5rem; font-weight: bold; }}
        .card-label {{ color: #666; margin-top: 5px; }}
        .success {{ color: #22c55e; }}
        .warning {{ color: #f59e0b; }}
        .error {{ color: #ef4444; }}
        .info {{ color: #3b82f6; }}
        table {{ width: 100%; background: white; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1); margin-bottom: 30px; }}
        th, td {{ padding: 12px 16px; text-align: left; border-bottom: 1px solid #eee; }}
        th {{ background: #f8f9fa; font-weight: 600; }}
        tr:hover {{ background: #f8f9fa; }}
        tr.error {{ background: #fef2f2; }}
        tr.warning {{ background: #fffbeb; }}
        tr.info {{ background: #eff6ff; }}
        a {{ color: #3b82f6; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
        code {{ background: #f1f5f9; padding: 2px 6px; border-radius: 4px; font-size: 0.9em; }}
        h2 {{ margin: 30px 0 15px; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>📊 SEO Report</h1>
        <p class="subtitle">Generated: {} | Site: {}</p>
        
        <div class="cards">
            <div class="card">
                <div class="card-value {}">{}</div>
                <div class="card-label">Average Score</div>
            </div>
            <div class="card">
                <div class="card-value">{}</div>
                <div class="card-label">Posts Analyzed</div>
            </div>
            <div class="card">
                <div class="card-value">{}</div>
                <div class="card-label">Pages Analyzed</div>
            </div>
            <div class="card">
                <div class="card-value {}">{}</div>
                <div class="card-label">Errors</div>
            </div>
            <div class="card">
                <div class="card-value {}">{}</div>
                <div class="card-label">Warnings</div>
            </div>
        </div>
        
        <h2>📝 Posts</h2>
        <table>
            <thead>
                <tr>
                    <th>Title</th>
                    <th>Title Length</th>
                    <th>Description Length</th>
                    <th>Issues</th>
                    <th>Score</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
        
        <h2>📄 Pages</h2>
        <table>
            <thead>
                <tr>
                    <th>Title</th>
                    <th>Title Length</th>
                    <th>Description Length</th>
                    <th>Issues</th>
                    <th>Score</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
        
        <h2>⚠️ All Issues ({})</h2>
        <table>
            <thead>
                <tr>
                    <th>Severity</th>
                    <th>Category</th>
                    <th>Message</th>
                    <th>URL</th>
                    <th>Suggestion</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>
</body>
</html>"#,
        html_escape(&report.site_url),
        report.generated_at,
        html_escape(&report.site_url),
        score_class,
        report.average_score,
        report.total_posts,
        report.total_pages,
        errors_class,
        report.errors,
        warnings_class,
        report.warnings,
        post_rows,
        page_rows,
        report.total_issues,
        issue_rows
    )
}

/// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
