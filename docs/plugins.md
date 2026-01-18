# Explog CMS Plugin Development Guide

## Overview

Explog CMS supports plugins that hook into various stages of the build process. Plugins are shell-script based and can execute custom logic at specific lifecycle events.

---

## Quick Start

### Step 1: Create Plugin Directory

```bash
# Automatic creation (recommended)
./target/release/explog plugin new my-plugin

# Or manual creation
mkdir -p plugins/my-plugin/scripts
```

This creates:
```
plugins/my-plugin/
├── plugin.toml          # Plugin manifest (required)
├── scripts/             # Hook scripts
│   └── after_build.bat  # Windows script
│   └── after_build.sh   # Unix script
└── README.md            # Documentation
```

### Step 2: Configure Plugin (plugin.toml)

```toml
[plugin]
name = "my-plugin"
version = "1.0.0"
description = "My awesome plugin"
author = "Your Name"
api_version = "0.3"

[hooks]
after_build = "scripts/after_build.sh"

[config]
custom_option = "value"
```

### Step 3: Create Hook Script

**Windows (scripts/after_build.bat):**
```batch
@echo off
echo Plugin executed!
echo Output: %EXPLOG_OUTPUT_DIR%
exit /b 0
```

**Unix (scripts/after_build.sh):**
```bash
#!/bin/bash
echo "Plugin executed!"
echo "Output: $EXPLOG_OUTPUT_DIR"
exit 0
```

### Step 4: Make Script Executable (Unix)

```bash
chmod +x plugins/my-plugin/scripts/after_build.sh
```

### Step 5: Test Plugin

```bash
# List plugins to verify
./target/release/explog plugin list

# Build to trigger hook
./target/release/explog build --clean
```

---

## Plugin Manifest (plugin.toml)

The manifest defines your plugin's metadata and hooks:

```toml
[plugin]
name = "my-plugin"
version = "1.0.0"
description = "My awesome plugin"
author = "Your Name"
api_version = "0.3"              # Minimum Explog version required

[hooks]
# Define which scripts run at each hook
after_content_load = "scripts/after_content.sh"
before_render = "scripts/before_render.sh"
after_build = "scripts/after_build.sh"
on_dev_start = "scripts/on_dev.sh"

[config]
# Custom configuration options (passed as env vars)
custom_option = "value"
another_option = "123"
```

---

## Available Hooks

| Hook | Timing | Use Cases |
|------|--------|-----------|
| `after_content_load` | After posts/pages are loaded | Content validation, transformation |
| `before_render` | Before templates render | Inject data, preprocess |
| `after_build` | After build completes | Deploy, optimize, notifications |
| `on_dev_start` | When dev server starts | Start companion services |

### Hook Execution Order

```
1. Content Loading
2. → after_content_load
3. Transformation (related posts, navigation)
4. → before_render
5. Template Rendering
6. Asset Processing
7. SEO Generation
8. → after_build
```

---

## Environment Variables

Hook scripts receive these environment variables:

| Variable | Description | Example |
|----------|-------------|---------|
| `EXPLOG_OUTPUT_DIR` | Build output directory | `C:\project\public` |
| `EXPLOG_CONTENT_DIR` | Content directory | `C:\project\content` |
| `EXPLOG_THEME_DIR` | Active theme directory | `C:\project\themes\default` |
| `EXPLOG_HOOK` | Current hook name | `after_build` |
| `EXPLOG_PLUGIN_DIR` | Plugin's own directory | `C:\project\plugins\my-plugin` |
| `EXPLOG_CONFIG_*` | Custom config values | `EXPLOG_CONFIG_custom_option=value` |

---

## Hook Script Examples

### Windows (`after_build.bat`)

```batch
@echo off
echo ========================================
echo Explog Post-Build Plugin
echo ========================================
echo Hook: %EXPLOG_HOOK%
echo Output: %EXPLOG_OUTPUT_DIR%
echo ========================================

REM Example: Copy sitemap to another location
copy "%EXPLOG_OUTPUT_DIR%\sitemap.xml" "D:\backup\sitemap.xml"

REM Example: Notify via webhook
curl -X POST https://webhook.site/xxx -d "Build complete"

REM Exit with 0 for success, non-zero for failure
exit /b 0
```

### Unix (`after_build.sh`)

```bash
#!/bin/bash
echo "========================================"
echo "Explog Post-Build Plugin"
echo "========================================"
echo "Hook: $EXPLOG_HOOK"
echo "Output: $EXPLOG_OUTPUT_DIR"
echo "========================================"

# Example: Compress output
cd "$EXPLOG_OUTPUT_DIR"
tar -czf ../site-backup.tar.gz .

# Example: Deploy to server
# rsync -avz . user@server:/var/www/html/

# Example: Invalidate CDN cache
# curl -X POST https://api.cloudflare.com/purge

# Exit with 0 for success, non-zero for failure
exit 0
```

---

## Plugin Examples

### 1. Image Compression Plugin

```toml
# plugins/image-compress/plugin.toml
[plugin]
name = "image-compress"
version = "1.0.0"
description = "Compress images after build"

[hooks]
after_build = "scripts/compress.sh"

[config]
quality = "85"
```

```bash
# plugins/image-compress/scripts/compress.sh
#!/bin/bash
find "$EXPLOG_OUTPUT_DIR" -name "*.jpg" -exec jpegoptim -m$EXPLOG_CONFIG_quality {} \;
find "$EXPLOG_OUTPUT_DIR" -name "*.png" -exec optipng {} \;
echo "Images compressed!"
```

### 2. Deploy to Vercel Plugin

```toml
# plugins/vercel-deploy/plugin.toml
[plugin]
name = "vercel-deploy"
version = "1.0.0"
description = "Deploy to Vercel after build"

[hooks]
after_build = "scripts/deploy.bat"

[config]
project_name = "my-blog"
```

```batch
@echo off
REM plugins/vercel-deploy/scripts/deploy.bat
cd %EXPLOG_OUTPUT_DIR%
vercel --prod --yes
echo Deployed to Vercel!
```

### 3. Content Validation Plugin

```toml
# plugins/content-validator/plugin.toml
[plugin]
name = "content-validator"
version = "1.0.0"
description = "Validate content structure"

[hooks]
after_content_load = "scripts/validate.sh"
```

```bash
# plugins/content-validator/scripts/validate.sh
#!/bin/bash
for dir in "$EXPLOG_CONTENT_DIR"/posts/*/; do
    if [ ! -f "$dir/index.md" ]; then
        echo "ERROR: Missing index.md in $dir"
        exit 1
    fi
done
echo "Content validation passed!"
```

### 4. Backup Plugin

```toml
# plugins/backup/plugin.toml
[plugin]
name = "backup"
version = "1.0.0"
description = "Create backup after each build"

[hooks]
after_build = "scripts/backup.sh"

[config]
backup_dir = "/backups"
keep_count = "5"
```

```bash
# plugins/backup/scripts/backup.sh
#!/bin/bash
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$EXPLOG_CONFIG_backup_dir/site_$TIMESTAMP.tar.gz"

cd "$EXPLOG_OUTPUT_DIR"
tar -czf "$BACKUP_FILE" .

# Keep only last N backups
cd "$EXPLOG_CONFIG_backup_dir"
ls -t site_*.tar.gz | tail -n +$((EXPLOG_CONFIG_keep_count + 1)) | xargs -r rm

echo "Backup created: $BACKUP_FILE"
```

---

## CLI Commands

```bash
# List all installed plugins
./target/release/explog plugin list

# Show plugin details
./target/release/explog plugin show my-plugin

# Create new plugin from template
./target/release/explog plugin new my-plugin

# Remove a plugin
./target/release/explog plugin remove my-plugin
```

---

## Best Practices

### 1. Version Your Plugins
Use semantic versioning (major.minor.patch):
```toml
version = "1.2.3"
```

### 2. Check API Version
Ensure compatibility with Explog version:
```toml
api_version = "0.3"
```

### 3. Handle Errors Gracefully
```bash
#!/bin/bash
if ! command -v some-tool &> /dev/null; then
    echo "ERROR: some-tool not found"
    exit 1
fi
```

### 4. Cross-Platform Support
Provide both `.bat` and `.sh` scripts:
```
scripts/
├── after_build.bat   # Windows
└── after_build.sh    # Unix/Mac
```

### 5. Document Your Plugin
Include a `README.md` with:
- Description
- Requirements
- Configuration options
- Examples

### 6. Use Config Variables
Instead of hardcoding, use `[config]` section:
```toml
[config]
api_key = "your-key"
```
Access in scripts as `$EXPLOG_CONFIG_api_key` or `%EXPLOG_CONFIG_api_key%`.

---

## Troubleshooting

### Script Not Executing
- Check file permissions: `chmod +x script.sh`
- Verify shebang line: `#!/bin/bash`
- Check path in plugin.toml

### Environment Variables Not Set
- Variables are only set during hook execution
- Use `echo` to debug values

### Build Fails After Plugin Added
- Check plugin script exit code (0 = success)
- Look at build output for error messages
- Try running script manually
