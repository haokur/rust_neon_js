mod utils;

use crate::utils::trace::get_or_init_logger;
use tracing::dispatcher;
use tracing::{error, info, warn, Level};

fn log_main() {
    let logger = get_or_init_logger("main", Level::INFO);

    dispatcher::with_default(&logger, || {
        tracing::info!("主进程日志");
    });
}

fn log_render(){
    let logger = get_or_init_logger("render", Level::INFO);

    dispatcher::with_default(&logger, || {
        tracing::info!("渲染进程日志");
    });
}

#[tokio::main]
async fn main() {
    log_main();
    log_render();
    // let main_logger = get_or_init_logger("main", tracing::Level::INFO);
    // let renderer_logger = get_or_init_logger("renderer", tracing::Level::DEBUG);
    //
    // main_logger.with(|| {
    //     info!("主进程启动");
    // });
    //
    // renderer_logger.with(|| {
    //     info!("渲染初始化完成");
    //     warn!("渲染性能预警");
    //     error!("渲染模块崩溃");
    // });
}
