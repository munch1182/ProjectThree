use simple_log::LogConfigBuilder;

pub(crate) fn init_log() {
    let config = LogConfigBuilder::builder()
        .path("./log/log.log")
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S") //E.g:%H:%M:%S.%f
        .level("debug")
        .output_file()
        .output_console()
        .build();

    if let Err(e) = simple_log::new(config) {
        println!("fail to init log: {}", e)
    }
}