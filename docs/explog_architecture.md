# EXPLOG CMS: Technical Architecture Specification

## 1. Executive Summary

**Project Name:** Explog CMS  
**Type:** Static Site Generator (SSG)  
**Language:** Rust  
**Version:** 0.3.8  
**Target Scale:** 500,000+ posts  
**Core Philosophy:** "Pure Static" — Zero Runtime, Deterministic Builds, Plugin-Extensible.

---

## 2. Core Design Principles

### 2.1 The "Pure Static" Mandate
- **No Server-Side Runtime:** Output is static HTML/CSS/JS/Media files.
- **No Client-Side Routing:** Standard `<a>` navigation. JavaScript for enhancement only.
- **Physical URL Mapping:** `1 URL = 1 Physical File`
    - `/about/` → `/about/index.html`
    - `/post/hello-world/` → `/post/hello-world/index.html`

### 2.2 Performance Goals
| Metric | Target |
|--------|--------|
| TTFB | < 50ms (CDN) |
| FCP | < 800ms |
| Incremental Build | < 1 second |
| Memory | Handle 500k+ posts |

---

## 3. System Architecture

### 3.1 Build Pipeline (Stage-Based)

```
┌─────────────────────────────────────────────────────────────┐
│                    BUILD PIPELINE                           │
├─────────────────────────────────────────────────────────────┤
│  Stage 1: ContentLoader (Parallel - Rayon)                  │
│    ├── Parse Markdown + YAML Frontmatter                    │
│    ├── Auto-link bare URLs                                  │
│    └── Build Content Graph                                  │
├─────────────────────────────────────────────────────────────┤
│  Stage 2: Transformation                                    │
│    ├── Build Categories/Tags                                │
│    ├── Calculate RelatedPosts                               │
│    └── Calculate Prev/Next Navigation                       │
├─────────────────────────────────────────────────────────────┤
│  Stage 3: Generation                                        │
│    ├── Render Templates (Tera)                              │
│    ├── Copy/Minify Assets                                   │
│    └── Optimize Images (WebP - Parallel)                    │
├─────────────────────────────────────────────────────────────┤
│  Stage 4: SEO & Search                                      │
│    ├── Generate Sitemap (Chunked: 5000 URLs/file)           │
│    ├── Generate RSS/Atom Feeds                              │
│    └── Generate Sharded Search Index                        │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Parallel Processing (Rayon)

| Operation | Parallelization |
|-----------|-----------------|
| Content Loading | `par_iter()` on file paths |
| Image Optimization | `par_iter()` on image files |
| Template Rendering | Sequential (Tera not thread-safe) |

### 3.3 Incremental Build Strategy

```
┌──────────────────┐     ┌─────────────────┐
│   Source File    │────▶│   SHA-256 Hash  │
└──────────────────┘     └────────┬────────┘
                                  │
                    ┌─────────────▼─────────────┐
                    │     Cache Comparison      │
                    │ (.cache/content-hashes)   │
                    └─────────────┬─────────────┘
                                  │
        ┌─────────────────────────┼─────────────────────────┐
        │                         │                         │
        ▼                         ▼                         ▼
  ┌──────────┐           ┌─────────────┐           ┌─────────────┐
  │ Unchanged│           │   Changed   │           │    New      │
  │  Skip    │           │   Rebuild   │           │   Build     │
  └──────────┘           └─────────────┘           └─────────────┘
```

**Dependency Tracking:**
- Post changes → Rebuild post + affected categories/tags
- Theme changes → Full rebuild
- Config changes → Full rebuild

---

## 4. Directory Structure

```text
explog/
├── content/
│   ├── posts/                 # Blog posts
│   │   └── my-post/
│   │       ├── index.md       # Content file
│   │       └── images/        # Co-located assets
│   └── pages/                 # Static pages (require page.json)
│       ├── about/             # type: markdown
│       ├── test-webgl/        # type: webgl
│       └── expgames/          # type: gallery
├── themes/
│   ├── shared/                # Shared assets (copied first to public/)
│   │   ├── css/               # Shared CSS (markdown-base, prism)
│   │   └── js/                # Shared JS (code block copy)
│   └── [theme-name]/
│       ├── theme.toml         # Theme config
│       ├── layouts/           # Tera templates (.html)
│       │   ├── base.html
│       │   ├── home.html
│       │   ├── post.html
│       │   ├── category.html
│       │   ├── tag.html
│       │   └── components/
│       ├── assets/            # Theme-specific CSS/JS (overrides shared)
│       └── core/              # CSS styling
├── plugins/                   # Plugin directory
│   └── [plugin-name]/
│       ├── plugin.toml
│       └── scripts/
├── public/                    # Build output
├── .cache/                    # Incremental build cache
├── docs/                      # Documentation
└── explog.toml                # Site configuration
```

---

## 5. Configuration Schema

### 5.1 explog.toml (Site Configuration)
```toml
[site]
title = "My Blog"
base_url = "https://example.com"
language = "vi"

[[site.navigation]]
label = "Home"
url = "/"

[[site.socials]]
platform = "github"
url = "https://github.com/user"

[authors.admin]
name = "Admin"
email = "admin@example.com"

[build]
theme = "default"
output_dir = "public"
minify = true
strict_assets = false

[seo]
generate_sitemap = true
generate_rss = true
```

### 5.2 theme.toml (Theme Configuration)
```toml
[theme]
name = "default"
version = "1.0.0"
extends = ""  # Parent theme for inheritance

[layout.home]
sections = ["hero", "featured_posts", "recent_posts"]
sidebar = true
widgets = ["search", "categories", "tags"]

[settings]
posts_per_page = 12
related_posts_count = 4
show_reading_time = true
show_author = true
show_date = true
show_categories = true
show_tags = true
```

---

## 6. Core Modules

| Module | File | Description |
|--------|------|-------------|
| Content Loading | `content.rs` | Parallel Markdown parsing, URL auto-linking |
| Builder | `builder.rs` | Main build orchestration |
| Template Engine | `template.rs` | Tera + Theme inheritance |
| Theme Loader | `theme_loader.rs` | Parse theme.toml settings |
| Cache | `cache.rs` | Incremental build tracking |
| Minification | `minifier.rs` | CSS/JS minification |
| Image Processing | `images.rs` | WebP conversion (parallel) |
| Sitemap | `sitemap.rs` | Chunked sitemap generation |
| Feeds | `feed.rs` | RSS 2.0 + Atom 1.0 |
| Search | `search.rs` | Sharded static search index |
| Scheduling | `scheduling.rs` | Scheduled publishing, draft preview |
| Plugin System | `plugin_system.rs` | Hook-based plugins |

### 6.1 Stage Modules (`src/core/stages/`)

| Stage | Priority | Description |
|-------|----------|-------------|
| `content_loader.rs` | 10 | Load posts and pages |
| `taxonomy_builder.rs` | 20 | Build categories/tags |
| `related_posts.rs` | 30 | Calculate related posts |
| `navigation.rs` | 35 | Prev/next navigation |
| `template_renderer.rs` | 40 | Render all templates |
| `asset_processor.rs` | 50 | Copy and process assets |
| `seo_generator.rs` | 60 | Sitemap, feeds, search |

---

## 7. Data Contracts (Template API)

### 7.1 Global Context
```json
{
  "site": {
    "title": "Blog",
    "base_url": "https://...",
    "navigation": [...],
    "socials": [...]
  },
  "categories": [...],
  "tags": [...]
}
```

### 7.2 Post Context
```json
{
  "post": {
    "title": "...",
    "content": "<html>",
    "date": "2024-01-01",
    "categories": [...],
    "tags": [...],
    "cover": "/media/slug/cover.jpg"
  },
  "related": [...],
  "navigation": {
    "prev": {...},
    "next": {...}
  }
}
```

### 7.3 Pagination Context
```json
{
  "pagination": {
    "current_page": 1,
    "total_pages": 5,
    "total_items": 56,
    "has_prev": false,
    "has_next": true,
    "prev_url": null,
    "next_url": "/page/2/"
  }
}
```

---

## 8. CLI Commands

| Command | Description |
|---------|-------------|
| `explog build` | Production build |
| `explog build --clean` | Force full rebuild |
| `explog dev` | Dev server with hot-reload |
| `explog dev --port 3000` | Dev server on custom port |
| `explog clean` | Clear cache |
| `explog new post <slug>` | Create new post |
| `explog new page <slug>` | Create new page |
| `explog plugin list` | List installed plugins |
| `explog plugin new <name>` | Create new plugin |

---

## 9. Feature Summary

| Feature | Status |
|---------|--------|
| Core Build Pipeline | ✅ Complete |
| Incremental Builds | ✅ Complete |
| Parallel Processing | ✅ Complete |
| Theme Inheritance | ✅ Complete |
| Related Posts | ✅ Complete |
| Prev/Next Navigation | ✅ Complete |
| RSS/Atom Feeds | ✅ Complete |
| Sitemap Chunking | ✅ Complete |
| Sharded Search | ✅ Complete |
| Image Optimization | ✅ Complete |
| URL Auto-linking | ✅ Complete |
| Configurable Settings | ✅ Complete |
| Plugin System | ✅ Complete |
| Scheduled Publishing | ✅ Complete |
| Draft Preview | ✅ Complete |
