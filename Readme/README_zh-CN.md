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

使用 Rust 编写的超快速静态网站生成器，针对博客和内容丰富的网站进行了优化。

## ✨ 功能

### 核心
- 🚀 **超快速** - 使用 Rayon 进行并行处理
- 📝 **Markdown 支持** - 完整的 CommonMark 及扩展
- 🔄 **增量构建** - 仅重建更改的内容
- 🎨 **主题系统** - 使用 Tera 模板的可继承主题
- 🔌 **插件系统** - 基于钩子的可扩展性

### 内容
- 📰 **文章和页面** - 博客文章和静态页面
- 🏷️ **分类和标签** - 完整的分类支持
- 🔗 **相关文章** - 根据共享标签/分类自动计算
- ⬅️➡️ **导航** - 上一篇/下一篇文章链接
- 📅 **定时发布** - 在特定日期/时间发布文章
- 🔒 **草稿预览** - 通过令牌查看草稿

### SEO 和性能
- 🗺️ **站点地图** - 自动分割（5000 URL/文件）
- 📡 **RSS/Atom 订阅** - 自动生成
- 🔍 **搜索** - 静态分片搜索索引
- 🖼️ **图片优化** - 自动 WebP 转换
- 📦 **压缩** - CSS/JS 压缩

---

## 🚀 快速开始

### 安装

```bash
# 克隆仓库
git clone https://github.com/your-username/explog.git
cd explog

# 构建发布版本
cargo build --release
```

### 创建第一篇文章

```bash
# 工作目录：项目根目录（explog.toml 所在位置）
./target/release/explog new post my-first-post
```

### 构建和预览

```bash
# 构建网站
./target/release/explog build

# 启动带热重载的开发服务器
./target/release/explog dev --port 3000
```

---

## ⚙️ 配置

### 站点配置 (`explog.toml`)

```toml
[site]
title = "我的博客"
description = "个人博客"
base_url = "https://example.com"
language = "zh-CN"

[[site.navigation]]
label = "首页"
url = "/"

# 下拉菜单（带子项）
[[site.navigation]]
label = "分类"
url = "#"
children = [
    { label = "博客", url = "/category/blog/" },
    { label = "技术", url = "/category/tech/" },
    { label = "教程", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "关于"
url = "/about/"

[build]
theme = "default"
output_dir = "public"
minify = true

[seo]
generate_sitemap = true
generate_rss = true
```

---

## 📚 文档

| 文档 | 描述 |
|-----|------|
| [explog_architecture.md](../explog_architecture.md) | 技术架构 |
| [docs/plugins.md](../docs/plugins.md) | 插件开发 |
| [docs/themes.md](../docs/themes.md) | 主题开发 |

---

## 📄 许可证

MIT 许可证 - 详见 [LICENSE](../LICENSE)。

