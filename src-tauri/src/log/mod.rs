use simple_log::LogConfigBuilder;

pub fn init_log() {
    let config = LogConfigBuilder::builder()
        .path("../log/log.log")
        .output_file()
        .output_console()
        .time_format("%Y%m%d%H%M%S")
        .build();
    if simple_log::new(config).is_err() {
        println!("error to config simple_log.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    #[test]
    fn test_log() {
        init_log();
        // 两者一样
        simple_log::info!("test info for simple log");
        info!("test info for log");
    }
}
