#!/bin/bash

# 當任何指令失敗時，立即停止腳本
set -e

echo "🚀 開始打包程序..."

# --- 1. 獲取資訊 ---
echo "🔍 正在從 Cargo.toml 獲取專案資訊..."

# 從 Cargo.toml 中解析專案名稱和版本
PROJECT_NAME=$(grep '^name =' Cargo.toml | sed 's/name = "\(.*\)"/\1/')
VERSION=$(grep '^version =' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

# 獲取作業系統和架構資訊
OS_NAME=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# 檢查是否成功獲取資訊
if [ -z "$PROJECT_NAME" ] || [ -z "$VERSION" ]; then
    echo "❌ 錯誤：無法從 Cargo.toml 中解析專案名稱或版本。" >&2
    exit 1
fi

echo "    - 專案名稱: $PROJECT_NAME"
echo "    - 版本: v$VERSION"
echo "    - 作業系統: $OS_NAME"
echo "    - 架構: $ARCH"

# --- 2. 編譯專案 ---
echo "
🛠️  正在編譯 Release 版本... (這可能需要一些時間)"
cargo build --release

# --- 3. 準備打包檔案 ---
echo "
📦 正在準備打包檔案..."

# 建立一個臨時的打包目錄
STAGE_DIR="./${PROJECT_NAME}-staging"
rm -rf "$STAGE_DIR" # 清理舊的臨時目錄
mkdir -p "$STAGE_DIR"

# 執行檔的路徑
BINARY_PATH="./target/release/$PROJECT_NAME"

# 複製必要的檔案到臨時目錄
cp "$BINARY_PATH" "$STAGE_DIR/"
cp README.md "$STAGE_DIR/"

echo "    - 已將執行檔和 README.md 複製到臨時目錄"

# --- 4. 建立壓縮檔 ---

# 定義最終的壓縮檔名稱
ARCHIVE_NAME="${PROJECT_NAME}-v${VERSION}-${OS_NAME}-${ARCH}.tar.gz"

echo "
🗜️  正在建立壓縮檔: $ARCHIVE_NAME"

# 使用 tar 建立 .tar.gz 壓縮檔
# -C 選項可以在打包前切換目錄，這樣壓縮檔內部就不會包含 STAGE_DIR 這層目錄
tar -czvf "$ARCHIVE_NAME" -C "$STAGE_DIR" .

mkdir -p release
mv "$ARCHIVE_NAME" ./release

# --- 5. 清理 ---
echo "
🧹 正在清理臨時檔案..."
rm -rf "$STAGE_DIR"

echo "
✅ **打包成功！**"
echo "   您的安裝包已建立於: $(pwd)/release/$ARCHIVE_NAME"
