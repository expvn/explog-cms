# Changelog

All notable changes to Explog CMS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.8] - 2026-01-21

### Added
- **User Guide Documentation** (`docs/user_guide.md`): Comprehensive getting started guide
- **Security Guide** (`docs/security.md`): Cargo audit instructions and best practices
- **Cache Module Tests**: Unit tests for hash functions, BuildCache, and ChangeSet
- **Stages Module Documentation**: Detailed docs explaining pipeline architecture and future plans

### Changed
- **Safer Error Handling**: Replaced `.unwrap()` with `.unwrap_or()` in 6 locations across builder.rs, template.rs, asset_processor.rs, asset_validator.rs
- **API Module Documentation**: Added `#![allow(dead_code)]` with doc comments to image_cdn.rs and seo.rs explaining they are public APIs for plugins
- **Removed Unused Import**: Removed `std::collections::HashMap` from image_cdn.rs

### Fixed
- **Compiler Warnings**: Reduced warnings from 21 to minimal (only intentional dead_code for API modules)

## [0.3.7] - 2026-01-20

### Fixed
- **Selective rebuild for Gallery pages**: `explog build --page <slug>` now correctly renders Gallery pages with proper template and copies gallery assets
- **Incremental build detection**: Now detects file modifications by hashing source files, not just parsed content. Files edited after build will be correctly detected and rebuilt

### Changed
- **Gallery template renamed**: `customize/default.html` → `gallery.html` (moved out of customize/ folder)
- Template lookup maintains backward compatibility with old `customize/default.html` path

## [0.3.6] - 2026-01-18

### Added
- **Image CDN Module** (`src/core/image_cdn.rs`)
  - `CdnProvider` trait for extensible provider system
  - Cloudinary provider with fetch mode and transformations
  - Imgix provider with parameter-based transforms
  - Bunny CDN provider with pull zone support
  - Custom provider with pattern-based URL generation
  - Noop provider for disabled CDN
- **CDN Configuration** in `explog.toml`
  - `[cdn]` section with enabled/provider fields
  - `[cdn.cloudinary]` - cloud_name, transformations
  - `[cdn.imgix]` - domain, params
  - `[cdn.bunny]` - pull_zone
  - `[cdn.custom]` - base_url, pattern
- **Shared Assets Architecture** (`themes/shared/`)
  - `themes/shared/css/markdown-base.css` - Obsidian-like markdown styling
  - `themes/shared/css/prism-synthwave.css` - Syntax highlighting
  - `themes/shared/js/main.js` - Code block copy button
  - Modified `assets.rs` to copy shared first, then theme (theme overrides)
- **Enhanced Markdown Parsing** (`src/core/content.rs`)
  - Line breaks: SoftBreak → HardBreak (single newline = `<br>`)
  - Callouts: `> [!NOTE]`, `> [!WARNING]`, `> [!TIP]`, `> [!INFO]`
  - YouTube embeds: auto-convert to iframe
  - Centered images: standalone `<img>` in `<figure>`
  - Bold/italic emphasis with visual distinction

## [0.3.5] - 2026-01-18

### Added
- **SEO Optimizer CLI**: Analyze site for SEO issues with `explog seo` command
  - Generates HTML dashboard at `.seo-report/index.html`
  - Generates JSON data at `.seo-report/report.json`
  - Custom output: `explog seo --output ./custom-folder`
- **JSON-LD Structured Data** (`src/core/seo.rs`)
  - Article schema for blog posts
  - Website schema for homepage
  - BreadcrumbList schema support
- **SEO Analysis Engine** (`src/core/seo_report.rs`)
  - Title length optimization (50-60 chars)
  - Meta description analysis (150-160 chars)
  - Cover image check for Open Graph
  - Content length analysis (300+ words)
  - Image alt text detection
  - Score calculation per page/post

## [0.3.4] - 2026-01-18

### Added
- **Selective Rebuild CLI Flags**: Rebuild specific content without full clean build
  - `--page <slug>`: Rebuild specific page
  - `--post <slug>`: Rebuild specific post
  - `--category <name>`: Rebuild specific category
  - `--tag <name>`: Rebuild specific tag
- **RebuildOptions struct**: New options model in cli/build.rs
- **build_site_selective()**: Wrapper function for selective rebuilds
- **filetime crate**: For modifying file timestamps to trigger cache invalidation

### Changed
- Updated `cli/build.rs` run() signature to accept optional rebuild filters
- Updated `cli/dev.rs` to use new 5-argument run() signature

## [0.3.3] - 2026-01-17

### Added
- **Page Schema v2**: Separated `type` and `mode` fields
  - `type`: Content type (`markdown`, `html`, `gallery`)
  - `mode`: Display mode (`embedded`, `standalone`)
- **ContentType enum**: Markdown, Html, Gallery
- **PageMode enum**: Embedded, Standalone

### Changed
- `gallery_items` → `items` in Page struct
- `path` → `url` in GalleryItem struct
- Removed old `PageType` enum (replaced by `ContentType` + `PageMode`)

### Removed
- `background` field from page config
- `page_type` field (superseded by `content_type` + `mode`)

## [0.3.2] - 2026-01-16

### Added
- **Flat page.json System**: Unified config-driven pages
  - Every page requires `page.json` with `type` field
  - Types: `markdown`, `webgl`, `standalone`, `gallery`
  - Flat URLs: `/{slug}/` (no prefixes)
- **EmbedConfig struct**: for WebGL/iframe pages with src, width, height

### Changed
- Simplified folder structure: `content/pages/{name}/`
- Removed subdirectories: `embedded/`, `standalone/`, `customize/`
- Removed `[pages]` config section from explog.toml

### Removed
- URL prefixes (`/e/`, `/s/`, `/c/`)

## [0.3.1] - 2026-01-16

### Added
- Pages URL prefix system (superseded by 0.3.2)

## [0.3.0] - 2026-01-15

### Added
- **Auto-linking for bare URLs**: URLs in Markdown content are automatically converted to clickable links during build
- **Configurable posts_per_page**: Now reads from `theme.toml` `[settings]` section
- **Configurable related_posts_count**: Now reads from `theme.toml` `[settings]` section

### Fixed
- `posts_per_page` setting now properly read from theme config (was hardcoded to 10)
- `related_posts_count` setting now properly read from theme config (was hardcoded to 4)

### Removed
- Legacy `themes/default/layout/` directory (use `layouts/` instead)

## [0.2.0] - 2026-01-15

### Added

- **Pipeline Architecture**: New extensible stage-based build pipeline (`build_stage.rs`, `pipeline_builder.rs`)
  - `ContentLoaderStage`: Parallel content loading
  - `TaxonomyBuilderStage`: Category/tag building
  - `RelatedPostsStage`: Related posts calculation
  - `NavigationStage`: Prev/next navigation
  - `TemplateRendererStage`: Full + incremental rendering
  - `AssetProcessorStage`: Asset copying, minification, image optimization
  - `SeoGeneratorStage`: Sitemap, feeds, search index

- **Strict Asset Mode**: Build fails if referenced assets are missing (`build.strict_assets = true`)

- **Hot Reload Improvements**: Dev server now watches `explog.toml`, has 300ms debouncing, and logs which file triggered rebuild

- **Theme System Architecture**: Complete restructure of theme system
  - Core/Layout separation: `core/` for CSS styling, `layout/` for HTML templates
  - Component system: Reusable partials (header, footer, sidebar, pagination)
  - Widget system: Configurable sidebar widgets (categories, tags, recent_posts, search)
  - Config-driven layouts: Define page sections in `theme.toml`

- **Page Types**: Three types of pages with CLI scaffolding
  - Customize: JSON-driven grid layouts (`explog new customize portfolio`)
  - Embedded: Markdown pages using theme (`explog new embedded about`)
  - Standalone: Independent landing pages (`explog new standalone landing`)

- **Plugin System**: Extensible plugin architecture with hooks
  - Plugin manifest format (`plugin.toml`)
  - Hook system: `after_content_load`, `before_render`, `after_build`, `on_dev_start`
  - CLI commands: `explog plugin list/show/new/remove`
  - Plugin documentation: `docs/plugins.md`

- **Scheduled Publishing**: Posts can have `publish_date` in frontmatter
  - Posts won't appear until publish_date is reached
  - `scheduling.rs` module handles date filtering

- **Draft Preview**: Draft posts can have `preview_token` for preview access
  - Access drafts via token URL
  - Generate tokens with `generate_preview_token()`

- **CSS/JS Minification**: New `minifier.rs` module for minifying CSS and JavaScript assets during build. Configurable via `build.minify` in `explog.toml`.

- **Theme Inheritance**: Support for child themes that can extend parent themes via `theme.toml` with `extends` field.

- **Related Posts**: Automatic related posts detection based on shared categories and tags. Posts can also explicitly define related posts in frontmatter.

- **Previous/Next Navigation**: Post pages now include navigation links to the previous and next posts in chronological order.

- **Dynamic Navigation Menu**: Navigation menu is now configurable via `explog.toml` with support for nested menu items.

- **RSS/Atom Feeds**: New `feed.rs` module generating both RSS 2.0 and Atom 1.0 feeds. Automatically linked in HTML head.

- **Authors System**: Multiple authors support with `[authors.id]` config. Each author has name, email, bio, avatar, url.

- **Social Links**: Configurable social media links in `[[site.socials]]`.

- **Sitemap Chunking**: Automatic sitemap splitting for large sites (5,000 URLs per file with sitemap-index.xml).

- **Strict Asset Mode**: Option `build.strict_assets` to fail build on missing assets.

- **Sharded Search**: Static search index with alphabet, category, and year shards for scalable search.

- **Image Optimization**: Automatic WebP conversion with parallel processing.

### Changed

- Updated `base.html` template with RSS/Atom feed links and dynamic navigation menu
- Updated `post.html` template with related posts section and prev/next navigation
- Integrated sitemap, feeds, and asset minification into the main build pipeline
- Incremental builds now properly include related posts and navigation

## [0.1.0] - 2024-01-01

### Added

- Initial release of Explog CMS
- Markdown content parsing with YAML frontmatter
- Tera template engine integration
- Incremental builds with content hashing
- Parallel processing with Rayon
- Sitemap generation
- Category and tag support
- Pagination for home and listing pages
