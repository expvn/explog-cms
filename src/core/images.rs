use anyhow::{Context, Result};
use image::codecs::webp::WebPEncoder;
use image::{GenericImageView, ImageReader};
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use tracing::info;
use walkdir::WalkDir;

/// Image optimization settings
#[derive(Debug, Clone)]
pub struct ImageOptSettings {
    /// WebP quality (1-100) - reserved for future lossy encoding
    #[allow(dead_code)]
    pub quality: u8,
    /// Max width for resizing (0 = no resize)
    pub max_width: u32,
    /// Max height for resizing (0 = no resize)
    pub max_height: u32,
    /// Whether to keep original files
    pub keep_originals: bool,
}

impl Default for ImageOptSettings {
    fn default() -> Self {
        Self {
            quality: 80,
            max_width: 1920,
            max_height: 1080,
            keep_originals: true,
        }
    }
}

/// Optimize all images in the media directory
pub fn optimize_images(output_dir: &str, settings: &ImageOptSettings) -> Result<OptimizationStats> {
    let media_dir = Path::new(output_dir).join("media");
    
    if !media_dir.exists() {
        return Ok(OptimizationStats::default());
    }

    // Collect all image files
    let image_paths: Vec<_> = WalkDir::new(&media_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| is_optimizable_image(e.path()))
        .map(|e| e.path().to_path_buf())
        .collect();

    info!("Found {} images to optimize", image_paths.len());

    // Process images in parallel
    let results: Vec<_> = image_paths
        .par_iter()
        .map(|path| optimize_single_image(path, settings))
        .collect();

    // Calculate stats
    let mut stats = OptimizationStats::default();
    for result in results {
        match result {
            Ok((original_size, new_size)) => {
                stats.processed += 1;
                stats.original_bytes += original_size;
                stats.optimized_bytes += new_size;
            }
            Err(e) => {
                stats.failed += 1;
                tracing::warn!("Failed to optimize image: {}", e);
            }
        }
    }

    if stats.processed > 0 {
        let saved = stats.original_bytes.saturating_sub(stats.optimized_bytes);
        let percent = if stats.original_bytes > 0 {
            (saved as f64 / stats.original_bytes as f64 * 100.0) as u32
        } else {
            0
        };
        info!(
            "Optimized {} images, saved {}KB ({}%)",
            stats.processed,
            saved / 1024,
            percent
        );
    }

    Ok(stats)
}

/// Check if file is an optimizable image
fn is_optimizable_image(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff"
        ),
        None => false,
    }
}

/// Optimize a single image
fn optimize_single_image(path: &Path, settings: &ImageOptSettings) -> Result<(u64, u64)> {
    let original_size = fs::metadata(path)?.len();

    // Read image
    let img = ImageReader::open(path)
        .with_context(|| format!("Failed to open image: {}", path.display()))?
        .decode()
        .with_context(|| format!("Failed to decode image: {}", path.display()))?;

    // Resize if needed
    let img = if settings.max_width > 0 || settings.max_height > 0 {
        let (w, h) = img.dimensions();
        let max_w = if settings.max_width > 0 { settings.max_width } else { w };
        let max_h = if settings.max_height > 0 { settings.max_height } else { h };
        
        if w > max_w || h > max_h {
            img.resize(max_w, max_h, image::imageops::FilterType::Lanczos3)
        } else {
            img
        }
    } else {
        img
    };

    // Convert to WebP
    let webp_path = path.with_extension("webp");
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    
    let mut output = Vec::new();
    let encoder = WebPEncoder::new_lossless(&mut output);
    encoder.encode(&rgba, width, height, image::ExtendedColorType::Rgba8)
        .with_context(|| format!("Failed to encode WebP: {}", path.display()))?;

    // Write WebP file
    fs::write(&webp_path, &output)?;
    let new_size = output.len() as u64;

    // Remove original if configured
    if !settings.keep_originals && new_size < original_size {
        fs::remove_file(path)?;
    }

    Ok((original_size, new_size))
}

/// Statistics about image optimization
#[derive(Debug, Default)]
pub struct OptimizationStats {
    pub processed: usize,
    pub failed: usize,
    pub original_bytes: u64,
    pub optimized_bytes: u64,
}
