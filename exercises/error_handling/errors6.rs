// errors6.rs
//
// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here, we
// define a custom error type to make it possible for callers to decide what to
// do next when our function returns an error.
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.



use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {  /*定义一个枚举类型,他中有两个成员，Creation和ParseInt，Creation的作用是将CreationError类型的值转换为ParsePosNonzeroError类型的值，
    ParseInt的作用是将ParseIntError类型的值转换为ParsePosNonzeroError类型的值 */
    Creation(CreationError),//这个是原始数据转换的错误类型，如果是0，或者负数，返回错误。
    ParseInt(ParseIntError),//这个是数据转换的错误类型，如果是转换失败，返回错误。
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)   //返回的是一个ParsePosNonzeroError类型的值,方法的作用是将CreationError类型的值转换为ParsePosNonzeroError类型的值
    }
    // TODO: add another error conversion function here.
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err) //返回的是一个ParsePosNonzeroError类型的值，方法是将ParseIntError类型的值转换为ParsePosNonzeroError类型的值。
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> { //返回的是一个Result类型的值
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    // let x: i64 = s.parse().unwrap();  
    /* unwrap()方法如果接收一个Result类型，会返回一个Result类型的值，如果Result类型是Ok，
    则返回Ok中的值，如果Result类型是Err，则返回Err中的值 。
    具体代码：let x: i64 = s.parse().unwrap()，也可以写成：let x = s.parse::<i64>.unwarp（）。意思是强制将s:&str类型转换为i64,如果转换成功，
    用unwarp解析ok()的值，如果转换失败，解析err()的值*/

    let x: i64 = match s.parse(){  //采用match模式匹配，如果成功，返回Ok中的值，如果失败，返回Err中的值。
        Ok(x) => x,
        Err(e) => return Err(ParsePosNonzeroError::from_parseint(e)),
    };

    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)  //map_err可以接收一个函数，将一个值转换为另一个值，
}
//传值路线: s -> x -> PositiveNonzeroInteger::new(x) -> Result<PositiveNonzeroInteger, CreationError> -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> ->

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
