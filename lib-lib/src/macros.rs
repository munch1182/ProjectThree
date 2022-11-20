#[macro_export]
macro_rules! err {
    // 之间将字符转为std::io::Error
    ($e: expr) => {
        std::io::Error::new(std::io::ErrorKind::Other, $e)
    };

    // 字符带参数
    ($($e:tt)*) => {
        std::io::Error::new(std::io::ErrorKind::Other, format!($($e)*))
    };
}

/**
 * 将一个类型的错误的Result转为另一个类型的Result
 * 第二个参数可传入自定义原因, 用以替代原来的原因
 */
#[macro_export]
macro_rules! err_to {
    ($e: expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => Err(err!(format!("{}", err))), // 没有传递err类型
        }
    };
    // 第二个参数可传入自定义原因, 用以替代原来的原因, 原因的错误信息会被忽略
    ($e: expr,$($s:tt)*) => {
        match $e {
            Ok(val) => Ok(val),
            Err(_) => Err(err!($($s)*)),
        }
    };
}

/**
 * 将Option转为Result, 可传参数描述原因, 默认值为no value
 */
#[macro_export]
macro_rules! option2result {
    ($e:expr) => {
        match $e {
            Some(v) => Ok(v),
            None => Err(err!("no value")),
        }
    };
    // 第二个参数可传入自定义原因
    ($e:expr, $($s:tt)*) => {
        match $e {
            Some(v) => Ok(v),
            None => Err(err!($($s)*)),
        }
    };
}

/**
 * 将Result转为Option
 */
#[macro_export]
macro_rules! result2option {
    ($e:expr) => {
        match $e {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    };
}

#[cfg(test)]
mod tests {
    type R = Result<(), std::io::Error>;
    // cargo.exe test -- macros::tests::test_macro --exact --nocapture
    #[test]
    fn test_macro() {
        let result: R = Ok(());
        let opt = result2option!(result);

        println!("1. {:?}", opt);

        let result = option2result!(opt);
        println!("2. {:?}", result);

        let result: R = Err(err!("123"));
        println!("3. {:?}", result);

        let opt = result2option!(result);
        println!("4. {:?}", opt);

        let result = Err(err!("{}", 223));
        let opt = result2option!(result);
        println!("5. {:?}", opt);

        let result: R = option2result!(opt, "error info");
        println!("6. {:?}", result);

        let result: R = option2result!(opt, "error info:{}", 323);
        println!("7. {:?}", result);
    }

    // cargo.exe test -- macros::tests::teset_err_to --exact --nocapture
    #[test]
    fn teset_err_to() {
        let err1 = "a".parse::<u8>();
        println!("err: {:?}", err1);

        let err = err_to!(&err1);
        println!("err_to: {:?}", err);

        let err = err_to!(err1, "err info");
        println!("err_to: {:?}", err);
    }
}
