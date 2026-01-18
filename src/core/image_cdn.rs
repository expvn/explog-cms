use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Image transformation options
#[derive(Debug, Clone, Default)]
pub struct ImageTransformOpts {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub quality: Option<u8>,
    pub format: Option<String>,  // webp, avif, auto, jpg, png
}

impl ImageTransformOpts {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn width(mut self, w: u32) -> Self {
        self.width = Some(w);
        self
    }
    
    pub fn height(mut self, h: u32) -> Self {
        self.height = Some(h);
        self
    }
    
    pub fn quality(mut self, q: u8) -> Self {
        self.quality = Some(q);
        self
    }
    
    pub fn format(mut self, f: &str) -> Self {
        self.format = Some(f.to_string());
        self
    }
}

/// CDN provider trait - implement this for custom providers
pub trait CdnProvider: Send + Sync {
    /// Transform a local image path to CDN URL
    fn transform_url(&self, local_path: &str, base_url: &str, opts: &ImageTransformOpts) -> String;
    
    /// Provider name for logging
    fn name(&self) -> &'static str;
}

/// CDN configuration from explog.toml
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CdnConfig {
    #[serde(default)]
    pub enabled: bool,
    
    #[serde(default = "default_provider")]
    pub provider: String,
    
    #[serde(default)]
    pub cloudinary: Option<CloudinaryConfig>,
    
    #[serde(default)]
    pub imgix: Option<ImgixConfig>,
    
    #[serde(default)]
    pub bunny: Option<BunnyConfig>,
    
    #[serde(default)]
    pub custom: Option<CustomConfig>,
}

fn default_provider() -> String {
    "none".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudinaryConfig {
    pub cloud_name: String,
    #[serde(default = "default_cloudinary_transforms")]
    pub transformations: String,
}

fn default_cloudinary_transforms() -> String {
    "f_auto,q_auto".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImgixConfig {
    pub domain: String,
    #[serde(default = "default_imgix_params")]
    pub params: String,
}

fn default_imgix_params() -> String {
    "auto=format,compress".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BunnyConfig {
    pub pull_zone: String,
    #[serde(default)]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomConfig {
    pub base_url: String,
    /// Pattern with placeholders: {base}, {path}, {width}, {height}, {quality}, {format}
    #[serde(default = "default_custom_pattern")]
    pub pattern: String,
}

fn default_custom_pattern() -> String {
    "{base}/{path}".to_string()
}

// ============================================================================
// Provider Implementations
// ============================================================================

/// Cloudinary CDN Provider
pub struct CloudinaryProvider {
    cloud_name: String,
    transformations: String,
}

impl CloudinaryProvider {
    pub fn new(config: &CloudinaryConfig) -> Self {
        Self {
            cloud_name: config.cloud_name.clone(),
            transformations: config.transformations.clone(),
        }
    }
}

impl CdnProvider for CloudinaryProvider {
    fn transform_url(&self, local_path: &str, base_url: &str, opts: &ImageTransformOpts) -> String {
        let mut transforms = self.transformations.clone();
        
        // Add dimension transforms
        if let Some(w) = opts.width {
            transforms = format!("{},w_{}", transforms, w);
        }
        if let Some(h) = opts.height {
            transforms = format!("{},h_{}", transforms, h);
        }
        if let Some(q) = opts.quality {
            // Remove default q_auto if custom quality specified
            transforms = transforms.replace("q_auto", &format!("q_{}", q));
        }
        
        // Build full URL
        let source_url = format!("{}{}", base_url, local_path);
        format!(
            "https://res.cloudinary.com/{}/image/fetch/{}/{}",
            self.cloud_name, transforms, source_url
        )
    }
    
    fn name(&self) -> &'static str {
        "Cloudinary"
    }
}

/// Imgix CDN Provider
pub struct ImgixProvider {
    domain: String,
    params: String,
}

impl ImgixProvider {
    pub fn new(config: &ImgixConfig) -> Self {
        Self {
            domain: config.domain.clone(),
            params: config.params.clone(),
        }
    }
}

impl CdnProvider for ImgixProvider {
    fn transform_url(&self, local_path: &str, _base_url: &str, opts: &ImageTransformOpts) -> String {
        let mut params = self.params.clone();
        
        if let Some(w) = opts.width {
            params = format!("{}&w={}", params, w);
        }
        if let Some(h) = opts.height {
            params = format!("{}&h={}", params, h);
        }
        if let Some(q) = opts.quality {
            params = format!("{}&q={}", params, q);
        }
        if let Some(ref f) = opts.format {
            params = format!("{}&fm={}", params, f);
        }
        
        // Clean up path (remove leading slash)
        let path = local_path.trim_start_matches('/');
        
        format!("https://{}{}?{}", self.domain, path, params)
    }
    
    fn name(&self) -> &'static str {
        "Imgix"
    }
}

/// Bunny CDN Provider
pub struct BunnyProvider {
    pull_zone: String,
}

impl BunnyProvider {
    pub fn new(config: &BunnyConfig) -> Self {
        Self {
            pull_zone: config.pull_zone.clone(),
        }
    }
}

impl CdnProvider for BunnyProvider {
    fn transform_url(&self, local_path: &str, _base_url: &str, opts: &ImageTransformOpts) -> String {
        let path = local_path.trim_start_matches('/');
        
        let mut params = Vec::new();
        if let Some(w) = opts.width {
            params.push(format!("width={}", w));
        }
        if let Some(h) = opts.height {
            params.push(format!("height={}", h));
        }
        if let Some(q) = opts.quality {
            params.push(format!("quality={}", q));
        }
        
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        
        format!("https://{}/{}{}", self.pull_zone, path, query)
    }
    
    fn name(&self) -> &'static str {
        "Bunny CDN"
    }
}

/// Custom CDN Provider with pattern-based URL generation
pub struct CustomProvider {
    base_url: String,
    pattern: String,
}

impl CustomProvider {
    pub fn new(config: &CustomConfig) -> Self {
        Self {
            base_url: config.base_url.clone(),
            pattern: config.pattern.clone(),
        }
    }
}

impl CdnProvider for CustomProvider {
    fn transform_url(&self, local_path: &str, _base_url: &str, opts: &ImageTransformOpts) -> String {
        let path = local_path.trim_start_matches('/');
        
        let mut url = self.pattern.clone();
        url = url.replace("{base}", &self.base_url);
        url = url.replace("{path}", path);
        url = url.replace("{width}", &opts.width.map(|w| w.to_string()).unwrap_or_default());
        url = url.replace("{height}", &opts.height.map(|h| h.to_string()).unwrap_or_default());
        url = url.replace("{quality}", &opts.quality.map(|q| q.to_string()).unwrap_or("80".to_string()));
        url = url.replace("{format}", opts.format.as_deref().unwrap_or("auto"));
        
        url
    }
    
    fn name(&self) -> &'static str {
        "Custom CDN"
    }
}

/// No-op provider (CDN disabled)
pub struct NoopProvider;

impl CdnProvider for NoopProvider {
    fn transform_url(&self, local_path: &str, _base_url: &str, _opts: &ImageTransformOpts) -> String {
        local_path.to_string()
    }
    
    fn name(&self) -> &'static str {
        "None"
    }
}

// ============================================================================
// Factory Function
// ============================================================================

/// Get the appropriate CDN provider based on configuration
pub fn get_provider(config: &CdnConfig) -> Box<dyn CdnProvider> {
    if !config.enabled {
        return Box::new(NoopProvider);
    }
    
    match config.provider.to_lowercase().as_str() {
        "cloudinary" => {
            if let Some(ref cfg) = config.cloudinary {
                Box::new(CloudinaryProvider::new(cfg))
            } else {
                tracing::warn!("Cloudinary selected but not configured, falling back to noop");
                Box::new(NoopProvider)
            }
        }
        "imgix" => {
            if let Some(ref cfg) = config.imgix {
                Box::new(ImgixProvider::new(cfg))
            } else {
                tracing::warn!("Imgix selected but not configured, falling back to noop");
                Box::new(NoopProvider)
            }
        }
        "bunny" => {
            if let Some(ref cfg) = config.bunny {
                Box::new(BunnyProvider::new(cfg))
            } else {
                tracing::warn!("Bunny CDN selected but not configured, falling back to noop");
                Box::new(NoopProvider)
            }
        }
        "custom" => {
            if let Some(ref cfg) = config.custom {
                Box::new(CustomProvider::new(cfg))
            } else {
                tracing::warn!("Custom CDN selected but not configured, falling back to noop");
                Box::new(NoopProvider)
            }
        }
        _ => Box::new(NoopProvider),
    }
}

/// Transform image URL using the configured CDN provider
/// This is the main public API for the CDN module
pub fn transform_image_url(
    local_path: &str,
    base_url: &str,
    config: &CdnConfig,
    opts: &ImageTransformOpts,
) -> String {
    let provider = get_provider(config);
    provider.transform_url(local_path, base_url, opts)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cloudinary_transform() {
        let config = CloudinaryConfig {
            cloud_name: "test-cloud".to_string(),
            transformations: "f_auto,q_auto".to_string(),
        };
        let provider = CloudinaryProvider::new(&config);
        let opts = ImageTransformOpts::new().width(800);
        
        let url = provider.transform_url("/images/test.jpg", "https://example.com", &opts);
        assert!(url.contains("res.cloudinary.com"));
        assert!(url.contains("w_800"));
    }
    
    #[test]
    fn test_noop_provider() {
        let provider = NoopProvider;
        let opts = ImageTransformOpts::new().width(800);
        
        let url = provider.transform_url("/images/test.jpg", "https://example.com", &opts);
        assert_eq!(url, "/images/test.jpg");
    }
}
