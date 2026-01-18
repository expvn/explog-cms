# Explog CMS Theme Development Guide

## Overview

Explog CMS uses the **Tera** templating engine (similar to Jinja2/Django templates) for themes. This guide covers everything you need to create custom themes.

---

## Quick Start

### Step 1: Create Theme Directory

```
themes/my-theme/
├── theme.toml           # Required: Theme configuration
├── layouts/             # Required: Template files
│   ├── base.html
│   ├── home.html
│   ├── post.html
│   ├── category.html
│   ├── tag.html
│   └── page.html
├── assets/              # Optional: Theme-specific assets (override shared)
│   ├── css/
│   ├── js/
│   └── images/
└── core/                # Optional: Core CSS
```

### Step 2: Configure Theme (theme.toml)

```toml
[theme]
name = "my-theme"
version = "1.0.0"
description = "My custom theme"
author = "Your Name"

[settings]
posts_per_page = 12
related_posts_count = 4
```

### Step 3: Create Required Templates

Copy from an existing theme or create from scratch:

```bash
# Option A: Copy from default theme
cp -r themes/default/layouts themes/my-theme/layouts
```

### Step 4: Activate Theme

Edit `explog.toml`:
```toml
[build]
theme = "my-theme"
```

### Step 5: Build and Test

```bash
./target/release/explog build --clean
./target/release/explog dev --port 3000
```

---

## Shared Assets Architecture

Explog uses a **shared-first** asset copy pattern:

```
themes/shared/          → Copied first to public/assets/
themes/{theme}/assets/  → Copied second (can override shared)
```

### Shared Files (all themes inherit)

| File | Purpose |
|------|---------|
| `themes/shared/css/markdown-base.css` | Markdown styling (callouts, images, code, bold/italic) |
| `themes/shared/css/prism-synthwave.css` | Code block syntax highlighting |
| `themes/shared/js/main.js` | Code block copy button |

### Theme Override Example

If you want custom code highlighting:

```css
/* themes/my-theme/assets/css/prism-synthwave.css */
/* This will OVERRIDE the shared prism-synthwave.css */
pre code { background: #1a1a2e; }
```

---

## Theme Configuration (theme.toml)

```toml
[theme]
name = "my-theme"
version = "1.0.0"
description = "My custom theme"
author = "Your Name"
extends = ""              # Optional: parent theme to inherit from

# Layout configurations for different page types
[layout.home]
sections = ["hero", "featured_posts", "recent_posts"]
sidebar = true
widgets = ["search", "categories", "tags", "recent_posts"]

[layout.post]
sections = ["header", "content", "author", "related_posts", "navigation"]
sidebar = true
widgets = ["search", "categories", "tags"]

[layout.category]
sections = ["header", "post_list"]
sidebar = true
widgets = ["search", "categories", "tags"]

# Theme-specific settings
[settings]
posts_per_page = 12           # Posts per listing page
related_posts_count = 4       # Related posts to show
show_reading_time = true      # Display reading time
show_author = true            # Display author info
show_date = true              # Display publish date
show_categories = true        # Display categories
show_tags = true              # Display tags
```

---

## Template Files

### Required Templates

| Template | Purpose | Context Variables |
|----------|---------|-------------------|
| `base.html` | Base layout (header, footer, etc.) | `site`, `categories`, `tags` |
| `home.html` | Homepage | `posts`, `pagination`, `site` |
| `post.html` | Single post page | `post`, `related`, `navigation`, `site` |
| `category.html` | Category archive | `category`, `posts`, `site` |
| `tag.html` | Tag archive | `tag`, `posts`, `site` |
| `page.html` | Static page | `page`, `site` |

### base.html (Base Template)

```html
<!DOCTYPE html>
<html lang="{{ site.language | default(value='en') }}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% block title %}{{ site.title }}{% endblock %}</title>
    
    <!-- Base CSS -->
    <link rel="stylesheet" href="/assets/css/style.css">
    
    <!-- RSS/Atom feeds -->
    <link rel="alternate" type="application/rss+xml" href="/rss.xml">
    <link rel="alternate" type="application/atom+xml" href="/atom.xml">
    
    {% block head %}{% endblock %}
</head>
<body>
    <header>
        {% include "components/header.html" %}
    </header>
    
    <main>
        {% block content %}{% endblock %}
    </main>
    
    <footer>
        {% include "components/footer.html" %}
    </footer>
    
    {% block scripts %}{% endblock %}
</body>
</html>
```

### home.html (Homepage)

```html
{% extends "base.html" %}

{% block title %}{{ site.title }} - Home{% endblock %}

{% block content %}
<div class="container">
    <h1>Latest Posts</h1>
    
    <div class="posts-grid">
        {% for post in posts %}
        <article class="post-card">
            {% if post.cover %}
            <img src="{{ post.cover }}" alt="{{ post.title }}">
            {% endif %}
            
            <div class="post-card-body">
                {% if post.categories | length > 0 %}
                <a href="/category/{{ post.categories | first | slugify }}/" class="category">
                    {{ post.categories | first }}
                </a>
                {% endif %}
                
                <h2><a href="{{ post.url }}">{{ post.title }}</a></h2>
                <time>{{ post.date }}</time>
                
                {% if post.summary %}
                <p>{{ post.summary }}</p>
                {% endif %}
            </div>
        </article>
        {% endfor %}
    </div>
    
    <!-- Pagination -->
    {% if pagination %}
    <nav class="pagination">
        {% if pagination.has_prev %}
        <a href="{{ pagination.prev_url }}">← Previous</a>
        {% endif %}
        
        <span>Page {{ pagination.current_page }} of {{ pagination.total_pages }}</span>
        
        {% if pagination.has_next %}
        <a href="{{ pagination.next_url }}">Next →</a>
        {% endif %}
    </nav>
    {% endif %}
</div>
{% endblock %}
```

### post.html (Single Post)

```html
{% extends "base.html" %}

{% block title %}{{ post.title }} - {{ site.title }}{% endblock %}

{% block content %}
<article class="post">
    <header class="post-header">
        <h1>{{ post.title }}</h1>
        <div class="post-meta">
            <time>📅 {{ post.date }}</time>
            <span>👤 {{ post.author | default(value='Admin') }}</span>
        </div>
    </header>
    
    {% if post.cover %}
    <div class="post-cover">
        <img src="{{ post.cover }}" alt="{{ post.title }}">
    </div>
    {% endif %}
    
    <div class="post-content">
        {{ post.content | safe }}
    </div>
    
    <!-- Tags -->
    {% if post.tags | length > 0 %}
    <div class="post-tags">
        {% for tag in post.tags %}
        <a href="/tag/{{ tag | slugify }}/">{{ tag }}</a>
        {% endfor %}
    </div>
    {% endif %}
    
    <!-- Navigation -->
    {% if navigation %}
    <nav class="post-navigation">
        {% if navigation.prev %}
        <a href="{{ navigation.prev.url }}">← {{ navigation.prev.title }}</a>
        {% endif %}
        {% if navigation.next %}
        <a href="{{ navigation.next.url }}">{{ navigation.next.title }} →</a>
        {% endif %}
    </nav>
    {% endif %}
    
    <!-- Related Posts -->
    {% if related | length > 0 %}
    <section class="related-posts">
        <h3>Related Posts</h3>
        <div class="related-grid">
            {% for r in related %}
            <a href="{{ r.url }}">
                {% if r.cover %}<img src="{{ r.cover }}" alt="">{% endif %}
                <span>{{ r.title }}</span>
            </a>
            {% endfor %}
        </div>
    </section>
    {% endif %}
</article>
{% endblock %}
```

---

## Context Variables Reference

### Site Object (`site`)
```
site.title          - Site title
site.description    - Site description
site.base_url       - Base URL (https://...)
site.language       - Language code
site.navigation     - Array of nav items (see below)
site.socials        - Array of social links [{platform, url}]
```

#### Navigation Item Structure
```
item.label          - Display text
item.url            - Link URL
item.children       - Array of child items (for dropdowns)
```

**Example: Rendering Navigation with Dropdowns**
```html
{% for item in site.navigation %}
    {% if item.children | length > 0 %}
    <div class="dropdown">
        <a href="{{ item.url }}">{{ item.label }} ▾</a>
        <div class="dropdown-menu">
            {% for child in item.children %}
            <a href="{{ child.url }}">{{ child.label }}</a>
            {% endfor %}
        </div>
    </div>
    {% else %}
    <a href="{{ item.url }}">{{ item.label }}</a>
    {% endif %}
{% endfor %}
```

### Post Object (`post`)
```
post.title          - Post title
post.slug           - URL slug
post.url            - Full URL path
post.date           - Publish date
post.content        - HTML content (use | safe filter)
post.summary        - Post summary/excerpt
post.cover          - Cover image URL
post.categories     - Array of category names
post.tags           - Array of tag names
post.author         - Author ID
post.featured       - Boolean: is featured
```

### Pagination Object (`pagination`)
```
pagination.current_page   - Current page number
pagination.total_pages    - Total number of pages
pagination.total_items    - Total number of items
pagination.has_prev       - Boolean: has previous page
pagination.has_next       - Boolean: has next page
pagination.prev_url       - Previous page URL (null if none)
pagination.next_url       - Next page URL (null if none)
```

### Navigation Object (`navigation`)
```
navigation.prev     - Previous post {title, url} or null
navigation.next     - Next post {title, url} or null
```

### Related Posts Array (`related`)
```
related[].title     - Post title
related[].url       - Post URL
related[].cover     - Cover image URL
related[].date      - Post date
```

---

## Tera Template Syntax

### Variables
```html
{{ variable }}
{{ post.title }}
{{ site.navigation | length }}
```

### Filters
```html
{{ title | upper }}                    - Uppercase
{{ text | lower }}                     - Lowercase
{{ name | slugify }}                   - URL-safe slug
{{ content | safe }}                   - Render as raw HTML
{{ items | length }}                   - Count items
{{ value | default(value="fallback") }} - Default value
{{ date | date(format="%Y-%m-%d") }}   - Format date
```

### Conditionals
```html
{% if condition %}
    ...
{% elif other_condition %}
    ...
{% else %}
    ...
{% endif %}
```

### Loops
```html
{% for item in items %}
    {{ loop.index }}    - 1-indexed position
    {{ loop.index0 }}   - 0-indexed position
    {{ loop.first }}    - Boolean: first item
    {{ loop.last }}     - Boolean: last item
    {{ item }}
{% endfor %}
```

### Includes
```html
{% include "components/header.html" %}
```

### Template Inheritance
```html
{# In child template #}
{% extends "base.html" %}

{% block content %}
    Child content here
{% endblock %}
```

---

## Theme Inheritance

Themes can extend other themes using the `extends` field:

```toml
# themes/my-theme/theme.toml
[theme]
name = "my-theme"
extends = "default"    # Inherit from default theme
```

With inheritance:
1. Missing templates are loaded from parent theme
2. Assets from both themes are available
3. Settings can be overridden

---

## Best Practices

### 1. Use CSS Variables
```css
:root {
    --color-primary: #3b82f6;
    --color-text: #1f2937;
    --radius-md: 8px;
}
```

### 2. Mobile-First Design
```css
.container {
    padding: 16px;
}

@media (min-width: 768px) {
    .container {
        padding: 32px;
    }
}
```

### 3. Semantic HTML
- Use `<article>` for posts
- Use `<nav>` for navigation
- Use `<time>` for dates
- Use `<header>` and `<footer>`

### 4. Performance
- Minimize CSS/JS files
- Use `loading="lazy"` for images
- Inline critical CSS in `<head>`

### 5. Accessibility
- Add `alt` text to images
- Use proper heading hierarchy
- Ensure color contrast
