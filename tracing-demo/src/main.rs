use std::io;

use log::{info, trace};
use tracing::{event, level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry
};
/**
 * 关键struct：
 * - Subscriber
 * - Registry
 * - SubscriberBuilder
 *
 * 关键 trait
 *  - Layer
 *  - Subscriber
 */
fn main() {}
//这种方式需要设置环境变量才行显示日志RUST_LOG=debug
#[allow(unused)]
fn log_controller_by_env() {
    tracing_subscriber::fmt::init();
    info!("an example trace log");
    event!(Level::INFO, "something happened");
}
#[allow(unused)]
fn log_io_stdout() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_writer(io::stdout)
        .with_max_level(LevelFilter::TRACE)
        .finish()
        .init();
    trace!("an example trace log");
}
//需要依赖crate lib:tracing-appender
#[allow(unused)]
fn log_io_file() {
    let file_appender = tracing_appender::rolling::daily("./log", "prefix.log");
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_writer(file_appender)
        .with_max_level(LevelFilter::TRACE)
        .finish()
        .init();
    trace!("an example trace log");
}
#[allow(unused)]
// 日志打印到控制台和日志文件里
fn log_out_std_and_file() {
    let registry = Registry::default();
    let iostd = tracing_subscriber::fmt::layer();
    let file_appender = tracing_appender::rolling::daily("./log", "prefix-1.log");
    let file = tracing_subscriber::fmt::layer()
    .with_writer(file_appender);
    registry.with(iostd).with(file).init();
    trace!("an log_out_std_and_file example trace log");

}
#[cfg(test)]
mod tests {
    use crate::{log_controller_by_env, log_io_file, log_io_stdout, log_out_std_and_file};

    #[test]
    fn test_log_controller_by_env() {
        log_controller_by_env();
    }
    #[test]
    fn test_log_io_stdout() {
        log_io_stdout();
    }
    #[test]
    fn test_log_io_file() {
        log_io_file();
    }
    #[test]
    fn test_log_out_std_and_file(){
        log_out_std_and_file()
    }
}
