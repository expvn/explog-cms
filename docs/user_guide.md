# Explog CMS - User Guide

## Quick Start

### 1. Installation

```bash
git clone https://github.com/your-username/explog.git
cd explog
cargo build --release
```

### 2. Create Your First Site

```bash
# Create a new post
./target/release/explog new post hello-world

# Edit content/posts/hello-world/index.md with your content
```

### 3. Build and Preview

```bash
# Build the site
./target/release/explog build

# Start development server
./target/release/explog dev --port 3000
# Open http://localhost:3000
```

---

## Content Management

### Posts

Posts are stored in `content/posts/{slug}/index.md`:

```yaml
---
title: "My Post"
date: 2024-01-15
categories:
  - Technology
tags:
  - rust
---

Your markdown content here...
```

### Pages

Pages are stored in `content/pages/{slug}/`:

1. **Markdown pages**: `index.md` file
2. **Gallery pages**: `page.json` with items array
3. **Standalone pages**: Full HTML/WebGL projects

---

## Themes

### Switching Themes

Edit `explog.toml`:

```toml
[build]
theme = "vibe"  # aurora, default, journal, magazina, minimal, vibe, zenith
```

### Available Themes

| Theme | Description |
|-------|-------------|
| `default` | Clean, professional |
| `aurora` | Modern with gradients |
| `journal` | Magazine-style |
| `magazina` | News/magazine layout |
| `minimal` | Simple, fast |
| `vibe` | Young, dynamic |
| `zenith` | Premium, elegant |

---

## Advanced Features

### Scheduled Publishing

```yaml
---
title: "Future Post"
publish_date: "2024-01-20T10:00:00Z"
---
```

### Draft Preview

```yaml
---
title: "Draft Post"
draft: true
preview_token: "secret123"
---
```

Access via: `http://localhost:3000/post/slug?preview=secret123`

### SEO Analysis

```bash
./target/release/explog seo
# View report at .seo-report/index.html
```

---

## Troubleshooting

### Build Issues

```bash
# Clear cache and rebuild
./target/release/explog build --clean
```

### Content Not Showing

1. Check frontmatter syntax (must be valid YAML)
2. Ensure `draft: false` or not set
3. Check `publish_date` is not in the future

### Theme Issues

1. Verify theme exists in `themes/` directory
2. Check `theme.toml` is valid
3. Ensure `base.html` exists in `layouts/`

---

## Getting Help

- [Architecture Docs](./explog_architecture.md)
- [Theme Development](./themes.md)
- [Plugin Development](./plugins.md)
