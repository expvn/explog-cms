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

Un Generador de Sitios Estáticos ultrarrápido escrito en Rust, optimizado para blogs y sitios web con mucho contenido.

## ✨ Características

### Núcleo
- 🚀 **Ultrarrápido** - Procesamiento paralelo con Rayon
- 📝 **Soporte Markdown** - CommonMark completo con extensiones
- 🔄 **Builds Incrementales** - Solo reconstruye contenido modificado
- 🎨 **Sistema de Temas** - Temas heredables con plantillas Tera
- 🔌 **Sistema de Plugins** - Extensibilidad basada en hooks

### Contenido
- 📰 **Posts y Páginas** - Publicaciones de blog y páginas estáticas
- 🏷️ **Categorías y Etiquetas** - Soporte completo de taxonomía
- 🔗 **Posts Relacionados** - Calculados automáticamente por etiquetas/categorías compartidas
- ⬅️➡️ **Navegación** - Enlaces a post anterior/siguiente
- 📅 **Publicación Programada** - Publica posts en fecha/hora específica
- 🔒 **Vista Previa de Borradores** - Previsualiza borradores mediante token

### SEO y Rendimiento
- 🗺️ **Sitemap** - Dividido automáticamente para sitios grandes (5000 URLs/archivo)
- 📡 **Feeds RSS/Atom** - Feeds generados automáticamente
- 🔍 **Búsqueda** - Índice de búsqueda estático fragmentado
- 🖼️ **Optimización de Imágenes** - Conversión automática a WebP
- 📦 **Minificación** - Minificación de CSS/JS
- 🔗 **Auto-enlace de URLs** - URLs convertidas a enlaces clicables

---

## 🚀 Inicio Rápido

### Instalación

```bash
# Clonar el repositorio
git clone https://github.com/your-username/explog.git
cd explog

# Compilar binario release
cargo build --release

# El binario estará en: target/release/explog.exe (Windows) o target/release/explog (Unix)
```

### Crea tu Primer Post

```bash
# Directorio de trabajo: raíz del proyecto (donde está explog.toml)
./target/release/explog new post mi-primer-post
```

Esto crea:
```
content/posts/mi-primer-post/
├── index.md      # Tu contenido del post
└── images/       # Imágenes específicas del post
```

### Compilar y Previsualizar

```bash
# Directorio de trabajo: raíz del proyecto (donde está explog.toml)

# Compilar el sitio
./target/release/explog build

# Iniciar servidor dev con hot-reload
./target/release/explog dev --port 3000
```

Abre `http://localhost:3000` en tu navegador.

---

## 📁 Estructura del Proyecto

```
explog/
├── content/
│   ├── posts/           # Posts del blog (cada uno en su carpeta)
│   └── pages/           # Páginas estáticas
├── themes/
│   └── default/         # Tema activo
│       ├── theme.toml   # Configuración del tema
│       ├── layouts/     # Plantillas Tera
│       ├── assets/      # CSS/JS/Imágenes
│       └── core/        # Estilos CSS principales
├── plugins/             # Directorio de plugins
├── public/              # Salida de compilación (auto-generado)
├── .cache/              # Caché de compilación (auto-generado)
└── explog.toml          # Configuración del sitio
```

---

## 💻 Comandos CLI

> **Importante:** Todos los comandos deben ejecutarse desde el directorio raíz del proyecto (donde está `explog.toml`).

### Comandos de Compilación

```bash
# Compilación completa
./target/release/explog build

# Forzar reconstrucción limpia (ignorar caché)
./target/release/explog build --clean
```

### Servidor de Desarrollo

```bash
# Iniciar servidor dev (puerto predeterminado 8080)
./target/release/explog dev

# Puerto personalizado
./target/release/explog dev --port 3000
```

El servidor dev:
- Sirve archivos desde el directorio `public/`
- Observa cambios en `content/`, `themes/`, y `explog.toml`
- Recompila automáticamente cuando hay cambios

### Creación de Contenido

```bash
# Crear nuevo post
./target/release/explog new post mi-post-slug

# Crear nueva página
./target/release/explog new page acerca-de
```

### Gestión de Caché

```bash
# Limpiar caché de compilación
./target/release/explog clean
```

### Gestión de Plugins

```bash
# Listar plugins instalados
./target/release/explog plugin list

# Mostrar detalles del plugin
./target/release/explog plugin show nombre-plugin

# Crear nuevo plugin
./target/release/explog plugin new mi-plugin

# Eliminar plugin
./target/release/explog plugin remove nombre-plugin
```

---

## ⚙️ Configuración

### Configuración del Sitio (`explog.toml`)

```toml
[site]
title = "Mi Blog"
description = "Un blog personal"
base_url = "https://example.com"
language = "es"

[[site.navigation]]
label = "Inicio"
url = "/"

# Menú desplegable (con children)
[[site.navigation]]
label = "Categorías"
url = "#"
children = [
    { label = "Blog", url = "/category/blog/" },
    { label = "Tecnología", url = "/category/tech/" },
    { label = "Tutorial", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "Acerca de"
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
bio = "Autor del blog"

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

## 📝 Frontmatter del Post

```yaml
---
title: "Título de Mi Post"
date: 2024-01-15
slug: "mi-post-slug"           # Opcional, predeterminado al nombre de la carpeta
categories:
  - "Tecnología"
  - "Desarrollo Web"
tags:
  - "rust"
  - "static-site"
summary: "Breve descripción"    # Opcional, auto-generado si falta
cover: "images/cover.jpg"       # Relativo a la carpeta del post
featured: true                  # Mostrar en sección destacada
draft: false                    # No publicar si es true
author: "admin"                 # Debe coincidir con [authors.id] en explog.toml
publish_date: "2024-01-20T10:00:00Z"  # Publicación programada
preview_token: "abc123"         # Acceso a borrador vía token
---

Tu contenido markdown aquí...
```

---

## 📚 Documentación

| Documento | Descripción |
|-----------|-------------|
| [explog_architecture.md](../explog_architecture.md) | Arquitectura técnica |
| [docs/plugins.md](../docs/plugins.md) | Guía de desarrollo de plugins |
| [docs/themes.md](../docs/themes.md) | Guía de desarrollo de temas |
| [docs/FRONTMATTER_SCHEMA.md](../docs/FRONTMATTER_SCHEMA.md) | Referencia de frontmatter |

---

## 📄 Licencia

Licencia MIT - ver [LICENSE](../LICENSE) para más detalles.

