// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        // *element = *element*2
        *element *= 2
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}
/*在 Rust 中，iter() 和 iter_mut() 有不同的行为。当你使用 iter() 时，它会产生一个不可变的元素引用的迭代器，
这意味着你不能通过这些引用来修改原始集合中的元素。因此，在 map 方法中，你直接得到了每个元素的引用，而不需要解引用，因为你不需要修改它们。
另一方面，iter_mut() 产生一个可变的元素引用的迭代器，允许你修改原始集合中的元素。在这种情况下，你需要使用解引用运算符 * 来访问引用指向的值，并进行修改。
在 Rust 中，当你使用 iter().map() 时，你确实不需要在闭包参数中使用 & 符号。闭包会自动处理引用和解引用。所以，正确的代码应该是这样的：
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| element * 2).collect()
}
AI-generated code. Review and use carefully. More info on FAQ.

而这是使用 iter_mut() 的情况，需要解引用来修改值：
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        *element *= 2;
    }
    v
} */

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter()
        .map(|element| {
            // TODO: Do the same thing as above - but instead of mutating the
            // Vec, you can just return the new number!
            element * 2
        })
        .collect()
}
/*在Rust中，iter() 方法会创建一个迭代器，它会借用集合中的每个元素。这意味着迭代器中的元素是对原始数据的引用。当你使用 map 方法时，
它会遍历迭代器中的每个元素，这些元素已经是引用了。因此，在 map 闭包中，element 是 &i32 类型，即一个指向整数的引用。
在你的代码示例中，element*2 实际上是使用了自动解引用（auto dereferencing）。Rust 有一个叫做 解引用强制多态（Deref coercion）的特性，
它允许自动将引用转换为它们所指向的值。所以，当你写 element*2 时，Rust 会自动解引用 element，从 &i32 变成 i32，然后再进行乘法运算。 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
