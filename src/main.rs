use crate::logger::init_log;

mod agent;
mod cmd;
mod commons;
mod configure;
mod logger;
mod httpquerry;

#[tokio::main]
async fn main() {
    init_log();
    cmd::run_app();
}
