fn main() {
    println!("Hello, world!");
    let _logical: bool = true;
    let _a_float: f32 = 1.0;
    let _an_float = 1f32;

    let _default_float = 3.0;   //f64
    let _default_interge = 3;   //i32

    let mut _inferred_type = 12;
    _inferred_type = 4294967296i64;

    let mut _mutable = 12; // Mutable `i32`
    _mutable = 21;

    // 报错！变量的类型并不能改变。
    // mutable = true;


    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let _mutable = true;

}
