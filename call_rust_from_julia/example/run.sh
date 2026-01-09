#!/bin/bash

# JuliaからRustを呼び出す例 - ビルド＆実行スクリプト
#
# このスクリプトは以下を実行します：
# 1. Juliaのインストールパスを自動検出
# 2. Rustライブラリをビルド
# 3. Juliaスクリプトを実行

set -e  # エラーが発生したら終了

# カラー出力用
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== JuliaからRustを呼び出す例 ===${NC}"
echo ""

# 1. Juliaのインストールパスを検出
echo -e "${YELLOW}[1/3] Juliaのインストールパスを検出中...${NC}"

if ! command -v julia &> /dev/null; then
    echo -e "${RED}エラー: Juliaが見つかりません。Juliaをインストールしてください。${NC}"
    exit 1
fi

# JuliaのBINDIRを取得
JULIA_BINDIR=$(julia -e "println(Sys.BINDIR)" 2>/dev/null)

if [ -z "$JULIA_BINDIR" ]; then
    echo -e "${RED}エラー: JuliaのBINDIRを取得できませんでした。${NC}"
    exit 1
fi

# Juliaのルートディレクトリを取得（binの親ディレクトリ）
JULIA_DIR=$(dirname "$JULIA_BINDIR")

# julia.hの存在を確認
if [ ! -f "$JULIA_DIR/include/julia/julia.h" ]; then
    echo -e "${RED}エラー: Juliaのヘッダーファイルが見つかりません: $JULIA_DIR/include/julia/julia.h${NC}"
    echo "JLRS_JULIA_DIR環境変数を手動で設定してください。"
    exit 1
fi

echo "  Juliaディレクトリ: $JULIA_DIR"
echo "  ヘッダーファイル: $JULIA_DIR/include/julia/julia.h"
echo ""

# 2. Rustライブラリをビルド
echo -e "${YELLOW}[2/3] Rustライブラリをビルド中...${NC}"

# スクリプトのディレクトリに移動
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# JLRS_JULIA_DIRを設定してビルド
export JLRS_JULIA_DIR="$JULIA_DIR"

if cargo build --release; then
    echo -e "${GREEN}ビルド成功！${NC}"
    echo ""
else
    echo -e "${RED}ビルドに失敗しました。${NC}"
    exit 1
fi

# 3. Juliaスクリプトを実行
echo -e "${YELLOW}[3/3] Juliaスクリプトを実行中...${NC}"
echo ""

# ライブラリのパスを確認
if [ "$(uname)" == "Darwin" ]; then
    # macOS
    LIB_NAME="librust_from_julia_example.dylib"
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    # Linux
    LIB_NAME="librust_from_julia_example.so"
else
    # Windows (Git Bashなど)
    LIB_NAME="rust_from_julia_example.dll"
fi

LIB_PATH="$SCRIPT_DIR/target/release/$LIB_NAME"

if [ ! -f "$LIB_PATH" ]; then
    echo -e "${RED}エラー: ライブラリが見つかりません: $LIB_PATH${NC}"
    exit 1
fi

# Juliaスクリプトを実行
julia example.jl

echo ""
echo -e "${GREEN}=== 完了 ===${NC}"
