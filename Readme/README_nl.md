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

Een razendsnelle Static Site Generator geschreven in Rust, geoptimaliseerd voor blogs en content-rijke websites.

## ✨ Functies

### Kern
- 🚀 **Razend Snel** - Parallelle verwerking met Rayon
- 📝 **Markdown Ondersteuning** - Volledige CommonMark met extensies
- 🔄 **Incrementele Builds** - Alleen gewijzigde content herbouwen
- 🎨 **Themasysteem** - Erfbare thema's met Tera-templates
- 🔌 **Pluginsysteem** - Hook-gebaseerde uitbreidbaarheid

### Content
- 📰 **Posts en Pagina's** - Blogposts en statische pagina's
- 🏷️ **Categorieën en Tags** - Volledige taxonomie-ondersteuning
- 🔗 **Gerelateerde Posts** - Automatisch berekend op gedeelde tags/categorieën
- ⬅️➡️ **Navigatie** - Links naar vorige/volgende post
- 📅 **Geplande Publicatie** - Posts publiceren op specifieke datum/tijd
- 🔒 **Concept Preview** - Concepten bekijken via token

### SEO en Prestaties
- 🗺️ **Sitemap** - Automatisch gesplitst (5000 URLs/bestand)
- 📡 **RSS/Atom Feeds** - Automatisch gegenereerd
- 🔍 **Zoeken** - Statische gesegmenteerde zoekindex
- 🖼️ **Afbeeldingsoptimalisatie** - Automatische WebP-conversie
- 📦 **Minificatie** - CSS/JS minificatie

---

## 🚀 Snel Starten

### Installatie

```bash
# Repository klonen
git clone https://github.com/your-username/explog.git
cd explog

# Release binary bouwen
cargo build --release
```

### Eerste Post Maken

```bash
# Werkdirectory: projectroot (waar explog.toml staat)
./target/release/explog new post mijn-eerste-post
```

### Bouwen en Voorvertonen

```bash
# Site bouwen
./target/release/explog build

# Dev-server starten met hot-reload
./target/release/explog dev --port 3000
```

---

## ⚙️ Configuratie

### Site Configuratie (`explog.toml`)

```toml
[site]
title = "Mijn Blog"
description = "Een persoonlijke blog"
base_url = "https://example.com"
language = "nl"

[[site.navigation]]
label = "Home"
url = "/"

# Dropdown menu (met kinderen)
[[site.navigation]]
label = "Categorieën"
url = "#"
children = [
    { label = "Blog", url = "/category/blog/" },
    { label = "Technologie", url = "/category/tech/" },
    { label = "Tutorial", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "Over Ons"
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

## 📚 Documentatie

| Document | Beschrijving |
|----------|--------------|
| [explog_architecture.md](../explog_architecture.md) | Technische architectuur |
| [docs/plugins.md](../docs/plugins.md) | Plugin ontwikkeling |
| [docs/themes.md](../docs/themes.md) | Thema ontwikkeling |

---

## 📄 Licentie

MIT Licentie - zie [LICENSE](../LICENSE) voor details.

