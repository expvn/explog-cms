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

Un Générateur de Sites Statiques ultra-rapide écrit en Rust, optimisé pour les blogs et les sites web riches en contenu.

## ✨ Fonctionnalités

### Noyau
- 🚀 **Ultra Rapide** - Traitement parallèle avec Rayon
- 📝 **Support Markdown** - CommonMark complet avec extensions
- 🔄 **Builds Incrémentaux** - Ne reconstruit que le contenu modifié
- 🎨 **Système de Thèmes** - Thèmes héritables avec templates Tera
- 🔌 **Système de Plugins** - Extensibilité basée sur les hooks

### Contenu
- 📰 **Articles et Pages** - Articles de blog et pages statiques
- 🏷️ **Catégories et Tags** - Support taxonomique complet
- 🔗 **Articles Connexes** - Calculés automatiquement par tags/catégories partagés
- ⬅️➡️ **Navigation** - Liens article précédent/suivant
- 📅 **Publication Programmée** - Publiez des articles à une date/heure spécifique
- 🔒 **Aperçu des Brouillons** - Prévisualisez les brouillons via token

### SEO et Performance
- 🗺️ **Sitemap** - Divisé automatiquement pour les grands sites (5000 URLs/fichier)
- 📡 **Flux RSS/Atom** - Flux générés automatiquement
- 🔍 **Recherche** - Index de recherche statique fragmenté
- 🖼️ **Optimisation des Images** - Conversion automatique en WebP
- 📦 **Minification** - Minification CSS/JS
- 🔗 **Auto-lien des URLs** - URLs converties en liens cliquables

---

## 🚀 Démarrage Rapide

### Installation

```bash
# Cloner le dépôt
git clone https://github.com/your-username/explog.git
cd explog

# Compiler le binaire release
cargo build --release

# Le binaire sera à : target/release/explog.exe (Windows) ou target/release/explog (Unix)
```

### Créez Votre Premier Article

```bash
# Répertoire de travail : racine du projet (où se trouve explog.toml)
./target/release/explog new post mon-premier-article
```

Cela crée :
```
content/posts/mon-premier-article/
├── index.md      # Votre contenu d'article
└── images/       # Images spécifiques à l'article
```

### Compiler et Prévisualiser

```bash
# Répertoire de travail : racine du projet (où se trouve explog.toml)

# Compiler le site
./target/release/explog build

# Démarrer le serveur dev avec hot-reload
./target/release/explog dev --port 3000
```

Ouvrez `http://localhost:3000` dans votre navigateur.

---

## 📁 Structure du Projet

```
explog/
├── content/
│   ├── posts/           # Articles de blog (chacun dans son dossier)
│   └── pages/           # Pages statiques
├── themes/
│   └── default/         # Thème actif
│       ├── theme.toml   # Configuration du thème
│       ├── layouts/     # Templates Tera
│       ├── assets/      # CSS/JS/Images
│       └── core/        # Styles CSS principaux
├── plugins/             # Répertoire des plugins
├── public/              # Sortie de compilation (auto-généré)
├── .cache/              # Cache de compilation (auto-généré)
└── explog.toml          # Configuration du site
```

---

## 💻 Commandes CLI

> **Important :** Toutes les commandes doivent être exécutées depuis le répertoire racine du projet (où se trouve `explog.toml`).

### Commandes de Compilation

```bash
# Compilation complète
./target/release/explog build

# Forcer une reconstruction propre (ignorer le cache)
./target/release/explog build --clean
```

### Serveur de Développement

```bash
# Démarrer le serveur dev (port par défaut 8080)
./target/release/explog dev

# Port personnalisé
./target/release/explog dev --port 3000
```

Le serveur dev :
- Sert les fichiers depuis le répertoire `public/`
- Surveille les changements dans `content/`, `themes/`, et `explog.toml`
- Recompile automatiquement lors des changements

### Création de Contenu

```bash
# Créer un nouvel article
./target/release/explog new post mon-article-slug

# Créer une nouvelle page
./target/release/explog new page a-propos
```

### Gestion du Cache

```bash
# Vider le cache de compilation
./target/release/explog clean
```

### Gestion des Plugins

```bash
# Lister les plugins installés
./target/release/explog plugin list

# Afficher les détails du plugin
./target/release/explog plugin show nom-plugin

# Créer un nouveau plugin
./target/release/explog plugin new mon-plugin

# Supprimer un plugin
./target/release/explog plugin remove nom-plugin
```

---

## ⚙️ Configuration

### Configuration du Site (`explog.toml`)

```toml
[site]
title = "Mon Blog"
description = "Un blog personnel"
base_url = "https://example.com"
language = "fr"

[[site.navigation]]
label = "Accueil"
url = "/"

# Menu déroulant (avec children)
[[site.navigation]]
label = "Catégories"
url = "#"
children = [
    { label = "Blog", url = "/category/blog/" },
    { label = "Technologie", url = "/category/tech/" },
    { label = "Tutoriel", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "À Propos"
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
bio = "Auteur du blog"

[build]
theme = "default"
output_dir = "public"
minify = true
strict_assets = false

[seo]
generate_sitemap = true
generate_rss = true
```

---

## 📝 Frontmatter de l'Article

```yaml
---
title: "Titre de Mon Article"
date: 2024-01-15
slug: "mon-article-slug"        # Optionnel, par défaut le nom du dossier
categories:
  - "Technologie"
  - "Développement Web"
tags:
  - "rust"
  - "static-site"
summary: "Brève description"    # Optionnel, auto-généré si absent
cover: "images/cover.jpg"       # Relatif au dossier de l'article
featured: true                  # Afficher dans la section vedette
draft: false                    # Ne pas publier si true
author: "admin"                 # Doit correspondre à [authors.id] dans explog.toml
publish_date: "2024-01-20T10:00:00Z"  # Publication programmée
preview_token: "abc123"         # Accès au brouillon via token
---

Votre contenu markdown ici...
```

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [explog_architecture.md](../explog_architecture.md) | Architecture technique |
| [docs/plugins.md](../docs/plugins.md) | Guide de développement de plugins |
| [docs/themes.md](../docs/themes.md) | Guide de développement de thèmes |
| [docs/FRONTMATTER_SCHEMA.md](../docs/FRONTMATTER_SCHEMA.md) | Référence frontmatter |

---

## 📄 Licence

Licence MIT - voir [LICENSE](../LICENSE) pour les détails.

