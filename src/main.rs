use crate::configure::set_config;
use crate::logger::init_log;

mod agent;
mod cmd;
mod commons;
mod configure;
mod logger;
mod httpquerry;
mod s3;
mod checkers;

#[tokio::main]
async fn main() {
    init_log();
    cmd::run_app();
}
