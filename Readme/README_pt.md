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

Um Gerador de Sites Estáticos ultra-rápido escrito em Rust, otimizado para blogs e sites ricos em conteúdo.

## ✨ Funcionalidades

### Núcleo
- 🚀 **Ultra Rápido** - Processamento paralelo com Rayon
- 📝 **Suporte Markdown** - CommonMark completo com extensões
- 🔄 **Builds Incrementais** - Reconstrói apenas conteúdo modificado
- 🎨 **Sistema de Temas** - Temas herdáveis com templates Tera
- 🔌 **Sistema de Plugins** - Extensibilidade baseada em hooks

### Conteúdo
- 📰 **Posts e Páginas** - Posts de blog e páginas estáticas
- 🏷️ **Categorias e Tags** - Suporte completo de taxonomia
- 🔗 **Posts Relacionados** - Calculados automaticamente por tags/categorias
- ⬅️➡️ **Navegação** - Links para post anterior/próximo
- 📅 **Publicação Agendada** - Publique posts em data/hora específica
- 🔒 **Pré-visualização de Rascunhos** - Visualize rascunhos via token

### SEO e Performance
- 🗺️ **Sitemap** - Dividido automaticamente (5000 URLs/arquivo)
- 📡 **Feeds RSS/Atom** - Gerados automaticamente
- 🔍 **Busca** - Índice de busca estático fragmentado
- 🖼️ **Otimização de Imagens** - Conversão automática para WebP
- 📦 **Minificação** - Minificação de CSS/JS

---

## 🚀 Início Rápido

### Instalação

```bash
# Clonar o repositório
git clone https://github.com/your-username/explog.git
cd explog

# Compilar binário release
cargo build --release
```

### Crie Seu Primeiro Post

```bash
# Diretório de trabalho: raiz do projeto (onde está explog.toml)
./target/release/explog new post meu-primeiro-post
```

### Compilar e Pré-visualizar

```bash
# Compilar o site
./target/release/explog build

# Iniciar servidor dev com hot-reload
./target/release/explog dev --port 3000
```

---

## ⚙️ Configuração

### Configuração do Site (`explog.toml`)

```toml
[site]
title = "Meu Blog"
description = "Um blog pessoal"
base_url = "https://example.com"
language = "pt"

[[site.navigation]]
label = "Início"
url = "/"

# Menu dropdown (com filhos)
[[site.navigation]]
label = "Categorias"
url = "#"
children = [
    { label = "Blog", url = "/category/blog/" },
    { label = "Tecnologia", url = "/category/tech/" },
    { label = "Tutorial", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "Sobre"
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

## 📚 Documentação

| Documento | Descrição |
|-----------|-----------|
| [explog_architecture.md](../explog_architecture.md) | Arquitetura técnica |
| [docs/plugins.md](../docs/plugins.md) | Desenvolvimento de plugins |
| [docs/themes.md](../docs/themes.md) | Desenvolvimento de temas |

---

## 📄 Licença

Licença MIT - veja [LICENSE](../LICENSE) para detalhes.

