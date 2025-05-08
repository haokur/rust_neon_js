use log::{error, info, warn};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Mutex, OnceLock};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tracing::{dispatcher, Dispatch, Level};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

struct LogMsgContent {
    module: String,
    content: String,
}
static LOGGER: OnceLock<Sender<LogMsgContent>> = OnceLock::new();

/// 保存每个模块的日志分发器和写入线程守护器
pub struct ModuleLogger {
    pub dispatch: Dispatch,
    pub _guard: WorkerGuard,
}

/// 全局日志注册表：模块名 -> ModuleLogger
static LOGGER_MAP: OnceCell<Mutex<HashMap<String, ModuleLogger>>> = OnceCell::new();

/// 初始化全局 map（仅需调用一次）
pub fn init_log_registry() {
    LOGGER_MAP.get_or_init(|| Mutex::new(HashMap::new()));
}

/// 获取或初始化某个模块的日志器
pub fn get_or_init_logger(module: &str, level: Level) -> Dispatch {
    init_log_registry();

    let map = LOGGER_MAP.get().unwrap();
    let mut guard = map.lock().unwrap();

    if let Some(existing) = guard.get(module) {
        return existing.dispatch.clone();
    }

    // 新建 writer（日志文件在 logs/ 下）
    // let file_appender = tracing_appender::rolling::daily("logs", format!("{}.log", module));
    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(tracing_appender::rolling::Rotation::DAILY) // 每日滚动
        .filename_prefix(module)
        .filename_suffix("log")
        .build("logs")
        .expect("Failed to create appender");
    let (writer, guard_writer) = tracing_appender::non_blocking(file_appender);

    let fmt_layer = fmt::Layer::default()
        .with_writer(writer.clone())
        .with_ansi(false)
        .with_target(true)
        .with_filter(EnvFilter::from_default_env().add_directive(level.into()));

    let subscriber = Registry::default().with(fmt_layer);
    let dispatch = Dispatch::new(subscriber);

    guard.insert(
        module.to_string(),
        ModuleLogger {
            dispatch: dispatch.clone(),
            _guard: guard_writer,
        },
    );

    dispatch
}

fn init_logger_thread() {
    let (tx, rx) = mpsc::channel::<LogMsgContent>();
    LOGGER.set(tx).ok();
    let main_logger = get_or_init_logger("main", Level::INFO);
    let render_logger = get_or_init_logger("render", Level::DEBUG);

    let handle = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            let content = msg.content.to_string();
            // println!("{} {}", msg.module, content);
            if msg.module == "main" {
                dispatcher::with_default(&main_logger, || {
                    tracing::info!("{}", content);
                });
            } else if msg.module == "render" {
                dispatcher::with_default(&render_logger, || {
                    tracing::info!("{}", content);
                });
            }
        }
    });
}

pub fn logger_execute(module: String, msg: String) {
    if LOGGER.get().is_none() {
        init_logger_thread();
    }

    if let Some(tx) = LOGGER.get() {
        let _ = tx.send(LogMsgContent {
            module: module.to_string(),
            content: msg.to_string(),
        });
    }
}

#[test]
fn test_logger() {
    // let main_logger = get_or_init_logger("main", Level::INFO);
    // dispatcher::with_default(&main_logger, || {
    //     tracing::info!("主进程日志");
    // });
    //
    // let render_logger = get_or_init_logger("render", Level::DEBUG);
    // dispatcher::with_default(&render_logger, || {
    //     tracing::info!("渲染进程日志");
    // })
    if LOGGER.get().is_none() {
        init_logger_thread();
    }

    let msg = LogMsgContent {
        module: "main".to_string(),
        content: "Hello World!".to_string(),
    };
    if let Some(tx) = LOGGER.get() {
        // let _ = tx.send(msg); // 可以加上错误处理
        // let _ = tx.send(LogMsgContent {
        //     module: "main".to_string(),
        //     content: "Hello World from main!".to_string(),
        // });
        let _ = tx.send(LogMsgContent {
            module: "render".to_string(),
            content: "Hello World from render!".to_string(),
        });
    }

    sleep(Duration::from_millis(1000));
}
