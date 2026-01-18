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

Ein blitzschneller Static Site Generator in Rust, optimiert für Blogs und inhaltsreiche Websites.

## ✨ Funktionen

### Kern
- 🚀 **Blitzschnell** - Parallele Verarbeitung mit Rayon
- 📝 **Markdown-Unterstützung** - Vollständiges CommonMark mit Erweiterungen
- 🔄 **Inkrementelle Builds** - Nur geänderte Inhalte neu erstellen
- 🎨 **Theme-System** - Vererbbare Themes mit Tera-Templates
- 🔌 **Plugin-System** - Hook-basierte Erweiterbarkeit

### Inhalt
- 📰 **Beiträge & Seiten** - Blog-Beiträge und statische Seiten
- 🏷️ **Kategorien & Tags** - Vollständige Taxonomie-Unterstützung
- 🔗 **Verwandte Beiträge** - Automatisch nach Tags/Kategorien berechnet
- ⬅️➡️ **Navigation** - Links zu vorherigem/nächstem Beitrag
- 📅 **Geplante Veröffentlichung** - Beiträge zeitgesteuert veröffentlichen
- 🔒 **Entwurfs-Vorschau** - Entwürfe über Token anzeigen

### SEO & Performance
- 🗺️ **Sitemap** - Automatisch aufgeteilt (5000 URLs/Datei)
- 📡 **RSS/Atom Feeds** - Automatisch generiert
- 🔍 **Suche** - Statischer fragmentierter Suchindex
- 🖼️ **Bildoptimierung** - Automatische WebP-Konvertierung
- 📦 **Minifizierung** - CSS/JS Minifizierung
- 🔗 **Auto-Verlinkung** - URLs automatisch in klickbare Links umwandeln

---

## 🚀 Schnellstart

### Installation

```bash
# Repository klonen
git clone https://github.com/your-username/explog.git
cd explog

# Release-Binary kompilieren
cargo build --release

# Binary befindet sich unter: target/release/explog.exe (Windows) oder target/release/explog (Unix)
```

### Ersten Beitrag erstellen

```bash
# Arbeitsverzeichnis: Projektwurzel (wo sich explog.toml befindet)
./target/release/explog new post mein-erster-beitrag
```

Dies erstellt:
```
content/posts/mein-erster-beitrag/
├── index.md      # Ihr Beitragsinhalt
└── images/       # Beitragsspezifische Bilder
```

### Kompilieren & Vorschau

```bash
# Arbeitsverzeichnis: Projektwurzel (wo sich explog.toml befindet)

# Seite kompilieren
./target/release/explog build

# Dev-Server mit Hot-Reload starten
./target/release/explog dev --port 3000
```

Öffnen Sie `http://localhost:3000` in Ihrem Browser.

---

## 📁 Projektstruktur

```
explog/
├── content/
│   ├── posts/           # Blog-Beiträge (jeder in eigenem Ordner)
│   └── pages/           # Statische Seiten
├── themes/
│   └── default/         # Aktives Theme
│       ├── theme.toml   # Theme-Konfiguration
│       ├── layouts/     # Tera-Templates
│       ├── assets/      # CSS/JS/Bilder
│       └── core/        # Kern-CSS-Stile
├── plugins/             # Plugin-Verzeichnis
├── public/              # Build-Ausgabe (automatisch generiert)
├── .cache/              # Build-Cache (automatisch generiert)
└── explog.toml          # Seitenkonfiguration
```

---

## 💻 CLI-Befehle

> **Wichtig:** Alle Befehle müssen vom Projektwurzelverzeichnis ausgeführt werden (wo sich `explog.toml` befindet).

### Kompilierungsbefehle

```bash
# Vollständige Kompilierung
./target/release/explog build

# Saubere Neukompilierung erzwingen (Cache ignorieren)
./target/release/explog build --clean
```

### Entwicklungsserver

```bash
# Dev-Server starten (Standardport 8080)
./target/release/explog dev

# Benutzerdefinierter Port
./target/release/explog dev --port 3000
```

### Inhaltserstellung

```bash
# Neuen Beitrag erstellen
./target/release/explog new post mein-beitrag-slug

# Neue Seite erstellen
./target/release/explog new page ueber-uns
```

### Plugin-Verwaltung

```bash
# Installierte Plugins auflisten
./target/release/explog plugin list

# Plugin-Details anzeigen
./target/release/explog plugin show plugin-name

# Neues Plugin erstellen
./target/release/explog plugin new mein-plugin

# Plugin entfernen
./target/release/explog plugin remove plugin-name
```

---

## ⚙️ Konfiguration

### Seitenkonfiguration (`explog.toml`)

```toml
[site]
title = "Mein Blog"
description = "Ein persönlicher Blog"
base_url = "https://example.com"
language = "de"

[[site.navigation]]
label = "Startseite"
url = "/"

# Dropdown-Menü (mit Untermenüs)
[[site.navigation]]
label = "Kategorien"
url = "#"
children = [
    { label = "Blog", url = "/category/blog/" },
    { label = "Technologie", url = "/category/tech/" },
    { label = "Tutorial", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "Über uns"
url = "/about/"

[[site.socials]]
platform = "github"
url = "https://github.com/username"

[authors.admin]
name = "Admin"
email = "admin@example.com"
bio = "Blog-Autor"

[build]
theme = "default"
output_dir = "public"
minify = true

[seo]
generate_sitemap = true
generate_rss = true
```

---

## 📝 Beitrags-Frontmatter

```yaml
---
title: "Mein Beitragstitel"
date: 2024-01-15
slug: "mein-beitrag-slug"
categories:
  - "Technologie"
  - "Webentwicklung"
tags:
  - "rust"
  - "static-site"
summary: "Kurze Beschreibung"
cover: "images/cover.jpg"
featured: true
draft: false
author: "admin"
---

Ihr Markdown-Inhalt hier...
```

---

## 📚 Dokumentation

| Dokument | Beschreibung |
|----------|--------------|
| [explog_architecture.md](../explog_architecture.md) | Technische Architektur |
| [docs/plugins.md](../docs/plugins.md) | Plugin-Entwicklung |
| [docs/themes.md](../docs/themes.md) | Theme-Entwicklung |
| [docs/FRONTMATTER_SCHEMA.md](../docs/FRONTMATTER_SCHEMA.md) | Frontmatter-Referenz |

---

## 📄 Lizenz

MIT-Lizenz - siehe [LICENSE](../LICENSE) für Details.

