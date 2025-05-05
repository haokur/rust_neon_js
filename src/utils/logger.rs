use std::sync::mpsc::{self, Sender};
use std::thread;

use flexi_logger::writers::LogWriter;
use flexi_logger::{
    Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, LoggerHandle, Naming,
};
use log::{debug, info, LevelFilter, Record};
use std::sync::{Mutex, OnceLock};
use std::thread::sleep;
use std::time::Duration;

static LOGGER_HANDLE: OnceLock<Mutex<LoggerHandle>> = OnceLock::new();

static LOGGER: OnceLock<Sender<String>> = OnceLock::new();
const IS_PRODUCTION: bool = true;

struct CustomWriter;
impl LogWriter for CustomWriter {
    fn write(&self, _now: &mut DeferredNow, _record: &Record) -> std::io::Result<()> {
        // println!("custom write record is {:?}", record);
        // 自定日志输出拦截，可推送到服务器，elasticsearch等
        Ok(())
    }
    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }
}

// 日志信息脱敏
fn sanitize_log_record(record: &Record) -> String {
    let record_str = record.args().to_string();
    // TODO
    return record_str
}

struct LoggerConfig {
    directory: String, // logs
    basename: String, // cli
    log_level: String, // info debug
    duplicate_level:Duplicate,
}

pub fn init_logger(logger_config: LoggerConfig) {
    let format = |write: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record| {
        let time_str = now.format("%Y-%m-%d %H:%M:%S");
        write!(
            write,
            "{} [{}] {}",
            time_str,
            record.level(),
            sanitize_log_record(record)
        )
            .unwrap();
        Ok(())
    };

    let log_file_path = FileSpec::default()
        .directory(logger_config.directory)
        .basename(logger_config.basename)
        .suffix("log");
    // .discriminant(Local::now().format("%Y-%m-%d").to_string());

    let log_level = logger_config.log_level;
    let duplicate_level = logger_config.duplicate_level;

    let logger_handle = Logger::try_with_str(log_level)
        .unwrap()
        .duplicate_to_stdout(duplicate_level)
        .format(format)
        .log_to_file_and_writer(log_file_path, Box::new(CustomWriter))
        .rotate(
            Criterion::Size(10 * 1024 * 1024),
            // Criterion::Size(1024),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(10),
        )
        .append()
        .start()
        .unwrap();

    LOGGER_HANDLE
        .set(Mutex::new(logger_handle))
        .unwrap_or_else(|_| println!("LOGGER_HANDLE set logger_handle error"));
    info!("logger serve init successful");
}

fn init_logger_thread() {
    let (tx, rx) = mpsc::channel::<String>();
    LOGGER.set(tx).ok();

    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            info!("[Rust Log]: {}", msg);
            sleep(Duration::new(10, 0));
        }
    });
}

pub fn logger_execute(msg: String) {
    println!("[Rust Log]: {}", msg);
    // if LOGGER.get().is_none() {
    //     init_logger();
    //     init_logger_thread();
    // }
    //
    // if let Some(tx) = LOGGER.get() {
    //     let _ = tx.send(msg); // 可以加上错误处理
    // }
}

#[test]
fn test_logger() {
    // logger_execute(String::from("test"));
    // thread::sleep(Duration::from_secs(1));


}
