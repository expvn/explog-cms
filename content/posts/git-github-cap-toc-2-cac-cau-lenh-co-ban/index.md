---
title: "[GIT/GITHUB CẤP TỐC #2] – TỪ ĐIỂN CÂU LỆNH GIT (SẮP XẾP THEO TẦN SUẤT)"
date: 2023-07-25
slug: "git-github-cap-toc-2-cac-cau-lenh-co-ban"
categories:
  - "Develop"
tags:
  - "git-commands"
  - "git-cheat-sheet"
summary: "Tổng hợp các câu lệnh Git quan trọng nhất, được giải thích chi tiết theo hướng tài liệu kỹ thuật và sắp xếp từ cơ bản đến nâng cao."
cover: "images/22-221456_git-branch.jpg"
featured: false
draft: false
author: "admin"
---

Bài viết này đóng vai trò như một cuốn từ điển tra cứu nhanh. Các câu lệnh được sắp xếp theo mức độ thường xuyên sử dụng: từ những lệnh bạn dùng hàng ngày đến những lệnh dùng để quản trị hoặc xử lý sự cố.

---

### 1. Nhóm lệnh làm việc hàng ngày (Daily Workflow)
Đây là những lệnh bạn sẽ gõ hàng chục lần mỗi ngày khi code.

#### `git status`
*   **Ý nghĩa:** Kiểm tra trạng thái hiện tại của kho mã.
*   **Khi nào dùng:** Bất cứ lúc nào bạn muốn biết: "Mình vừa sửa file nào?", "File nào chưa được theo dõi?", "Mình đang ở nhánh nào?".
*   **Sử dụng:**
    ```bash
    git status
    ```

#### `git add`
*   **Ý nghĩa:** Đưa file đã chỉnh sửa vào vùng chờ (Staging Area) để chuẩn bị commit.
*   **Khi nào dùng:** Khi bạn đã code xong một tính năng hoặc sửa xong một lỗi và muốn "đóng gói" nó.
*   **Sử dụng:**
    ```bash
    git add .           # Chọn tất cả các file đã sửa
    git add index.html  # Chỉ chọn file index.html
    ```

#### `git commit`
*   **Ý nghĩa:** Lưu lại trạng thái của vùng chờ vào lịch sử (tạo một mốc thời gian mới).
*   **Khi nào dùng:** Ngay sau khi `git add`. Mỗi commit nên là một đơn vị công việc hoàn chỉnh (VD: "Sửa lỗi header", không phải "Sửa linh tinh").
*   **Sử dụng:**
    ```bash
    git commit -m "Mô tả ngắn gọn việc bạn vừa làm"
    ```

#### `git push`
*   **Ý nghĩa:** Đẩy các commit từ máy tính cá nhân (Local) lên server (Remote/GitHub).
*   **Khi nào dùng:** Khi bạn muốn lưu trữ code an toàn trên mạng hoặc chia sẻ code cho thành viên khác.
*   **Sử dụng:**
    ```bash
    git push origin main        # Đẩy nhánh main lên server
    git push -u origin <ten-nhanh> # Lần đầu đẩy một nhánh mới lên
    ```

#### `git pull`
*   **Ý nghĩa:** Tải code mới nhất từ server về và tự động gộp vào code ở máy.
*   **Khi nào dùng:** Trước khi bắt đầu ngày làm việc mới, hoặc khi đồng nghiệp bảo "Tao vừa fix lỗi rồi, kéo về đi".
*   **Sử dụng:**
    ```bash
    git pull origin main
    ```

---

### 2. Nhóm lệnh khởi tạo & Cấu hình (Setup)
Thường chỉ dùng 1 lần đầu dự án hoặc khi cài lại máy.

#### `git clone`
*   **Ý nghĩa:** Sao chép một dự án (repo) từ trên mạng về máy tính.
*   **Khi nào dùng:** Khi bạn vào một dự án mới hoặc muốn tải code mẫu về xem.
*   **Sử dụng:**
    ```bash
    git clone https://github.com/username/project.git
    ```

#### `git init`
*   **Ý nghĩa:** Biến một thư mục bình thường thành một kho chứa Git.
*   **Khi nào dùng:** Khi bạn tạo dự án mới từ con số 0 ngay trên máy tính của mình (chưa có trên GitHub).
*   **Sử dụng:**
    ```bash
    git init
    ```

#### `git remote`
*   **Ý nghĩa:** Quản lý kết nối tới server (GitHub/GitLab).
*   **Khi nào dùng:** Khi bạn `git init` và muốn liên kết nó với một repo rỗng trên GitHub.
*   **Sử dụng:**
    ```bash
    git remote add origin https://github.com/username/project.git # Thêm liên kết
    git remote -v  # Xem danh sách các liên kết hiện có
    ```

#### `git config`
*   **Ý nghĩa:** Cài đặt thông tin người dùng.
*   **Khi nào dùng:** Lần đầu tiên cài Git trên máy mới.
*   **Sử dụng:**
    ```bash
    git config --global user.name "Ten Cua Ban"
    git config --global user.email "email@example.com"
    ```

---

### 3. Nhóm lệnh quản lý nhánh (Branching)
Dùng để chia công việc, tránh dẫm đạp lên nhau.

#### `git branch`
*   **Ý nghĩa:** Liệt kê, tạo hoặc xóa nhánh.
*   **Sử dụng:**
    ```bash
    git branch              # Xem danh sách các nhánh
    git branch <ten-moi>    # Tạo nhánh mới (nhưng chưa chuyển sang)
    git branch -d <ten-cu>  # Xóa một nhánh
    ```

#### `git checkout` (hoặc `git switch`)
*   **Ý nghĩa:** Chuyển đổi qua lại giữa các nhánh.
*   **Sử dụng:**
    ```bash
    git checkout main          # Chuyển về nhánh main
    git checkout -b feature-A  # Tạo nhánh feature-A và chuyển sang đó luôn
    ```
    *Lưu ý: Git bản mới khuyến khích dùng `git switch` để chuyển nhánh cho rõ nghĩa hơn.*

---

### 4. Nhóm lệnh hợp nhất & Lịch sử (Advanced)

#### `git merge`
*   **Ý nghĩa:** Gộp code từ nhánh khác vào nhánh hiện tại.
*   **Khi nào dùng:** Khi bạn (đang ở nhánh `main`) muốn gộp tính năng từ nhánh `feature-A` vào để phát hành.
*   **Sử dụng:**
    ```bash
    git merge feature-A
    ```

#### `git log`
*   **Ý nghĩa:** Xem nhật ký lịch sử.
*   **Khi nào dùng:** Khi muốn xem ai vừa làm hỏng code, hoặc xem lại 1 tuần qua nhóm đã làm được gì.
*   **Sử dụng:**
    ```bash
    git log           # Xem chi tiết
    git log --oneline # Xem gọn (mỗi commit 1 dòng)
    ```

#### `git stash`
*   **Ý nghĩa:** "Cất tạm" các thay đổi chưa commit vào ngăn kéo.
*   **Khi nào dùng:** Bạn đang code dở tính năng A (chưa xong nên chưa muốn commit) nhưng sếp bắt phải sửa gấp lỗi ở nhánh B. Bạn "stash" code A lại -> sửa B -> quay lại lôi code A ra làm tiếp.
*   **Sử dụng:**
    ```bash
    git stash        # Cất đi
    git stash pop    # Lôi ra trả lại
    ```

#### `git rebase`
*   **Ý nghĩa:** Viết lại lịch sử commit (thường dùng để gộp nhánh một cách "sạch sẽ" hơn merge).
*   **Khi nào dùng:** Khi bạn muốn nhánh của mình cập nhật theo code mới nhất của `main` mà không tạo ra các "merge commit" rác.
*   **Sử dụng:**
    ```bash
    git checkout feature-A
    git rebase main   # Đưa toàn bộ nhánh feature-A nối tiếp vào sau main
    ```

Hy vọng "cuốn từ điển" này sẽ giúp bạn tra cứu nhanh chóng trong quá trình làm việc với Git!
