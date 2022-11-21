#[macro_export]
macro_rules! log {
    // error!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // error!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => (log::log!(target: $target, $crate::Level::Error, $($arg)+));

    // error!("a {} event", "log")
    ($($arg:tt)+) => (log::log!($crate::Level::Error, $($arg)+))
}
