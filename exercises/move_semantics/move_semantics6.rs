// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut data = "Rust is great!".to_string();
    // let a ="Rust is great!";
    // let b = &data;
    /*在 Rust 中，String 和 str 是两种不同的字符串类型，它们在所有权和存储方式上有所区别。这里是它们的主要区别：
    String 是一个拥有所有权的字符串类型。它是可变的，可以增长和改变内容。String 类型的数据存储在堆上，因此它的大小在编译时是未知的，可以动态改变。
    str 是一个不可变的字符串切片类型，通常以引用的形式 &str 出现。它表示对某个字符串数据的视图，这些数据可以存储在任何地方，例如在堆上的 String 中，
    或者作为程序的一部分硬编码在可执行文件中。
    当你看到 &String 时，它实际上是对 String 类型的引用，而 &str 是对某个字符串数据的切片引用。通常，当你需要一个字符串的只读视图时，你会使用 &str。
    当你需要拥有字符串数据，可能需要修改它，或者需要将字符串数据传递给其他线程时，你会使用 String。
    这里是一个简单的例子来说明它们的使用：
    Rust
    // String 类型，可以修改
    let mut s = String::from("hello");
    s.push_str(", world!");
    // &str 类型，不可变的字符串切片
    let s_slice: &str = &s[0..5];
    AI-generated code. Review and use carefully. More info on FAQ.
    在这个例子中，s 是一个 String，我们可以修改它。s_slice 是一个 &str，它是 s 的一个切片，我们不能通过 s_slice 来修改 s 的内容。
    希望这有助于你理解 &String 和 &str 之间的区别。如果你有其他问题或需要进一步的帮助，请随时告诉我！ */

    get_char(&data);

    string_uppercase(&mut data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase(); //to_uppercase返回的是一个有所有权的string

    println!("{}", data);
}
