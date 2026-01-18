---
title: "[PYTHON CẤP TỐC #11] Xử lý ngoại lệ trong Python"
date: 2023-06-26
slug: "python-cap-toc-11-xu-ly-ngoai-le-trong-python"
categories:
  - "Develop"
tags:
  - "expvn"
  - "hoc-python"
  - "python-co-ban"
  - "xu-ly-ngoai-le"
summary: null
cover: "images/Python-la-gi.png"
featured: false
draft: false
author: "admin"
---
Xử lý ngoại lệ trong Python OOP là quá trình xử lý các tình huống bất thường xảy ra trong chương trình. Điều này giúp bạn kiểm soát và xử lý các lỗi và tránh việc chương trình bị dừng đột ngột. Trong bài này, chúng ta cùng tìm hiểu về xử lý ngoại lệ nhé.
Trong Python, xử lý ngoại lệ được thực hiện bằng cách sử dụng câu lệnh `try-except`. Câu lệnh `try` dùng để bao bọc các đoạn mã có thể gây ra ngoại lệ. Câu lệnh `except` được sử dụng để xử lý ngoại lệ và thực hiện các hành động tương ứng. Dưới đây là cú pháp của câu lệnh `try-except`:
```py
try:
    # Đoạn mã có thể gây ra ngoại lệ
    # ...
except ExceptionType:
    # Xử lý ngoại lệ và thực hiện các hành động tương ứng
    # ...
```
## **1\. Xử lý ngoại lệ chung**
Trường hợp này dùng để xử lý các ngoại lệ không cụ thể. Một lỗi chung có thể là `Exception`. Ví dụ:
```py
try:
    result = 10 / 0  # Chia một số cho 0
except Exception as e:
    print("Có lỗi xảy ra:", e)
```
Kết quả:
```py
Có lỗi xảy ra: division by zero
```
## **2\. Xử lý ngoại lệ cụ thể**
Trong trường hợp này, bạn xác định loại ngoại lệ cụ thể mà bạn muốn xử lý. Ví dụ:
```py
try:
    file = open("file.txt", "r")
    content = file.read()
    file.close()
except FileNotFoundError:
    print("File không tồn tại")
except PermissionError:
    print("Không có quyền truy cập vào file")
except Exception as e:
    print("Có lỗi xảy ra:", e)
```
Kết quả:
```py
File không tồn tại
```
## **3\. Xử lý ngoại lệ và finally**
Bạn có thể sử dụng khối `finally` để thực hiện các hành động bất kể có ngoại lệ xảy ra hay không. Ví dụ:
```py
try:
    x = 10 / 0
except ZeroDivisionError:
    print("Không thể chia cho 0")
finally:
    print("Khối lệnh finally được thực thi")
```
Kết quả:
```py
Không thể chia cho 0
Khối lệnh finally được thực thi
```
## **4\. Ném ngoại lệ**
Bạn cũng có thể ném ngoại lệ bằng câu lệnh `raise`. Ví dụ:
```py
def validate_age(age):
    if age < 0:
        raise ValueError("Tuổi không hợp lệ")
    elif age > 120:
        raise ValueError("Tuổi không hợp lệ")
    else:
        print("Tuổi hợp lệ")
try:
    validate_age(150)
except ValueError as e:
    print("Lỗi:", e)
```
Kết quả:
```py
Lỗi: Tuổi không hợp lệ
```
Vậy là chúng ta đã tìm hiểu một số ví dụ về xử lý ngoại lệ trong Python OOP. Bạn có thể sử dụng các cấu trúc và ví dụ trên như một cơ sở để nắm vững cách xử lý ngoại lệ trong Python và áp dụng vào các tình huống phù hợp trong ứng dụng của bạn.
