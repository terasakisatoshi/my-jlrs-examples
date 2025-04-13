use std::ffi::c_void;

use jlrs::prelude::*;

unsafe extern "C" fn add(a: f64, b: f64) -> f64 {
    a + b
}

static JULIA_CODE: &str = "function call_rust(ptr::Ptr{Cvoid}, a::Float64, b::Float64)
    ccall(ptr, Float64, (Float64, Float64), a, b)
end";

fn main() {
    let handle = Builder::new().start_local().expect("cannot init Julia");

    handle.local_scope::<_, 5>(|mut frame| {
        let ptr = Value::new(&mut frame, add as *mut c_void);

        let a = Value::new(&mut frame, 1.0f64);
        let b = Value::new(&mut frame, 2.0f64);

        // Safety: we're just defining a function.
        let func = unsafe { Value::eval_string(&mut frame, JULIA_CODE) }
            .expect("an exception occurred");

        // Safety: Immutable types are passed and returned by value, so `add`
        // has the correct signature for the `ccall` in `call_rust`. All
        // `add` does is add `a` and `b`, which is perfectly safe.
        let res = unsafe { func.call3(&mut frame, ptr, a, b) }
            .expect("an exception occurred")
            .unbox::<f64>()
            .expect("not an f64");

        assert_eq!(res, 3.0f64);
    });
}