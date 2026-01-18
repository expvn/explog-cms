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

مولد مواقع ثابتة فائق السرعة مكتوب بلغة Rust، محسّن للمدونات والمواقع الغنية بالمحتوى.

## ✨ الميزات

### الأساسية
- 🚀 **فائق السرعة** - معالجة متوازية مع Rayon
- 📝 **دعم Markdown** - CommonMark كامل مع الإضافات
- 🔄 **بناء تدريجي** - إعادة بناء المحتوى المتغير فقط
- 🎨 **نظام السمات** - سمات قابلة للتوريث مع قوالب Tera
- 🔌 **نظام الإضافات** - قابلية التوسع المبنية على الخطافات

### المحتوى
- 📰 **المنشورات والصفحات** - منشورات المدونة والصفحات الثابتة
- 🏷️ **التصنيفات والوسوم** - دعم كامل للتصنيف
- 🔗 **المنشورات ذات الصلة** - محسوبة تلقائيًا حسب الوسوم/التصنيفات المشتركة
- ⬅️➡️ **التنقل** - روابط المنشور السابق/التالي
- 📅 **النشر المجدول** - نشر المنشورات في تاريخ/وقت محدد
- 🔒 **معاينة المسودات** - عرض المسودات عبر الرمز

### SEO والأداء
- 🗺️ **خريطة الموقع** - تقسيم تلقائي (5000 رابط/ملف)
- 📡 **خلاصات RSS/Atom** - إنشاء تلقائي
- 🔍 **البحث** - فهرس بحث ثابت مجزأ
- 🖼️ **تحسين الصور** - تحويل WebP تلقائي
- 📦 **الضغط** - ضغط CSS/JS

---

## 🚀 البدء السريع

### التثبيت

```bash
# استنساخ المستودع
git clone https://github.com/your-username/explog.git
cd explog

# بناء نسخة الإصدار
cargo build --release
```

### إنشاء أول منشور

```bash
# دليل العمل: جذر المشروع (حيث يوجد explog.toml)
./target/release/explog new post my-first-post
```

### البناء والمعاينة

```bash
# بناء الموقع
./target/release/explog build

# بدء خادم التطوير مع إعادة التحميل التلقائي
./target/release/explog dev --port 3000
```

---

## ⚙️ الإعداد

### إعداد الموقع (`explog.toml`)

```toml
[site]
title = "مدونتي"
description = "مدونة شخصية"
base_url = "https://example.com"
language = "ar"

[[site.navigation]]
label = "الرئيسية"
url = "/"

# القائمة المنسدلة (مع العناصر الفرعية)
[[site.navigation]]
label = "التصنيفات"
url = "#"
children = [
    { label = "مدونة", url = "/category/blog/" },
    { label = "تقنية", url = "/category/tech/" },
    { label = "دروس", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "حول"
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

## 📚 التوثيق

| المستند | الوصف |
|---------|-------|
| [explog_architecture.md](../explog_architecture.md) | البنية التقنية |
| [docs/plugins.md](../docs/plugins.md) | تطوير الإضافات |
| [docs/themes.md](../docs/themes.md) | تطوير السمات |

---

## 📄 الترخيص

ترخيص MIT - انظر [LICENSE](../LICENSE) للتفاصيل.

