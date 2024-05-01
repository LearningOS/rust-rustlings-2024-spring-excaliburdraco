// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
       if let  Some(word) = optional_target{
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }
        /*这段代码的逻辑是：optional_integers是一个Vec<Option<i8>>类型，进入while循环，执行第一次循环，执行optional_integers.pop()方法返回的是切片的最后的一个Option<Option<i8>>，即Smoe(Some(10)),
        要将Some(Some(5))解析出来，需要定义个亿Some(Some(integer))类型，进行两次解析，才能得到整型类型。然后执行cursor -= 1;*/

        assert_eq!(cursor, 0);
    }
}
