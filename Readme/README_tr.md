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

Rust ile yazılmış son derece hızlı bir Statik Site Oluşturucu, bloglar ve içerik yoğun web siteleri için optimize edilmiştir.

## ✨ Özellikler

### Çekirdek
- 🚀 **Son Derece Hızlı** - Rayon ile paralel işleme
- 📝 **Markdown Desteği** - Uzantılarla tam CommonMark
- 🔄 **Artımlı Derleme** - Yalnızca değişen içeriği yeniden derle
- 🎨 **Tema Sistemi** - Tera şablonlarıyla miras alınabilir temalar
- 🔌 **Eklenti Sistemi** - Hook tabanlı genişletilebilirlik

### İçerik
- 📰 **Yazılar ve Sayfalar** - Blog yazıları ve statik sayfalar
- 🏷️ **Kategoriler ve Etiketler** - Tam taksonomi desteği
- 🔗 **İlişkili Yazılar** - Paylaşılan etiketler/kategorilere göre otomatik hesaplama
- ⬅️➡️ **Gezinme** - Önceki/sonraki yazı bağlantıları
- 📅 **Zamanlanmış Yayınlama** - Belirli tarih/saatte yazı yayınla
- 🔒 **Taslak Önizleme** - Token ile taslakları görüntüle

### SEO ve Performans
- 🗺️ **Site Haritası** - Otomatik bölme (5000 URL/dosya)
- 📡 **RSS/Atom Beslemeleri** - Otomatik oluşturma
- 🔍 **Arama** - Statik parçalı arama dizini
- 🖼️ **Görsel Optimizasyonu** - Otomatik WebP dönüşümü
- 📦 **Küçültme** - CSS/JS küçültme

---

## 🚀 Hızlı Başlangıç

### Kurulum

```bash
# Depoyu klonla
git clone https://github.com/your-username/explog.git
cd explog

# Release binary derle
cargo build --release
```

### İlk Yazınızı Oluşturun

```bash
# Çalışma dizini: proje kökü (explog.toml'un bulunduğu yer)
./target/release/explog new post ilk-yazim
```

### Derle ve Önizle

```bash
# Siteyi derle
./target/release/explog build

# Hot-reload ile dev sunucusunu başlat
./target/release/explog dev --port 3000
```

---

## ⚙️ Yapılandırma

### Site Yapılandırması (`explog.toml`)

```toml
[site]
title = "Blogum"
description = "Kişisel bir blog"
base_url = "https://example.com"
language = "tr"

[[site.navigation]]
label = "Ana Sayfa"
url = "/"

# Açılır menü (alt öğelerle)
[[site.navigation]]
label = "Kategoriler"
url = "#"
children = [
    { label = "Blog", url = "/category/blog/" },
    { label = "Teknoloji", url = "/category/tech/" },
    { label = "Eğitim", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "Hakkında"
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

## 📚 Dokümantasyon

| Belge | Açıklama |
|-------|----------|
| [explog_architecture.md](../explog_architecture.md) | Teknik mimari |
| [docs/plugins.md](../docs/plugins.md) | Eklenti geliştirme |
| [docs/themes.md](../docs/themes.md) | Tema geliştirme |

---

## 📄 Lisans

MIT Lisansı - detaylar için [LICENSE](../LICENSE) dosyasına bakın.

