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

Błyskawiczny generator stron statycznych napisany w Rust, zoptymalizowany dla blogów i stron bogatych w treści.

## ✨ Funkcje

### Rdzeń
- 🚀 **Błyskawiczny** - Równoległe przetwarzanie z Rayon
- 📝 **Obsługa Markdown** - Pełny CommonMark z rozszerzeniami
- 🔄 **Budowanie Przyrostowe** - Przebudowa tylko zmienionej zawartości
- 🎨 **System Motywów** - Dziedziczne motywy z szablonami Tera
- 🔌 **System Wtyczek** - Rozszerzalność oparta na hookach

### Treść
- 📰 **Posty i Strony** - Posty blogowe i strony statyczne
- 🏷️ **Kategorie i Tagi** - Pełna obsługa taksonomii
- 🔗 **Powiązane Posty** - Automatycznie obliczane według wspólnych tagów/kategorii
- ⬅️➡️ **Nawigacja** - Linki do poprzedniego/następnego postu
- 📅 **Zaplanowane Publikowanie** - Publikuj posty w określonym czasie
- 🔒 **Podgląd Wersji Roboczych** - Przeglądaj wersje robocze przez token

### SEO i Wydajność
- 🗺️ **Mapa Strony** - Automatyczny podział (5000 URL/plik)
- 📡 **Kanały RSS/Atom** - Automatycznie generowane
- 🔍 **Wyszukiwanie** - Statyczny podzielony indeks wyszukiwania
- 🖼️ **Optymalizacja Obrazów** - Automatyczna konwersja WebP
- 📦 **Minifikacja** - Minifikacja CSS/JS

---

## 🚀 Szybki Start

### Instalacja

```bash
# Klonuj repozytorium
git clone https://github.com/your-username/explog.git
cd explog

# Zbuduj wersję release
cargo build --release
```

### Utwórz Pierwszy Post

```bash
# Katalog roboczy: korzeń projektu (gdzie znajduje się explog.toml)
./target/release/explog new post moj-pierwszy-post
```

### Buduj i Podejrzyj

```bash
# Zbuduj stronę
./target/release/explog build

# Uruchom serwer dev z hot-reload
./target/release/explog dev --port 3000
```

---

## ⚙️ Konfiguracja

### Konfiguracja Strony (`explog.toml`)

```toml
[site]
title = "Mój Blog"
description = "Osobisty blog"
base_url = "https://example.com"
language = "pl"

[[site.navigation]]
label = "Strona Główna"
url = "/"

# Menu rozwijane (z dziećmi)
[[site.navigation]]
label = "Kategorie"
url = "#"
children = [
    { label = "Blog", url = "/category/blog/" },
    { label = "Technologia", url = "/category/tech/" },
    { label = "Poradnik", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "O Nas"
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

## 📚 Dokumentacja

| Dokument | Opis |
|----------|------|
| [explog_architecture.md](../explog_architecture.md) | Architektura techniczna |
| [docs/plugins.md](../docs/plugins.md) | Rozwój wtyczek |
| [docs/themes.md](../docs/themes.md) | Rozwój motywów |

---

## 📄 Licencja

Licencja MIT - zobacz [LICENSE](../LICENSE) po szczegóły.

