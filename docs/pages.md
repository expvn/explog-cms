# Pages Guide

Hướng dẫn tạo và quản lý trang tĩnh trong Explog CMS.

## Cấu trúc thư mục

```
content/pages/
├── about/
│   ├── page.json      # BẮT BUỘC
│   └── index.md       # cho type: markdown
├── test-webgl/
│   ├── page.json
│   ├── index.html
│   └── Build/         # WebGL assets
└── expgames/
    ├── page.json
    └── cover1.jpg
```

## page.json Schema

Mỗi trang **BẮT BUỘC** phải có file `page.json`:

```json
{
    "title": "Tên trang",
    "type": "markdown | html | gallery",
    "mode": "embedded | standalone",
    "description": "Mô tả ngắn (optional)",
    "slug": "custom-url (optional)",
    "template": "custom-template (optional)",
    
    "embed": {
        "src": "index.html",
        "width": "960px",
        "height": "650px"
    },
    
    "items": [
        {
            "cover": "cover.jpg",
            "title": "Item title",
            "description": "Optional desc",
            "url": "/target-page/"
        }
    ]
}
```

## Types (NỘI DUNG là gì)

| Type | Mô tả | Content file |
|------|-------|--------------|
| `markdown` | Parse markdown → HTML | index.md |
| `html` | Đọc HTML nguyên | index.html |
| `gallery` | Render items grid | (từ page.json) |

## Modes (HIỂN THỊ như thế nào)

| Mode | Header/Footer | Khi nào dùng |
|------|---------------|--------------|
| `embedded` | ✅ Có theme | Phần lớn trang |
| `standalone` | ❌ Không theme | Landing page, micro-site |

## Ví dụ

### Trang Markdown (About, Contact)
```json
{
    "title": "Giới thiệu",
    "type": "markdown",
    "mode": "embedded",
    "description": "Trang giới thiệu"
}
```

### WebGL Game
```json
{
    "title": "Space Survival",
    "type": "html",
    "mode": "embedded",
    "embed": {
        "src": "index.html",
        "width": "960px",
        "height": "650px"
    }
}
```

### Landing Page (Standalone)
```json
{
    "title": "Landing",
    "type": "html",
    "mode": "standalone"
}
```

### Gallery
```json
{
    "title": "EXPGAMES",
    "type": "gallery",
    "mode": "embedded",
    "items": [
        {
            "cover": "cover1.jpg",
            "title": "Game 1",
            "url": "/test-webgl/"
        }
    ]
}
```

## URL Output

Tất cả pages: `/{slug}/` (slug = folder name hoặc override)
