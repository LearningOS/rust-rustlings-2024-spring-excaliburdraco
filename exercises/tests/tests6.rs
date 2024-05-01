// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.



struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {  //raw_pointer_to_box接收一个指向Foo的指针，并返回一个指向Foo的Box
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.

    // let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr)};
    //定义一个变量，指向Foo的b字段，unsafe { Box::from_raw(ptr)}表示将ptr指向的内存转换为Box<Foo>类型

    // let  ret: Box<Foo> = Box::from_raw(ptr);
    // //定义一个变量，指向Foo的字段，Box::from_raw(ptr)表示将ptr指向的内存转换为Box<Foo>类型
    // ret

    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    //定义一个变量，指向Foo的字段，unsafe { Box::from_raw(ptr) }表示将ptr指向的内存转换为Box<Foo>类型,此时
    //ret是一个指向Foo的指针，它指向一个包含a和b字段的结构体，ret.a的值是1，ret.b的值是None，
    //ret.a的值为什么是1，因为 

    ret.b = Some("hello".to_string());
    ret
    /*定义一个变量，指向Foo的字段，unsafe { Box::from_raw(ptr) }表示将ptr指向的内存转换为Box<Foo>类型
    ret.b是一个Option类型的字段，它包含一个String类型的值。
    ret.b = Some("hello".to_string())表示将ret.b的值设置为Some("hello".to_string())。
     */
    
   
    // todo!("The rest of the code goes here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };
        //此时，ret是一个指向Foo的指针，它指向一个包含a和b字段的结构体,

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
