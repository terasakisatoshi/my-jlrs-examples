use jlrs::prelude::*;

fn main() {
    let handle = Builder::new().start_local().expect("cannot init Julia");

    handle.local_scope::<_, 3>(|mut frame| {
        let s = JuliaString::new(&mut frame, "Hello, World!").as_value();
        let println_fn = Module::base(&frame)
            .global(&mut frame, "println")
            .expect("println not found in Base");
        unsafe {
            println_fn.call1(&mut frame, s).expect("println threw an exception")
        };
    });
}
