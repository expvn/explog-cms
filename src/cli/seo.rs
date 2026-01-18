use anyhow::Result;
use tracing::info;

use crate::core::{config, content, seo_report};

/// Run the SEO analysis command
pub fn run(output_dir: Option<String>) -> Result<()> {
    info!("Running SEO analysis...");
    
    // Load configuration
    let config = config::load_config("explog.toml")?;
    info!("Loaded configuration for: {}", config.site.title);
    
    // Load content
    info!("Loading posts from {}...", config.content.posts_dir);
    let posts = content::load_posts(&config.content.posts_dir)?;
    info!("Loaded {} posts", posts.len());
    
    info!("Loading pages from {}...", config.content.pages_dir);
    let pages = content::load_pages(&config.content.pages_dir)?;
    info!("Loaded {} pages", pages.len());
    
    // Generate report
    let report = seo_report::generate_report(&posts, &pages, &config);
    
    // Determine output directory
    let output = output_dir.unwrap_or_else(|| ".seo-report".to_string());
    
    // Write report
    seo_report::write_report(&report, &output)?;
    
    // Print summary
    println!();
    println!("📊 SEO Report Summary");
    println!("═══════════════════════════════════════");
    println!("  Posts analyzed:    {}", report.total_posts);
    println!("  Pages analyzed:    {}", report.total_pages);
    println!("  Total issues:      {}", report.total_issues);
    println!("  Errors:            {}", report.errors);
    println!("  Warnings:          {}", report.warnings);
    println!("  Average score:     {}/100", report.average_score);
    println!();
    println!("  📁 Report output:  {}/index.html", output);
    println!("  📋 JSON data:      {}/report.json", output);
    println!();
    
    if report.errors > 0 {
        println!("⚠️  {} critical SEO issues found! Review the report for details.", report.errors);
    } else if report.warnings > 0 {
        println!("📝 {} warnings found. Consider addressing them for better SEO.", report.warnings);
    } else {
        println!("✅ Great job! No major SEO issues found.");
    }
    
    Ok(())
}
