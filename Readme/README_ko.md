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

Rust로 작성된 초고속 정적 사이트 생성기. 블로그 및 콘텐츠가 풍부한 웹사이트에 최적화되어 있습니다.

## ✨ 기능

### 핵심
- 🚀 **초고속** - Rayon을 이용한 병렬 처리
- 📝 **Markdown 지원** - 확장 기능이 있는 완전한 CommonMark
- 🔄 **증분 빌드** - 변경된 콘텐츠만 재빌드
- 🎨 **테마 시스템** - Tera 템플릿을 사용한 상속 가능한 테마
- 🔌 **플러그인 시스템** - 훅 기반 확장성

### 콘텐츠
- 📰 **게시물 및 페이지** - 블로그 게시물 및 정적 페이지
- 🏷️ **카테고리 및 태그** - 완전한 분류 지원
- 🔗 **관련 게시물** - 공유 태그/카테고리로 자동 계산
- ⬅️➡️ **탐색** - 이전/다음 게시물 링크
- 📅 **예약 게시** - 특정 날짜/시간에 게시물 게시
- 🔒 **초안 미리보기** - 토큰으로 초안 보기

### SEO 및 성능
- 🗺️ **사이트맵** - 자동 분할 (5000 URL/파일)
- 📡 **RSS/Atom 피드** - 자동 생성
- 🔍 **검색** - 정적 조각 검색 인덱스
- 🖼️ **이미지 최적화** - 자동 WebP 변환
- 📦 **압축** - CSS/JS 압축

---

## 🚀 빠른 시작

### 설치

```bash
# 저장소 복제
git clone https://github.com/your-username/explog.git
cd explog

# 릴리스 바이너리 빌드
cargo build --release
```

### 첫 번째 게시물 만들기

```bash
# 작업 디렉토리: 프로젝트 루트 (explog.toml이 있는 곳)
./target/release/explog new post my-first-post
```

### 빌드 및 미리보기

```bash
# 사이트 빌드
./target/release/explog build

# 핫 리로드가 있는 개발 서버 시작
./target/release/explog dev --port 3000
```

---

## ⚙️ 설정

### 사이트 설정 (`explog.toml`)

```toml
[site]
title = "내 블로그"
description = "개인 블로그"
base_url = "https://example.com"
language = "ko"

[[site.navigation]]
label = "홈"
url = "/"

# 드롭다운 메뉴 (하위 요소 포함)
[[site.navigation]]
label = "카테고리"
url = "#"
children = [
    { label = "블로그", url = "/category/blog/" },
    { label = "기술", url = "/category/tech/" },
    { label = "튜토리얼", url = "/category/tutorial/" }
]

[[site.navigation]]
label = "소개"
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

## 📚 문서

| 문서 | 설명 |
|-----|------|
| [explog_architecture.md](../explog_architecture.md) | 기술 아키텍처 |
| [docs/plugins.md](../docs/plugins.md) | 플러그인 개발 |
| [docs/themes.md](../docs/themes.md) | 테마 개발 |

---

## 📄 라이선스

MIT 라이선스 - 자세한 내용은 [LICENSE](../LICENSE)를 참조하세요.

