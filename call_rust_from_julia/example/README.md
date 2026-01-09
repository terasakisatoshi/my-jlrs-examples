# JuliaからRustを呼び出す例

このディレクトリには、jlrsを使用してJuliaからRustの関数と型を呼び出す完全な例が含まれています。

## ファイル構成

- `Cargo.toml` - Rustプロジェクトの設定
- `src/lib.rs` - Rustの実装コード
- `example.jl` - Julia側の使用例
- `README.md` - このファイル

## 前提条件

- **Julia 1.10以上**がインストールされていること
- Juliaが`PATH`に含まれていること（`which julia`で確認可能）
- Juliaのヘッダーファイル（`julia.h`）が利用可能であること

Juliaがインストールされていない場合、[公式サイト](https://julialang.org/downloads/)からダウンロードしてください。

## ビルド方法

### 1. Rustライブラリをビルド

```bash
cd example
cargo build --release
```

**注意**: ビルド時にはJuliaのヘッダーファイルが必要です。Juliaが見つからない場合は、`JLRS_JULIA_DIR`環境変数を設定してください：

```bash
# macOS/Linux の例
export JLRS_JULIA_DIR=/path/to/julia-x.y.z
cargo build --release

# juliaupを使用している場合の例（macOS）
export JLRS_JULIA_DIR="$HOME/.julia/juliaup/julia-1.12.4+0.aarch64.apple.darwin14"
cargo build --release
```

Juliaのインストールパスを確認するには：

```bash
# JuliaのBINDIRを確認
julia -e "println(Sys.BINDIR)"

# 上記の出力から、親ディレクトリがJuliaのルートディレクトリです
# 例: /Users/username/.julia/juliaup/julia-1.12.4+0.aarch64.apple.darwin14/bin
#     → /Users/username/.julia/juliaup/julia-1.12.4+0.aarch64.apple.darwin14
```

ビルド後、以下のパスにライブラリが生成されます：
- Linux: `target/release/librust_from_julia_example.so`
- macOS: `target/release/librust_from_julia_example.dylib`
- Windows: `target/release/rust_from_julia_example.dll`

### 2. Juliaパッケージのインストール

`example.jl`を実行すると、必要に応じて`JlrsCore`パッケージが自動的にインストールされます。

手動でインストールする場合：

```julia
import Pkg
Pkg.add("JlrsCore")
```

### 3. 実行

```bash
julia example.jl
```

または、Julia REPLで：

```julia
include("example.jl")
```

## 例の内容

この例では以下の機能を実装しています：

1. **シンプルな関数**: `add` - 2つの整数を足し算
2. **配列操作**:
   - `sum_array` - 配列の合計を計算
   - `double_array` - 配列の各要素を2倍にする（in-place）
3. **文字列操作**: `to_uppercase` - 文字列を大文字に変換
4. **カスタム型**:
   - `Counter` - カウンター型（インクリメント/デクリメント機能）
   - `Calculator` - 計算機型（四則演算機能）

## 重要なポイント

### Rust側の設定

1. `Cargo.toml`に以下が必要：
   ```toml
   [lib]
   crate-type = ["cdylib"]

   [profile.release]
   panic = "abort"
   ```

2. ランタイム機能は有効にしない：
   - `local-rt`, `async-rt`, `multi-rt`は使用しない
   - `ccall`機能のみを使用

3. `julia_module`マクロで関数と型をエクスポート

### Julia側の設定

1. `JlrsCore`パッケージが必要
2. `@wrapmodule`マクロでライブラリをラップ
3. `@initjlrs`で初期化

## トラブルシューティング

### ライブラリが見つからない

`example.jl`の`lib_path`を、実際のライブラリのパスに合わせて修正してください。

### ビルドエラー

#### `Cannot find julia_version.h`

Juliaがインストールされていないか、パスが正しく設定されていません。

解決方法：
1. Juliaがインストールされているか確認：`which julia`
2. `JLRS_JULIA_DIR`環境変数を設定：
   ```bash
   # macOS/Linux
   export JLRS_JULIA_DIR=/path/to/julia-x.y.z

   # Windows (PowerShell)
   $env:JLRS_JULIA_DIR="C:\path\to\Julia-x.y.z"
   ```
3. Juliaのライブラリパスを設定（必要に応じて）：
   ```bash
   # macOS
   export DYLD_LIBRARY_PATH=$JLRS_JULIA_DIR/lib:$DYLD_LIBRARY_PATH

   # Linux
   export LD_LIBRARY_PATH=$JLRS_JULIA_DIR/lib:$LD_LIBRARY_PATH
   ```

- `jlrs`の`ccall`機能が有効になっていることを確認

### 実行時エラー

- ライブラリのパスが正しいか確認
- Juliaのバージョンが1.10以上であることを確認

## 参考資料

- [jlrs公式ドキュメント](https://docs.rs/jlrs)
- [jlrsチュートリアル](https://taaitaaiger.github.io/jlrs-tutorial/)
