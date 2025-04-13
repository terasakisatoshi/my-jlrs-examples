use jlrs::prelude::*;

fn main() {
    let handle = Builder::new().start_local().expect("cannot init Julia");

    handle.local_scope::<_, 6>(|mut frame| {
        let one = Value::new(&mut frame, 1usize);
        let two = Value::new(&mut frame, 2usize);
        let plusmsg = JuliaString::new(&mut frame, "+").as_value();
        let equalmsg = JuliaString::new(&mut frame, "=").as_value();
        let println_fn = Module::base(&frame)
            .global(&mut frame, "println")
            .expect("println not found in Base");
        let plus_fn = Module::base(&frame)
            .global(&mut frame, "+")
            .expect("println not found in Base");
        // Safety: calling println with an integer is safe
        unsafe {
            let result = plus_fn.call(&mut frame, [one, two]).expect("addition threw an exception");
            println_fn.call(&mut frame, [one, plusmsg, two, equalmsg, result]).expect("println threw an exception")
        };
    });
}
