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

Un Generatore di Siti Statici ultra-veloce scritto in Rust, ottimizzato per blog e siti web ricchi di contenuti.

## ✨ Funzionalità

### Nucleo
- 🚀 **Ultra Veloce** - Elaborazione parallela con Rayon
- 📝 **Supporto Markdown** - CommonMark completo con estensioni
- 🔄 **Build Incrementali** - Ricostruisce solo i contenuti modificati
- 🎨 **Sistema di Temi** - Temi ereditabili con template Tera
- 🔌 **Sistema di Plugin** - Estensibilità basata su hook

### Contenuto
- 📰 **Post e Pagine** - Post del blog e pagine statiche
- 🏷️ **Categorie e Tag** - Supporto tassonomia completo
- 🔗 **Post Correlati** - Calcolati automaticamente per tag/categorie condivisi
- ⬅️➡️ **Navigazione** - Link post precedente/successivo
- 📅 **Pubblicazione Programmata** - Pubblica post a data/ora specifica
- 🔒 **Anteprima Bozze** - Visualizza bozze tramite token

### SEO e Performance
- 🗺️ **Sitemap** - Suddivisa automaticamente (5000 URL/file)
- 📡 **Feed RSS/Atom** - Generati automaticamente
- 🔍 **Ricerca** - Indice di ricerca statico frammentato
- 🖼️ **Ottimizzazione Immagini** - Conversione automatica WebP
- 📦 **Minificazione** - Minificazione CSS/JS

---

## 🚀 Avvio Rapido

### Installazione

```bash
# Clonare il repository
git clone https://github.com/your-username/explog.git
cd explog

# Compilare binario release
cargo build --release
```

### Crea il Tuo Primo Post

```bash
# Directory di lavoro: root del progetto (dove si trova explog.toml)
./target/release/explog new post mio-primo-post
```

### Compila e Anteprima

```bash
# Compilare il sito
./target/release/explog build

# Avviare server dev con hot-reload
./target/release/explog dev --port 3000
```

---

## ⚙️ Configurazione

### Configurazione Sito (`explog.toml`)

```toml
[site]
title = "Il Mio Blog"
description = "Un blog personale"
base_url = "https://example.com"
language = "it"

[[site.navigation]]
label = "Home"
url = "/"

# Menu a discesa (con figli)
[[site.navigation]]
label = "Categorie"
url = "#"
children = [
    { label = "Blog", url = "/category/blog/" },
    { label = "Tecnologia", url = "/category/tech/" },
    { label = "Tutorial", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "Chi Siamo"
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

## 📚 Documentazione

| Documento | Descrizione |
|-----------|-------------|
| [explog_architecture.md](../explog_architecture.md) | Architettura tecnica |
| [docs/plugins.md](../docs/plugins.md) | Sviluppo plugin |
| [docs/themes.md](../docs/themes.md) | Sviluppo temi |

---

## 📄 Licenza

Licenza MIT - vedere [LICENSE](../LICENSE) per i dettagli.

