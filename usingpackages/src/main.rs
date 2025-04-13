use jlrs::prelude::*;

fn main() {
    let handle = Builder::new().start_local().expect("cannot init Julia");

    // using LinearAlgebra をする前は dot 関数がロードされないので使えない。
    handle.local_scope::<_, 1>(|mut frame| {
        let dot = Module::main(&frame).global(&mut frame, "dot");
        // dot 関数は LinearAlgebra モジュールに定義されているので、LinearAlgebra を
        // 読み込む前は dot 関数は見つからない。
        // そのため、dot は Err になる。
        assert!(dot.is_err());
        assert!(!dot.is_ok());
    });

    unsafe {
        handle.using("LinearAlgebra")
    }.expect("Cannot load LinearAlgebra");

    handle.local_scope::<_, 4>(|mut frame| {
        let _dot = Module::main(&frame).global(&mut frame, "dot");
        // LinearAlgebra を読み込んだ後は dot 関数が見つかるので、dot は Ok になる。
        // そのため、dot は Ok になる。
        assert!(_dot.is_ok());
        assert!(!_dot.is_err());

        let dot = _dot.expect("cannot get dot function");

        let data1 = vec![1.0, 2.0, 3.0];
        let data2 = vec![4.0, 5.0, 6.0];
        let a = TypedArray::<f64>::from_vec(&mut frame, data1, 3)
            .expect("incompatible type and layout")
            .expect("invalid size");

        assert_eq!(a.rank(), 1);

        let b = TypedArray::<f64>::from_vec(&mut frame, data2, 3)
            .expect("incompatible type and layout")
            .expect("invalid size");

        assert_eq!(b.rank(), 1);

        assert_eq!(a.element_type(), b.element_type());

        let result: f64;
        unsafe {
            let jlresult = dot.call2(&mut frame, a.as_value(), b.as_value())
                .expect("cannot call dot function");
            result = jlresult.unbox::<f64>().expect("cannot unbox result");
        };

        println!("dot result: {}", result);
        // 32.0 = 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0
        let expected = 32.0;
        assert_eq!(result, expected);
    });
}
