// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
/*定义宏，并使用它，
 * 然后编译并运行程序，以检查宏是否按预期工作。
 * 确保在运行程序之前删除了生成的文件。
 * 如果需要，可以使用`cargo expand`命令检查宏展开后的结果。
 * 
 */

fn main() {
    my_macro!();
}
