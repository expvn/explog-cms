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

Rustで書かれた超高速静的サイトジェネレーター。ブログやコンテンツ豊富なウェブサイトに最適化。

## ✨ 機能

### コア
- 🚀 **超高速** - Rayonによる並列処理
- 📝 **Markdownサポート** - 拡張機能付きの完全なCommonMark
- 🔄 **インクリメンタルビルド** - 変更されたコンテンツのみを再構築
- 🎨 **テーマシステム** - Teraテンプレートを使用した継承可能なテーマ
- 🔌 **プラグインシステム** - フックベースの拡張性

### コンテンツ
- 📰 **投稿とページ** - ブログ投稿と静的ページ
- 🏷️ **カテゴリとタグ** - 完全な分類サポート
- 🔗 **関連投稿** - 共有タグ/カテゴリによる自動計算
- ⬅️➡️ **ナビゲーション** - 前/次の投稿へのリンク
- 📅 **予約投稿** - 特定の日時に投稿を公開
- 🔒 **下書きプレビュー** - トークンで下書きを表示

### SEOとパフォーマンス
- 🗺️ **サイトマップ** - 自動分割（5000 URL/ファイル）
- 📡 **RSS/Atomフィード** - 自動生成
- 🔍 **検索** - 静的フラグメント検索インデックス
- 🖼️ **画像最適化** - 自動WebP変換
- 📦 **圧縮** - CSS/JS圧縮

---

## 🚀 クイックスタート

### インストール

```bash
# リポジトリをクローン
git clone https://github.com/your-username/explog.git
cd explog

# リリースバイナリをビルド
cargo build --release
```

### 最初の投稿を作成

```bash
# 作業ディレクトリ: プロジェクトルート（explog.tomlがある場所）
./target/release/explog new post my-first-post
```

### ビルドとプレビュー

```bash
# サイトをビルド
./target/release/explog build

# ホットリロード付きdevサーバーを起動
./target/release/explog dev --port 3000
```

---

## ⚙️ 設定

### サイト設定 (`explog.toml`)

```toml
[site]
title = "私のブログ"
description = "個人ブログ"
base_url = "https://example.com"
language = "ja"

[[site.navigation]]
label = "ホーム"
url = "/"

# ドロップダウンメニュー（子要素付き）
[[site.navigation]]
label = "カテゴリ"
url = "#"
children = [
    { label = "ブログ", url = "/category/blog/" },
    { label = "テクノロジー", url = "/category/tech/" },
    { label = "チュートリアル", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "概要"
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

## 📚 ドキュメント

| ドキュメント | 説明 |
|------------|------|
| [explog_architecture.md](../explog_architecture.md) | 技術アーキテクチャ |
| [docs/plugins.md](../docs/plugins.md) | プラグイン開発 |
| [docs/themes.md](../docs/themes.md) | テーマ開発 |

---

## 📄 ライセンス

MITライセンス - 詳細は [LICENSE](../LICENSE) を参照。

