use flexi_logger::writers::LogWriter;
use flexi_logger::{
    Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, LoggerHandle, Naming,
};
use std::collections::HashMap;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};

type LogSender = mpsc::Sender<String>;
static LOG_WRITERS: OnceLock<Mutex<HashMap<String, LogSender>>> = OnceLock::new();

/// 初始化全局日志通道映射
pub fn init_logger_pool() {
    LOG_WRITERS.get_or_init(|| Mutex::new(HashMap::new()));
}

struct LoggerConfig {
    directory: String, // logs
    basename: String,  // cli
    log_level: String, // info debug
    duplicate_level: i32,
}

// 获取一个日志打印实例
fn get_logger(logger_config: LoggerConfig) -> LoggerHandle {
    let log_file_path = FileSpec::default()
        .directory(&logger_config.directory)
        .basename(&logger_config.basename)
        .suffix("log");
    let log_level = logger_config.log_level;
    let duplicate_level = logger_config.duplicate_level;

    let logger_handle = Logger::try_with_str(log_level)
        .unwrap()
        .log_to_file(log_file_path)
        .rotate(
            Criterion::Size(10 * 1024 * 1024),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(10),
        )
        .append()
        .start()
        .unwrap();

    logger_handle
}

async fn get_instance(logger_config: LoggerConfig) {
    let mut map: HashMap<String, LoggerHandle> = HashMap::new();
    let module_name = logger_config.basename.clone();
    let logger = get_logger(logger_config);
    map.insert(module_name, logger);
}

#[test]
fn test_instance() {
    let logger_config = LoggerConfig {
        directory: String::from("logs"),
        basename: String::from("cli2"),
        log_level: String::from("info"),
        duplicate_level: 1,
    };
    let _ = get_instance(logger_config);
    thread::sleep(Duration::from_secs(30));
    // let logger_module = String::from("renderer");
    // // let map = LOG_WRITERS.get().expect("Logger pool not initialized");
    // // let mut writers = map.lock().await;
    // let logger_modules_map = LOG_WRITERS.get().unwrap().lock().await;
    //
    // let logger_config = LoggerConfig {
    //     directory: String::from("logs"),
    //     basename: String::from("cli2"),
    //     log_level: String::from("info"),
    //     duplicate_level: 1,
    // };
    // let logger = get_logger(logger_config);
    // info!("hello world");
    // println!("IS_PRODUCTION");
}
