<p align="center">
  <img src="./themes/default/assets/favicon.ico" alt="Explog Logo" width="80" height="80">
</p>

<h1 align="center">Explog CMS</h1>

<p align="center">
  <a href="./Readme/README_en.md">English</a> |
  <a href="./Readme/README_vi.md">Tiếng Việt</a> |
  <a href="./Readme/README_es.md">Español</a> |
  <a href="./Readme/README_fr.md">Français</a> |
  <a href="./Readme/README_de.md">Deutsch</a> |
  <a href="./Readme/README_it.md">Italiano</a> |
  <a href="./Readme/README_pt.md">Português</a> |
  <a href="./Readme/README_ru.md">Русский</a> |
  <a href="./Readme/README_ja.md">日本語</a> |
  <a href="./Readme/README_ko.md">한국어</a> |
  <a href="./Readme/README_zh-CN.md">简体中文</a> |
  <a href="./Readme/README_zh-TW.md">繁體中文</a> |
  <a href="./Readme/README_ar.md">العربية</a> |
  <a href="./Readme/README_hi.md">हिन्दी</a> |
  <a href="./Readme/README_nl.md">Nederlands</a> |
  <a href="./Readme/README_pl.md">Polski</a> |
  <a href="./Readme/README_tr.md">Türkçe</a> |
  <a href="./Readme/README_th.md">ไทย</a>
</p>

---

![Version](https://img.shields.io/badge/version-0.3.8-blue)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-green)

A blazing-fast Static Site Generator written in Rust, optimized for blogs and content-heavy websites.

## ✨ Features

### Core
- 🚀 **Blazing Fast** - Parallel processing with Rayon
- 📝 **Markdown Support** - Full CommonMark with extensions
- 🔄 **Incremental Builds** - Only rebuild changed content
- 🎨 **Theme System** - Inheritable themes with Tera templates
- 🔌 **Plugin System** - Hook-based extensibility

### ContentSửa
- 📰 **Posts & Pages** - Blog posts and static pages
- 🏷️ **Categories & Tags** - Full taxonomy support
- 🔗 **Related Posts** - Auto-calculated by shared tags/categories
- ⬅️➡️ **Navigation** - Previous/next post links
- 📅 **Scheduled Publishing** - Publish posts at specific date/time
- 🔒 **Draft Preview** - Preview drafts via token

### SEO & Performance
- 🗺️ **Sitemap** - Auto-chunked for large sites (5000 URLs/file)
- 📡 **RSS/Atom Feeds** - Auto-generated feeds
- 🔍 **Search** - Static sharded search index  
- 🖼️ **Image Optimization** - Auto WebP conversion
- 📦 **Minification** - CSS/JS minification
- 🔗 **URL Auto-linking** - Bare URLs converted to clickable links

---

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/explog.git
cd explog

# Build release binary
cargo build --release

# The binary will be at: target/release/explog.exe (Windows) or target/release/explog (Unix)
```

### Create Your First Post

```bash
# Working directory: project root (where explog.toml is located)
./target/release/explog new post my-first-post
```

This creates:
```
content/posts/my-first-post/
├── index.md      # Your post content
└── images/       # Post-specific images
```

### Build & Preview

```bash
# Working directory: project root (where explog.toml is located)

# Build the site
./target/release/explog build

# Start dev server with hot-reload
./target/release/explog dev --port 3000
```

Open `http://localhost:3000` in your browser.

---

## 📁 Project Structure

```
explog/
├── content/
│   ├── posts/           # Blog posts (each in own folder)
│   └── pages/           # Static pages
├── themes/
│   └── default/         # Active theme
│       ├── theme.toml   # Theme configuration
│       ├── layouts/     # Tera templates
│       ├── assets/      # CSS/JS/Images
│       └── core/        # Core CSS styles
├── plugins/             # Plugins directory
├── public/              # Build output (auto-generated)
├── .cache/              # Build cache (auto-generated)
└── explog.toml          # Site configuration
```

---

## 💻 CLI Commands

> **Important:** All commands must be run from the project root directory (where `explog.toml` is located).

### Build Commands

```bash
# Full build
./target/release/explog build

# Force clean rebuild (ignore cache)
./target/release/explog build --clean

# Selective rebuild (rebuild specific content only)
./target/release/explog build --page page-slug
./target/release/explog build --post post-slug
./target/release/explog build --category category-name
./target/release/explog build --tag tag-name
```

### Development Server

```bash
# Start dev server (default port 8080)
./target/release/explog dev

# Custom port
./target/release/explog dev --port 3000
```

The dev server:
- Serves files from `public/` directory
- Watches for changes in `content/`, `themes/`, and `explog.toml`
- Auto-rebuilds on file changes

### Content Creation

```bash
# Create new post
./target/release/explog new post my-post-slug

# Create new page
./target/release/explog new page about
```

### Cache Management

```bash
# Clear build cache
./target/release/explog clean
```

### Plugin Management

```bash
# List installed plugins
./target/release/explog plugin list

# Show plugin details
./target/release/explog plugin show plugin-name

# Create new plugin
./target/release/explog plugin new my-plugin

# Remove plugin
./target/release/explog plugin remove plugin-name
```

### SEO Analyzer

```bash
# Generate SEO report (default: .seo-report/)
./target/release/explog seo

# Custom output directory
./target/release/explog seo --output ./custom-report
```

The SEO analyzer generates:
- `index.html` - Visual dashboard with scores and issues
- `report.json` - Machine-readable analysis data

---

## ⚙️ Configuration

### Site Configuration (`explog.toml`)

```toml
[site]
title = "My Blog"
description = "A personal blog"
base_url = "https://example.com"
language = "vi"

[[site.navigation]]
label = "Home"
url = "/"

[[site.navigation]]
label = "About"
url = "/about/"

[[site.socials]]
platform = "github"
url = "https://github.com/username"

[[site.socials]]
platform = "facebook"
url = "https://facebook.com/page"

[authors.admin]
name = "Admin"
email = "admin@example.com"
bio = "Blog author"

[build]
theme = "default"
output_dir = "public"
minify = true
strict_assets = false

[seo]
generate_sitemap = true
generate_rss = true

# CDN Configuration (optional - choose one provider)
[cdn]
enabled = true
provider = "cloudinary"  # cloudinary | imgix | bunny | custom

[cdn.cloudinary]
cloud_name = "your-cloud-name"
transformations = "f_auto,q_auto"

# [cdn.imgix]
# domain = "your-site.imgix.net"
# params = "auto=format,compress"

# [cdn.bunny]
# pull_zone = "your-zone.b-cdn.net"

# [cdn.custom]
# base_url = "https://your-cdn.com"
# pattern = "{base}/{path}?w={width}&q={quality}"
```

### Theme Configuration (`themes/default/theme.toml`)

```toml
[theme]
name = "default"
version = "1.0.0"
description = "Default theme for Explog CMS"
author = "Explog Team"
extends = ""  # Parent theme for inheritance

[layout.home]
sections = ["hero", "featured_posts", "recent_posts"]
sidebar = true
widgets = ["search", "categories", "tags", "recent_posts"]

[settings]
posts_per_page = 12          # Posts per page on listings
related_posts_count = 4      # Number of related posts to show
show_reading_time = true
show_author = true
show_date = true
show_categories = true
show_tags = true
```

---

## 📝 Post Frontmatter

```yaml
---
title: "My Post Title"
date: 2024-01-15
slug: "my-post-slug"           # Optional, defaults to folder name
categories:
  - "Technology"
  - "Web Development"
tags:
  - "rust"
  - "static-site"
summary: "Brief description"    # Optional, auto-generated if missing
cover: "images/cover.jpg"       # Relative to post folder
featured: true                  # Show in featured section
draft: false                    # Don't publish if true
author: "admin"                 # Must match [authors.id] in explog.toml
publish_date: "2024-01-20T10:00:00Z"  # Scheduled publishing
preview_token: "abc123"         # Access draft via token
---

Your markdown content here...
```

---

## 🎨 Theme Development

See [docs/themes.md](docs/themes.md) for complete theme development guide.

### Quick Overview

```
themes/my-theme/
├── theme.toml           # Theme configuration
├── layouts/
│   ├── base.html        # Base template (header, footer)
│   ├── home.html        # Homepage
│   ├── post.html        # Single post
│   ├── category.html    # Category archive
│   ├── tag.html         # Tag archive
│   ├── page.html        # Static page
│   └── components/      # Reusable components
├── assets/
│   ├── css/
│   └── js/
└── core/                # Core CSS styling
```

---

## 🔌 Plugin Development

See [docs/plugins.md](docs/plugins.md) for complete plugin development guide.

### Quick Overview

```bash
# Create a plugin
./target/release/explog plugin new my-plugin
```

This creates:
```
plugins/my-plugin/
├── plugin.toml          # Plugin manifest
├── scripts/             # Hook scripts
│   └── after_build.bat  # Runs after build completes
└── README.md
```

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [explog_architecture.md](explog_architecture.md) | Technical architecture |
| [docs/plugins.md](docs/plugins.md) | Plugin development guide |
| [docs/themes.md](docs/themes.md) | Theme development guide |
| [docs/FRONTMATTER_SCHEMA.md](docs/FRONTMATTER_SCHEMA.md) | Frontmatter reference |

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.
