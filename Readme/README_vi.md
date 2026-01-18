<p align="center">
  <img src="../themes/default/assets/favicon.ico" alt="Explog Logo" width="80" height="80">
</p>

<h1 align="center">Explog CMS</h1>

<p align="center">
  <a href="./README_en.md">English</a> |
  <a href="./README_vi.md">Tiếng Việt</a> |
  <a href="./README_es.md">Español</a> |
  <a href="./README_fr.md">Français</a> |
  <a href="./README_de.md">Deutsch</a> |
  <a href="./README_it.md">Italiano</a> |
  <a href="./README_pt.md">Português</a> |
  <a href="./README_ru.md">Русский</a> |
  <a href="./README_ja.md">日本語</a> |
  <a href="./README_ko.md">한국어</a> |
  <a href="./README_zh-CN.md">简体中文</a> |
  <a href="./README_zh-TW.md">繁體中文</a> |
  <a href="./README_ar.md">العربية</a> |
  <a href="./README_hi.md">हिन्दी</a> |
  <a href="./README_nl.md">Nederlands</a> |
  <a href="./README_pl.md">Polski</a> |
  <a href="./README_tr.md">Türkçe</a> |
  <a href="./README_th.md">ไทย</a>
</p>

---

![Version](https://img.shields.io/badge/version-0.3.4-blue)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-green)

Trình tạo trang web tĩnh siêu nhanh được viết bằng Rust, tối ưu hóa cho blog và website nhiều nội dung.

## ✨ Tính năng

### Cốt lõi
- 🚀 **Siêu Nhanh** - Xử lý song song với Rayon
- 📝 **Hỗ trợ Markdown** - CommonMark đầy đủ với các tiện ích mở rộng
- 🔄 **Build Tăng dần** - Chỉ rebuild nội dung đã thay đổi
- 🎨 **Hệ thống Theme** - Theme kế thừa với template Tera
- 🔌 **Hệ thống Plugin** - Khả năng mở rộng dựa trên hook

### Nội dung
- 📰 **Bài viết & Trang** - Bài blog và trang tĩnh
- 🏷️ **Danh mục & Thẻ** - Hỗ trợ phân loại đầy đủ
- 🔗 **Bài viết liên quan** - Tự động tính toán theo thẻ/danh mục chung
- ⬅️➡️ **Điều hướng** - Liên kết bài trước/sau
- 📅 **Đăng theo lịch** - Đăng bài vào ngày/giờ cụ thể
- 🔒 **Xem trước bản nháp** - Xem bản nháp qua token

### SEO & Hiệu suất
- 🗺️ **Sitemap** - Tự động chia cho site lớn (5000 URL/file)
- 📡 **RSS/Atom Feeds** - Tự động tạo feeds
- 🔍 **Tìm kiếm** - Index tìm kiếm tĩnh phân mảnh
- 🖼️ **Tối ưu hình ảnh** - Tự động chuyển đổi WebP
- 📦 **Minification** - Nén CSS/JS
- 🔗 **Tự động liên kết URL** - URL được chuyển thành link có thể nhấp

---

## 🚀 Bắt đầu nhanh

### Cài đặt

```bash
# Clone repository
git clone https://github.com/your-username/explog.git
cd explog

# Build bản release
cargo build --release

# Binary sẽ ở: target/release/explog.exe (Windows) hoặc target/release/explog (Unix)
```

### Tạo bài viết đầu tiên

```bash
# Thư mục làm việc: thư mục gốc dự án (nơi có explog.toml)
./target/release/explog new post my-first-post
```

Lệnh này tạo:
```
content/posts/my-first-post/
├── index.md      # Nội dung bài viết
└── images/       # Hình ảnh riêng của bài
```

### Build & Xem trước

```bash
# Thư mục làm việc: thư mục gốc dự án (nơi có explog.toml)

# Build trang web
./target/release/explog build

# Khởi động dev server với hot-reload
./target/release/explog dev --port 3000
```

Mở `http://localhost:3000` trong trình duyệt.

---

## 📁 Cấu trúc dự án

```
explog/
├── content/
│   ├── posts/           # Bài viết blog (mỗi bài một thư mục)
│   └── pages/           # Trang tĩnh
├── themes/
│   └── default/         # Theme đang dùng
│       ├── theme.toml   # Cấu hình theme
│       ├── layouts/     # Template Tera
│       ├── assets/      # CSS/JS/Hình ảnh
│       └── core/        # CSS cốt lõi
├── plugins/             # Thư mục plugin
├── public/              # Output build (tự động tạo)
├── .cache/              # Cache build (tự động tạo)
└── explog.toml          # Cấu hình trang
```

---

## 💻 Lệnh CLI

> **Quan trọng:** Tất cả lệnh phải chạy từ thư mục gốc dự án (nơi có `explog.toml`).

### Lệnh Build

```bash
# Build đầy đủ
./target/release/explog build

# Buộc rebuild sạch (bỏ qua cache)
./target/release/explog build --clean

# Selective rebuild (chỉ rebuild nội dung cụ thể)
./target/release/explog build --page page-slug
./target/release/explog build --post post-slug
./target/release/explog build --category category-name
./target/release/explog build --tag tag-name
```

### Server phát triển

```bash
# Khởi động dev server (cổng mặc định 8080)
./target/release/explog dev

# Cổng tùy chỉnh
./target/release/explog dev --port 3000
```

Dev server:
- Phục vụ file từ thư mục `public/`
- Theo dõi thay đổi trong `content/`, `themes/`, và `explog.toml`
- Tự động rebuild khi file thay đổi

### Tạo nội dung

```bash
# Tạo bài viết mới
./target/release/explog new post my-post-slug

# Tạo trang mới
./target/release/explog new page about
```

### Quản lý Cache

```bash
# Xóa cache build
./target/release/explog clean
```

### Quản lý Plugin

```bash
# Liệt kê plugin đã cài
./target/release/explog plugin list

# Hiển thị chi tiết plugin
./target/release/explog plugin show plugin-name

# Tạo plugin mới
./target/release/explog plugin new my-plugin

# Xóa plugin
./target/release/explog plugin remove plugin-name
```

---

## ⚙️ Cấu hình

### Cấu hình trang (`explog.toml`)

```toml
[site]
title = "Blog của tôi"
description = "Blog cá nhân"
base_url = "https://example.com"
language = "vi"

[[site.navigation]]
label = "Trang chủ"
url = "/"

# Menu dropdown (với children)
[[site.navigation]]
label = "Chuyên mục"
url = "#"
children = [
    { label = "Blog", url = "/category/blog/" },
    { label = "Game", url = "/category/game/" },
    { label = "Công nghệ", url = "/category/tech/" }
]

[[site.navigation]]
label = "Giới thiệu"
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
bio = "Tác giả blog"

[build]
theme = "default"
output_dir = "public"
minify = true
strict_assets = false

[seo]
generate_sitemap = true
generate_rss = true
```

### Cấu hình Theme (`themes/default/theme.toml`)

```toml
[theme]
name = "default"
version = "1.0.0"
description = "Theme mặc định cho Explog CMS"
author = "Explog Team"
extends = ""  # Theme cha để kế thừa

[layout.home]
sections = ["hero", "featured_posts", "recent_posts"]
sidebar = true
widgets = ["search", "categories", "tags", "recent_posts"]

[settings]
posts_per_page = 12          # Số bài mỗi trang
related_posts_count = 4      # Số bài liên quan hiển thị
show_reading_time = true
show_author = true
show_date = true
show_categories = true
show_tags = true
```

---

## 📝 Frontmatter bài viết

```yaml
---
title: "Tiêu đề bài viết"
date: 2024-01-15
slug: "my-post-slug"           # Tùy chọn, mặc định là tên thư mục
categories:
  - "Công nghệ"
  - "Phát triển Web"
tags:
  - "rust"
  - "static-site"
summary: "Mô tả ngắn"           # Tùy chọn, tự động tạo nếu thiếu
cover: "images/cover.jpg"       # Đường dẫn tương đối với thư mục bài
featured: true                  # Hiển thị trong phần nổi bật
draft: false                    # Không đăng nếu true
author: "admin"                 # Phải khớp với [authors.id] trong explog.toml
publish_date: "2024-01-20T10:00:00Z"  # Đăng theo lịch
preview_token: "abc123"         # Truy cập bản nháp qua token
---

Nội dung markdown ở đây...
```

---

## 🎨 Phát triển Theme

Xem [docs/themes.md](../docs/themes.md) để có hướng dẫn phát triển theme đầy đủ.

### Tổng quan nhanh

```
themes/my-theme/
├── theme.toml           # Cấu hình theme
├── layouts/
│   ├── base.html        # Template cơ sở (header, footer)
│   ├── home.html        # Trang chủ
│   ├── post.html        # Bài viết đơn
│   ├── category.html    # Lưu trữ danh mục
│   ├── tag.html         # Lưu trữ thẻ
│   ├── page.html        # Trang tĩnh
│   └── components/      # Các component tái sử dụng
├── assets/
│   ├── css/
│   └── js/
└── core/                # CSS cốt lõi
```

---

## 🔌 Phát triển Plugin

Xem [docs/plugins.md](../docs/plugins.md) để có hướng dẫn phát triển plugin đầy đủ.

### Tổng quan nhanh

```bash
# Tạo plugin
./target/release/explog plugin new my-plugin
```

Lệnh này tạo:
```
plugins/my-plugin/
├── plugin.toml          # Manifest plugin
├── scripts/             # Script hook
│   └── after_build.bat  # Chạy sau khi build xong
└── README.md
```

---

## 📚 Tài liệu

| Tài liệu | Mô tả |
|----------|-------|
| [explog_architecture.md](../explog_architecture.md) | Kiến trúc kỹ thuật |
| [docs/plugins.md](../docs/plugins.md) | Hướng dẫn phát triển plugin |
| [docs/themes.md](../docs/themes.md) | Hướng dẫn phát triển theme |
| [docs/FRONTMATTER_SCHEMA.md](../docs/FRONTMATTER_SCHEMA.md) | Tham khảo frontmatter |

---

## 📄 Giấy phép

Giấy phép MIT - xem [LICENSE](../LICENSE) để biết chi tiết.

