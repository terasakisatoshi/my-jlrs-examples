use jlrs::prelude::*;

fn main() {
    let handle = Builder::new().start_local().expect("cannot init Julia");

    handle.local_scope::<_, 1>(|mut frame| {
        // Safety: we only evaluate a print statement, which is perfectly safe.
        unsafe {
            Value::eval_string(&mut frame, "println(\"Hello, world!\")")
        }.expect("an exception occurred");
    });
}