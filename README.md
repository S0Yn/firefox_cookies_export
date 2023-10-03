# Firefox Cookies Export

## 描述

`firefox_cookies_export` 是一個基於 Rust 的命令行工具，專為從 Firefox 瀏覽器導出和導入 cookies 設計。導出的 cookies 會保存在一個加密文件中，並且可以隨時恢復到瀏覽器。這對於在不同系統或配置文件之間備份或轉移 cookies 尤為有用。

## 使用方法

### 導出 Cookies
1. 運行程序並選擇選項 1 以保存當前的 cookies。
2. 在提示時輸入加密密碼。

### 導入 Cookies
1. 選擇選項 2 以恢復 cookies。
2. 在提示時輸入加密密碼。

### 退出
- 選擇選項 3 以退出程序。

## 命令

運行程序：

```bash
cargo run
```

## 依賴

- **Rust**: 確保您已安裝最新版本的 Rust。
- **dirs crate**: 用於獲取家目錄路徑。

## 安裝

克隆存儲庫：

```bash
git clone https://github.com/your-username/firefox_cookies_export
```

導航到項目目錄：

```bash
cd firefox_cookies_export
```

構建項目：

```bash
cargo build --release
```

## 未來功能

- **完整的書籤和歷史備份**: 未來版本將包括導出和導入書籤和瀏覽器歷史的功能。
- **配置文件選擇**: 用戶將能夠選擇他們希望從哪個 Firefox 配置文件導出或導入 cookies。
- **增強的加密方法**: 將應用改進的加密技術以確保導出數據的安全性。

## 貢獻

如果您想對這個工具的發展做出貢獻，請隨意 fork 存儲庫，進行更改，並打開一個 pull 請求。任何貢獻都非常受歡迎！

## 許可證

該項目根據 MIT 許可證授權。請參閱 LICENSE 文件以獲取詳細信息。
