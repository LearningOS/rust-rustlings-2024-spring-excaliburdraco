// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

/* 这个错误是因为你试图在 main 函数中使用 my_macro 宏，
但是这个宏在 macros 模块中定义，而 main 函数并不能直接访问 macros 模块中的内容1。
在 Rust 中，模块默认是私有的，也就是说，它们只能在定义它们的模块中被访问21。
你可以通过在 mod macros 之前添加 #[macro_use] 属性来解决这个问题32。
#[macro_use] 属性会让当前模块中定义的所有宏都在当前模块的外部可见*/
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
