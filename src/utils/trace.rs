use tracing_subscriber::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use log::{error, info, warn};
use once_cell::sync::OnceCell;
use tracing::{dispatcher, Dispatch, Level};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

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
    let file_appender = tracing_appender::rolling::daily("logs", format!("{}.log", module));
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

#[test]
fn test_logger() {
    let main_logger = get_or_init_logger("main", Level::INFO);

    dispatcher::with_default(&main_logger, || {
        tracing::info!("主进程日志");
    });

    let render_logger = get_or_init_logger("render", Level::DEBUG);
    dispatcher::with_default(&render_logger, || {
        tracing::info!("渲染进程日志");
    })
}