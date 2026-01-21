# Explog CMS - Project Knowledge Base

> This file is intended for AI agents to quickly understand the project.

## Project Overview

**Name:** Explog CMS  
**Type:** Static Site Generator (SSG)  
**Language:** Rust  
**Version:** 0.3.8  
**Target:** Blogs, content-heavy websites  
**Philosophy:** "Pure Static" - Zero runtime, deterministic builds

---

## Architecture Summary

### Build Pipeline
```
ContentLoader → Transformation → Rendering → Asset Processing → Output
```

### Core Directories
| Directory | Purpose |
|-----------|---------|
| `src/cli/` | CLI commands (build, dev, new, clean, plugin) |
| `src/core/` | Build pipeline, templates, content loading |
| `src/models/` | Data structures (Post, Page, Site, etc.) |
| `themes/` | Theme layouts and assets (Tera templates) |
| `content/posts/` | Blog posts (Markdown + YAML frontmatter) |
| `content/pages/` | Static pages (page.json config) |
| `plugins/` | Hook-based plugins |
| `public/` | Build output (auto-generated) |
| `.cache/` | Build cache (auto-generated) |

---

## Key Files

### Configuration
- `explog.toml` - Site configuration (title, base_url, navigation, authors)
- `themes/{theme}/theme.toml` - Theme settings

### Core Rust Files
| File | Purpose |
|------|---------|
| `src/main.rs` | CLI entry point (clap) |
| `src/core/builder.rs` | Main build pipeline, page/post rendering |
| `src/core/content.rs` | Load posts/pages, enhanced markdown parsing |
| `src/core/template.rs` | Tera template engine wrapper |
| `src/core/image_cdn.rs` | CDN provider trait + implementations |
| `src/core/cache.rs` | Incremental build caching |
| `src/models/page.rs` | Page struct, ContentType, PageMode enums |
| `src/models/post.rs` | Post struct and related types |

### Markdown Features (Obsidian-like)
- **Line breaks**: Single newline = `<br>` (SoftBreak → HardBreak)
- Callouts: `> [!NOTE]`, `> [!WARNING]`, `> [!TIP]`, `> [!INFO]`
- YouTube embeds: `![](https://youtube.com/watch?v=ID)`
- Centered images: standalone images wrapped in `<figure>`
- Task lists: `- [ ]`, `- [x]`
- Tables, footnotes, strikethrough, smart punctuation

### Shared Assets Architecture
```
themes/shared/          → Copied first to public/assets/
themes/{theme}/assets/  → Copied second (can override shared)
```
| File | Purpose |
|------|---------|
| `themes/shared/css/markdown-base.css` | Markdown styling (callouts, images, code) |
| `themes/shared/css/prism-synthwave.css` | Code block syntax highlighting |
| `themes/shared/js/main.js` | Code block copy button |

### Template Files
| Template | Purpose |
|----------|---------|
| `base.html` | Base layout (header, footer) |
| `home.html` | Homepage with pagination |
| `post.html` | Single post page |
| `page.html` | Standard page (markdown/html fragments) |
| `page-embed.html` | Embedded pages with iframe (WebGL, etc.) |
| `category.html` | Category listing |
| `tag.html` | Tag listing |
| `gallery.html` | Gallery pages (items grid) |

---

## Page System (Schema v2)

### page.json Configuration
```json
{
    "title": "Page Title",
    "type": "markdown|html|gallery",
    "mode": "embedded|standalone",
    "description": "Optional description",
    "embed": { "src": "index.html", "width": "960", "height": "600" }
}
```

### ContentType + PageMode Matrix
| ContentType | PageMode | Result |
|-------------|----------|--------|
| markdown | embedded | Render MD → page.html with theme |
| html | embedded | If embed config → page-embed.html (iframe) |
| html | embedded | No embed config → inject content into page.html |
| html | standalone | Copy entire folder as-is |
| gallery | embedded | customize/default.html template |

---

## CLI Commands

```bash
explog build                    # Full build
explog build --clean            # Force rebuild
explog build --page <slug>      # Rebuild specific page
explog build --post <slug>      # Rebuild specific post
explog build --category <name>  # Rebuild specific category
explog build --tag <name>       # Rebuild specific tag
explog dev --port 3000          # Dev server with hot-reload
explog new post <slug>          # Create new post
explog new page <slug>          # Create new page
explog clean                    # Clear build artifacts
explog plugin list/show/new/remove  # Plugin management
explog seo                      # Generate SEO report (.seo-report/)
explog seo --output ./custom    # Custom output folder
```

---

## Development Rules (from .agent/rules/rules.md)

### Mandatory Versioning
- Use semantic versioning (0.x.y)
- On change: bump_version, update_changelog, update_readme

### Required Files on Change
- `Cargo.toml` - version field
- `CHANGELOG.md` - new version entry
- `README.md` - version badge
- `docs/explog_architecture.md` - version field

### Review Policy
- Require review before execution
- Block execution until approved

---

## Common Tasks

### Add New Feature
1. Implement in appropriate `src/` file
2. Update version in `Cargo.toml`
3. Add CHANGELOG entry
4. Update README if user-facing
5. Build and test

### Add New Page Type
1. Add ContentType variant in `src/models/page.rs`
2. Handle in `src/core/builder.rs` rendering loop
3. Create template if needed
4. Document in `docs/pages.md`

### Debug Build Issues
1. Check `cargo build --release 2>&1` for errors
2. Run `explog build --clean` to clear cache
3. Check `content/*/` for valid frontmatter/config

---

## Known Warnings (Intentional)

| Warning | Location | Reason |
|---------|----------|--------|
| `render_post` unused | template.rs | API method for plugins |
| `render_category_page` unused | template.rs | API method for plugins |
| `quality` field unused | images.rs | Reserved for future lossy WebP |

---

## Bug Fixes (v0.3.6)

| Issue | Solution |
|-------|----------|
| Post images not displaying | builder.rs now copies images/ folder from content/posts/{slug}/ to public/post/{slug}/ |
| URLs in code blocks auto-linked | linkify_urls now skips URLs inside backticks (code blocks and inline code) |
| Selective rebuild not working | invalidate_selective_cache now directly modifies cache JSON, removing entries to force rebuild |

---

## Bug Fixes (v0.3.7)

| Issue | Solution |
|-------|----------|
| Selective rebuild Gallery pages blank | Incremental rebuild now checks content_type and uses proper template (render_customize_page for Gallery) |
| customize/default.html messy structure | Moved to gallery.html directly in layouts/, with backward compatibility |
| Incremental build not detecting file changes | Cache now stores source_hash from original files, comparing both content and source file hashes |

---

## Audit Improvements (v0.3.8)

| Category | Improvement |
|----------|-------------|
| Error Handling | Replaced 6x `.unwrap()` with `.unwrap_or()` for safer fallbacks |
| Documentation | Added `docs/user_guide.md` and `docs/security.md` |
| Testing | Added 5 unit tests for cache module |
| Code Quality | Added `#![allow(dead_code)]` with docs for API modules |
| Stages Pipeline | Added comprehensive documentation for future extensibility |

---

## File Counts (as of v0.3.8)

- Source files: ~32 Rust files
- Themes: 8 (default, aurora, journal, magazina, minimal, vibe, zenith, shared)
- Localized READMEs: 18 languages
- Documentation: 7 files in docs/
- Unit Tests: cache, image_cdn, seo modules
