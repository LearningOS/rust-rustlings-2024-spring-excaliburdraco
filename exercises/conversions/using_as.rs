// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.



fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64
    //total是f64类型，values.len()default是usize类型，所以需要转换为f64类型
  //usize类型不能直接转换为f64类型，所以需要使用as关键字进行类型转换,usize是无符号整数，所以需要使用as关键字进行类型转换
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
