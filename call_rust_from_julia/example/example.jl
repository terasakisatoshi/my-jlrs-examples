# JuliaからRustを呼び出す例
#
# このファイルは、Rustで書かれたライブラリをJuliaから使用する方法を示します。

# JlrsCoreパッケージをインストール（まだインストールされていない場合）
try
    using JlrsCore
catch e
    import Pkg
    Pkg.add("JlrsCore")
    using JlrsCore
end

# RustライブラリをラップするJuliaモジュール
module RustExample
using JlrsCore.Wrap

# ライブラリのパスを指定
# ビルド後、以下のパスにライブラリが生成されます:
# - Linux: target/release/librust_from_julia_example.so
# - macOS: target/release/librust_from_julia_example.dylib
# - Windows: target/release/rust_from_julia_example.dll

# 現在のディレクトリから相対パスで指定
# run.shから実行する場合は、exampleディレクトリがカレントディレクトリになります
if Sys.iswindows()
    lib_path = "target/release/rust_from_julia_example.dll"
elseif Sys.isapple()
    lib_path = "target/release/librust_from_julia_example.dylib"
else
    lib_path = "target/release/librust_from_julia_example.so"
end

# モジュールをラップ
@wrapmodule(lib_path, :rust_from_julia_example_init_fn)

function __init__()
    @initjlrs
end
end

# 使用例
using .RustExample

println("=== JuliaからRustを呼び出す例 ===\n")

# 1. シンプルな関数の呼び出し
println("1. シンプルな関数の呼び出し")
result = RustExample.add(Int32(10), Int32(20))
println("   add(10, 20) = ", result)
println()

# 2. 配列の操作
println("2. 配列の操作")
arr = [1.0, 2.0, 3.0, 4.0, 5.0]
println("   元の配列: ", arr)

# 配列の合計
total = RustExample.sum_array(arr)
println("   合計: ", total)

# 配列を2倍にする（in-place）
RustExample.double_array(arr)
println("   2倍にした後: ", arr)
println()

# 3. 文字列の操作
println("3. 文字列の操作")
text = "hello, world!"
uppercase_text = RustExample.to_uppercase(text)
println("   元の文字列: ", text)
println("   大文字に変換: ", uppercase_text)
println()

# 4. Counter型の使用
println("4. Counter型の使用")
counter = RustExample.Counter(Int32(0))
println("   初期値: ", RustExample.get(counter))

RustExample.increment!(counter)
RustExample.increment!(counter)
RustExample.increment!(counter)
println("   3回インクリメント後: ", RustExample.get(counter))

RustExample.decrement!(counter)
println("   1回デクリメント後: ", RustExample.get(counter))

RustExample.set!(counter, Int32(100))
println("   100に設定後: ", RustExample.get(counter))
println()

# 5. Calculator型の使用
println("5. Calculator型の使用")
calc = RustExample.Calculator()
println("   初期値: ", RustExample.get_result(calc))

RustExample.add!(calc, 10.0)
println("   +10.0 後: ", RustExample.get_result(calc))

RustExample.multiply!(calc, 2.0)
println("   ×2.0 後: ", RustExample.get_result(calc))

RustExample.subtract!(calc, 5.0)
println("   -5.0 後: ", RustExample.get_result(calc))

result = RustExample.divide!(calc, 3.0)
if result === nothing
    println("   ÷3.0 後: ", RustExample.get_result(calc))
else
    println("   エラー: ", result)
end

RustExample.reset!(calc)
println("   リセット後: ", RustExample.get_result(calc))
println()

println("=== すべての例が完了しました ===")
