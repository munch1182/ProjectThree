//!
//! 基础宏相关
//!
///
/// 创建一个std::io::Error
///
/// Example
///
/// ```rust
/// use lib::err;
///
/// let opt = Some(());
/// assert!(opt.ok_or(err!()).is_ok()); // error信息将仅为"error"
///
/// err!("err txt");
///
/// err!("err: {}", 1);
/// ```
///
#[macro_export]
macro_rules! err {
    () => {
        std::io::Error::new(std::io::ErrorKind::Other, "error")
    };
    // 之间将字符转为std::io::Error
    ($e: expr) => {
        std::io::Error::new(std::io::ErrorKind::Other, $e)
    };

    // 字符带参数
    ($($e:tt)*) => {
        std::io::Error::new(std::io::ErrorKind::Other, format!($($e)*))
    };
}

///
/// 将其它类型的Result转为Err类型为std::io::Error类型的Result
///
/// ```rust
/// use lib::{err_to, err};
///  
/// let result1: Result<i32,std::num::ParseIntError> = i32::from_str_radix("a12", 10);
/// let result1: Result<i32,std::io::Error> = err_to!(result1);
///
/// let result2: Result<i32,std::num::ParseIntError> = i32::from_str_radix("a12", 10);
/// let result2: Result<i32,std::io::Error> = err_to!(result2, "err text");
///
/// let result3: Result<i32,std::num::ParseIntError> = i32::from_str_radix("a12", 10);
/// let result3: Result<i32,std::io::Error> = err_to!(result3, "err text: {}", "a12");
///
/// ```
///
///
#[macro_export]
macro_rules! err_to {
    ($e: expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => Err(err!(format!("{}", err))), // 没有传递err类型
        }
    };
    // 第二个参数可传入自定义原因, 用以替代原来的原因, 原因的错误信息会被忽略
    ($e: expr, $($s:tt)*) => {
        match $e {
            Ok(val) => Ok(val),
            Err(_) => Err(err!($($s)*)),
        }
    };
}
