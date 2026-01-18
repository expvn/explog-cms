---
title: "[GIT/GITHUB CẤP TỐC #1] - GIỚI THIỆU & QUY TRÌNH LÀM VIỆC NHÓM"
date: 2023-07-23
slug: "git-github-cap-toc-1-gioi-thieu-ve-git-github"
categories:
  - "Develop"
tags:
  - "git"
  - "github"
  - "version-control"
summary: "Tìm hiểu Git/GitHub qua câu chuyện làm việc nhóm thực tế: Từ khởi tạo dự án, xử lý xung đột đến chiến lược phân nhánh (branching)."
cover: "images/22-221456_git-branch.jpg"
featured: false
draft: false
author: "admin"
---

Chào các bạn, nếu bạn đang loay hoay không biết làm thế nào để nhiều người cùng code chung một dự án mà không "đá" nhau, thì **Git** và **GitHub** chính là câu trả lời.

*   **Git**: Công cụ cài trên máy để quản lý lịch sử code (như cỗ máy thời gian).
*   **GitHub**: Kho chứa code trên mạng (đám mây), nơi mọi người gửi code lên và kéo code về.

Để dễ hiểu, chúng ta hãy theo dõi câu chuyện của một nhóm phát triển ứng dụng **"SuperTodo"**.

**Nhân vật:**
*   **Tuấn (Leader):** Trưởng nhóm, chịu trách nhiệm quản lý chung.
*   **Lan, Hùng, Mai:** Các thành viên phát triển tính năng.

---

### Tình huống 1: Khởi tạo và thiết lập dự án

Tuấn bắt đầu dự án bằng cách tạo một "ngôi nhà chung" (repository) trên GitHub để chứa mã nguồn.

1.  **Tạo kho chứa (Repo):** Tuấn lên GitHub.com, bấm "New Repository" và đặt tên là `SuperTodo`.
2.  **Lấy dự án về máy (Clone):** Tuấn mở terminal và gõ lệnh để tải kho chứa rỗng về máy mình:
    ```bash
    git clone https://github.com/tuan_dev/SuperTodo.git
    cd SuperTodo
    ```
3.  **Tạo code nền móng:** Tuấn tạo file `index.html` đầu tiên.
    ```bash
    # Tạo file
    touch index.html

    # Lưu thay đổi vào lịch sử Git
    git add .
    git commit -m "Khoi tao du an SuperTodo"

    # Đẩy code lên GitHub
    git push origin main
    ```
4.  **Mời thành viên:** Trên GitHub, Tuấn vào *Settings > Collaborators* và thêm email của Lan, Hùng, Mai để họ có quyền gửi code lên.

---

### Tình huống 2: Xung đột (Conflict) khi cùng làm trên một nhánh

Ban đầu, để cho nhanh, Tuấn bảo: *"Mọi người cứ code rồi push thẳng lên nhánh `main` nhé!"*. Và rắc rối xảy ra ngay lập tức.

*   **Lan** chỉnh sửa dòng 10 của `index.html` để thêm Header.
*   **Hùng** cũng chỉnh sửa dòng 10 của `index.html` (khi chưa lấy code mới của Lan về) để thêm Logo.

**Diễn biến:**
1.  Lan code xong trước, Lan đẩy code lên:
    ```bash
    git add .
    git commit -m "Lan them Header"
    git push origin main
    ```
    -> **Thành công.** Server GitHub giờ đã có code của Lan.

2.  Hùng code xong sau, Hùng đẩy code lên:
    ```bash
    git push origin main
    ```
    -> **LỖI!** GitHub từ chối vì code của Hùng dựa trên phiên bản cũ, thiếu code của Lan.

3.  **Cách xử lý:** Hùng phải tải code mới nhất về trước.
    ```bash
    git pull origin main
    ```
    -> **CONFLICT!** Git báo: *"Tôi không biết lấy dòng 10 của Lan hay của Hùng, hai bạn tự sửa đi!"*.

4.  Hùng mở file `index.html`, thấy Git đánh dấu đoạn xung đột. Hùng sửa lại, giữ cả Header và Logo, xóa các ký tự lạ (`<<<<`, `====`, `>>>>`) rồi commit lại:
    ```bash
    git add .
    git commit -m "Hung them Logo va sua xung dot"
    git push origin main
    ```
    -> **Thành công.**

---

### Tình huống 3: Quy trình chuẩn - Phân nhánh (Branching)

Rút kinh nghiệm, Tuấn ra quy định mới: **"Mỗi người một nhánh, không ai được đụng trực tiếp vào `main`. Xong việc thì báo anh ghép (Merge) vào."**

#### Phân chia công việc:
*   Mai được giao làm chức năng "Đăng nhập".
*   Tuấn vẫn quản lý nhánh `main` (nhánh chính thức).

#### Bước 1: Mai làm việc trên nhánh riêng
Mai tạo một nhánh mới từ `main` đặt tên là `feature-login`:
```bash
# Tạo và chuyển sang nhánh mới
git checkout -b feature-login

# Mai code chức năng đăng nhập...
# Sau khi xong:
git add .
git commit -m "Mai hoan thien chuc nang dang nhap"

# Đẩy nhánh riêng của Mai lên GitHub (không đẩy vào main)
git push -u origin feature-login
```

#### Bước 2: Hợp nhất (Merge)
Mai lên GitHub tạo **Pull Request** (Yêu cầu hợp nhất). Tuấn xem xét thấy code ngon lành, Tuấn chấp nhận hợp nhất.
Tại máy của Tuấn (hoặc trên GitHub), lệnh tương đương sẽ là:
```bash
# Tuấn về nhánh chính
git checkout main
git pull origin main # Cập nhật mới nhất

# Gộp code của Mai vào
git merge feature-login
```

#### Bước 3: Cập nhật nhánh cũ (Rebase)
Trong khi Mai làm Đăng nhập, Lan đang làm chức năng "Thanh toán" ở nhánh `feature-payment`. Lúc này `main` đã có code Đăng nhập mới của Mai, nhưng nhánh của Lan thì chưa.
Để code của Lan không bị lạc hậu, Lan dùng **Rebase** để "bứng" code của mình để lên trên đầu code mới nhất của `main`.

```bash
# Lan đang ở nhánh feature-payment
git checkout feature-payment

# Lấy code mới nhất về máy (nhưng chưa gộp)
git fetch origin

# Tái cấu trúc: Lấy nền tảng mới nhất của main đắp vào dưới chân nhánh này
git rebase origin/main
```

Nếu có xung đột, Lan sửa tương tự Tình huống 2. Sau khi xong, lịch sử code sẽ thẳng tắp và đẹp mắt.

---

### Tổng kết
Qua câu chuyện trên, bạn đã hình dung được 3 thao tác cốt lõi:
1.  **Clone & Push**: Khởi tạo và đồng bộ.
2.  **Resolve Conflict**: Xử lý khi code "đá" nhau.
3.  **Branching & Merge/Rebase**: Chia để trị, giúp team làm việc song song hiệu quả.

Ở bài tiếp theo, chúng ta sẽ tra cứu chi tiết ý nghĩa kỹ thuật của từng câu lệnh này nhé!
