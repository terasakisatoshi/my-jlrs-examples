//! JuliaからRustを呼び出す例
//!
//! このライブラリは、jlrsの`julia_module`マクロを使用して
//! Rustの関数と型をJuliaから呼び出せるようにします。

use jlrs::{
    data::{
        managed::{
            string::StringRet,
            value::typed::{TypedValue, TypedValueRet},
        },
    },
    error::JlrsError,
    prelude::*,
    weak_handle_unchecked,
};

/// 2つの整数を足し算する関数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 配列の要素の合計を計算する関数
fn sum_array(arr: TypedArray<f64>) -> f64 {
    unsafe {
        let data = arr.bits_data();
        data.as_slice().iter().sum()
    }
}

/// 配列の各要素を2倍にする関数（in-place）
fn double_array(mut arr: TypedArray<f64>) {
    unsafe {
        let mut data = arr.bits_data_mut();
        for x in data.as_mut_slice() {
            *x *= 2.0;
        }
    }
}

/// 文字列を大文字に変換する関数
fn to_uppercase(s: JuliaString) -> StringRet {
    let rust_str = s.as_str().unwrap_or("");
    let handle = unsafe { weak_handle_unchecked!() };
    JuliaString::new(handle, rust_str.to_uppercase()).leak()
}

/// カウンター型 - 内部状態を持つ型の例
#[derive(Clone, OpaqueType)]
pub struct Counter {
    value: i32,
}

impl Counter {
    /// 新しいカウンターを作成
    fn new(initial: i32) -> TypedValueRet<Counter> {
        let weak_handle = unsafe { weak_handle_unchecked!() };
        TypedValue::new(weak_handle, Counter { value: initial }).leak()
    }

    /// カウンターをインクリメント
    fn increment(&mut self) {
        self.value += 1;
    }

    /// カウンターをデクリメント
    fn decrement(&mut self) {
        self.value -= 1;
    }

    /// 現在の値を取得
    fn get(&self) -> i32 {
        self.value
    }

    /// 値を設定
    fn set(&mut self, value: i32) {
        self.value = value;
    }
}

/// 計算機型 - より複雑な例
#[derive(Clone, OpaqueType)]
pub struct Calculator {
    result: f64,
}

impl Calculator {
    /// 新しい計算機を作成
    fn new() -> TypedValueRet<Calculator> {
        let weak_handle = unsafe { weak_handle_unchecked!() };
        TypedValue::new(weak_handle, Calculator { result: 0.0 }).leak()
    }

    /// 値を加算
    fn add(&mut self, value: f64) {
        self.result += value;
    }

    /// 値を減算
    fn subtract(&mut self, value: f64) {
        self.result -= value;
    }

    /// 値を乗算
    fn multiply(&mut self, value: f64) {
        self.result *= value;
    }

    /// 値を除算
    fn divide(&mut self, value: f64) -> JlrsResult<()> {
        if value == 0.0 {
            return Err(Box::new(JlrsError::exception("Division by zero")));
        }
        self.result /= value;
        Ok(())
    }

    /// 結果を取得
    fn get_result(&self) -> f64 {
        self.result
    }

    /// リセット
    fn reset(&mut self) {
        self.result = 0.0;
    }
}

// Juliaモジュールとしてエクスポート
julia_module! {
    become rust_from_julia_example_init_fn;

    // 関数をエクスポート
    fn add(a: i32, b: i32) -> i32;
    fn sum_array(arr: TypedArray<f64>) -> f64;
    fn double_array(arr: TypedArray<f64>);
    fn to_uppercase(s: JuliaString) -> StringRet;

    // Counter型とメソッドをエクスポート
    struct Counter;
    in Counter fn new(initial: i32) -> TypedValueRet<Counter> as Counter;
    in Counter fn increment(&mut self) as increment!;
    in Counter fn decrement(&mut self) as decrement!;
    in Counter fn get(&self) -> i32;
    in Counter fn set(&mut self, value: i32) as set!;

    // Calculator型とメソッドをエクスポート
    struct Calculator;
    in Calculator fn new() -> TypedValueRet<Calculator> as Calculator;
    in Calculator fn add(&mut self, value: f64) as add!;
    in Calculator fn subtract(&mut self, value: f64) as subtract!;
    in Calculator fn multiply(&mut self, value: f64) as multiply!;
    in Calculator fn divide(&mut self, value: f64) -> JlrsResult<()> as divide!;
    in Calculator fn get_result(&self) -> f64;
    in Calculator fn reset(&mut self) as reset!;
}
