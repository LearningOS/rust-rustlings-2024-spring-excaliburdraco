// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut v: Vec<String> = self;
        v.push(String::from("Bar"));
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar")); //执行pop(),删除最后一个元素，并返回删除的元素
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
        // assert_eq!(foo.pop().unwrap(), None);  //切片为空，panic，并不会返回None
    }
}
