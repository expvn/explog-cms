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

使用 Rust 編寫的超快速靜態網站產生器，針對部落格和內容豐富的網站進行了優化。

## ✨ 功能

### 核心
- 🚀 **超快速** - 使用 Rayon 進行並行處理
- 📝 **Markdown 支援** - 完整的 CommonMark 及擴展
- 🔄 **增量建構** - 僅重建更改的內容
- 🎨 **主題系統** - 使用 Tera 模板的可繼承主題
- 🔌 **外掛系統** - 基於鉤子的可擴展性

### 內容
- 📰 **文章和頁面** - 部落格文章和靜態頁面
- 🏷️ **分類和標籤** - 完整的分類支援
- 🔗 **相關文章** - 根據共享標籤/分類自動計算
- ⬅️➡️ **導航** - 上一篇/下一篇文章連結
- 📅 **定時發布** - 在特定日期/時間發布文章
- 🔒 **草稿預覽** - 透過令牌查看草稿

### SEO 和效能
- 🗺️ **網站地圖** - 自動分割（5000 URL/檔案）
- 📡 **RSS/Atom 訂閱** - 自動產生
- 🔍 **搜尋** - 靜態分片搜尋索引
- 🖼️ **圖片優化** - 自動 WebP 轉換
- 📦 **壓縮** - CSS/JS 壓縮

---

## 🚀 快速開始

### 安裝

```bash
# 複製儲存庫
git clone https://github.com/your-username/explog.git
cd explog

# 建構發布版本
cargo build --release
```

### 建立第一篇文章

```bash
# 工作目錄：專案根目錄（explog.toml 所在位置）
./target/release/explog new post my-first-post
```

### 建構和預覽

```bash
# 建構網站
./target/release/explog build

# 啟動帶熱重載的開發伺服器
./target/release/explog dev --port 3000
```

---

## ⚙️ 設定

### 網站設定 (`explog.toml`)

```toml
[site]
title = "我的部落格"
description = "個人部落格"
base_url = "https://example.com"
language = "zh-TW"

[[site.navigation]]
label = "首頁"
url = "/"

# 下拉選單（帶子項）
[[site.navigation]]
label = "分類"
url = "#"
children = [
    { label = "部落格", url = "/category/blog/" },
    { label = "技術", url = "/category/tech/" },
    { label = "教學", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "關於"
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

## 📚 文件

| 文件 | 說明 |
|-----|------|
| [explog_architecture.md](../explog_architecture.md) | 技術架構 |
| [docs/plugins.md](../docs/plugins.md) | 外掛開發 |
| [docs/themes.md](../docs/themes.md) | 主題開發 |

---

## 📄 授權

MIT 授權 - 詳見 [LICENSE](../LICENSE)。

