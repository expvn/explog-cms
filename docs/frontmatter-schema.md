# Explog Post Frontmatter Schema

## Cấu trúc chuẩn

```yaml
---
# ===== BẮT BUỘC =====
title: "Tiêu đề bài viết"
date: 2024-01-15                    # Ngày tạo (YYYY-MM-DD)

# ===== TỰ ĐỘNG (optional - sẽ generate nếu không có) =====
slug: "tieu-de-bai-viet"            # URL slug (auto từ folder name)

# ===== PHÂN LOẠI =====
categories:                          # Danh mục (1 hoặc nhiều)
  - Tech
  - Tutorial
tags:                                # Tags (0 hoặc nhiều)
  - rust
  - programming

# ===== HIỂN THỊ =====
summary: "Mô tả ngắn về bài viết"   # Auto extract nếu không có
cover: "cover.jpg"                   # Ảnh đại diện (trong cùng thư mục)
featured: false                      # Bài viết nổi bật

# ===== TRẠNG THÁI =====
draft: false                         # true = không xuất bản

# ===== LÊN LỊCH XUẤT BẢN (Phase 5) =====
publish_date: 2024-02-01T09:00:00   # Thời điểm xuất bản (ISO 8601)
                                     # Bài viết không hiển thị trước thời điểm này

# ===== XEM TRƯỚC BẢN NHÁP (Phase 5) =====
preview_token: "abc123"              # Token để xem trước bản nháp
                                     # Truy cập qua /preview/{token}

# ===== TÁC GIẢ =====
author: "admin"                      # ID tác giả

# ===== SEO (Optional) =====
seo:
  title: "Custom SEO Title"
  description: "Custom meta description"
  keywords:
    - keyword1
    - keyword2
  noindex: false

# ===== LIÊN QUAN (Optional) =====
related:                             # Bài viết liên quan (manual)
  - slug-bai-viet-1
  - slug-bai-viet-2
---
```

## Giá trị mặc định

| Trường | Mặc định | Mô tả |
|--------|----------|-------|
| `slug` | Tên thư mục | URL-friendly identifier |
| `categories` | `[]` | Danh mục bài viết |
| `tags` | `[]` | Tags bài viết |
| `summary` | 200 ký tự đầu | Tóm tắt bài viết |
| `cover` | `null` | Ảnh đại diện |
| `featured` | `false` | Bài viết nổi bật |
| `draft` | `false` | Bản nháp (không xuất bản) |
| `publish_date` | `null` | Xuất bản ngay lập tức |
| `preview_token` | `null` | Không có URL xem trước |
| `author` | `"admin"` | Tác giả mặc định |
| `seo` | `null` | Dùng title/summary mặc định |
| `related` | `[]` | Không có bài liên quan |

## Các trường mới (Phase 5)

### `publish_date` - Lên lịch xuất bản

Cho phép lên lịch xuất bản bài viết vào thời điểm cụ thể:

```yaml
publish_date: 2024-02-01T09:00:00   # Xuất bản lúc 9:00 ngày 1/2/2024
publish_date: 2024-02-01             # Xuất bản đầu ngày 1/2/2024
```

**Lưu ý:**
- Bài viết có `publish_date` trong tương lai sẽ **không hiển thị** khi build
- Sử dụng `--include-scheduled` để preview bài viết đã lên lịch

### `preview_token` - Xem trước bản nháp

Cho phép chia sẻ link xem trước cho bài viết draft:

```yaml
draft: true
preview_token: "secret-preview-abc123"
```

**Truy cập:** `/preview/secret-preview-abc123`

## Ví dụ đầy đủ

```yaml
---
title: "Hướng dẫn Rust cơ bản"
date: 2024-01-15
categories:
  - Tutorial
  - Programming
tags:
  - rust
  - beginner
summary: "Bài viết hướng dẫn cơ bản về ngôn ngữ Rust cho người mới bắt đầu"
cover: "rust-tutorial.png"
featured: true
draft: false
publish_date: 2024-01-20T08:00:00
author: "admin"
seo:
  title: "Học Rust từ A-Z - Hướng dẫn cho người mới"
  description: "Hướng dẫn chi tiết về Rust programming language"
related:
  - rust-advanced-concepts
  - rust-web-development
---

Nội dung bài viết...
```
