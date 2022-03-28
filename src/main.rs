mod agent;
mod cmd;
mod commons;
mod configure;
mod logger;

use std::borrow::Borrow;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::channel::tick;
use crate::agent::{curl_response_status_code, Task};
use crate::logger::init_log;


#[tokio::main]
async fn main() {
    init_log();
    cmd::run_app();
    // let n = Arc::new(AtomicUsize::new(0));
    // let start = Instant::now();
    // let ticker = tick(Duration::from_millis(1000));
    //
    // let task = Task::new(10);
    //
    // loop {
    //     let msg = ticker.recv().unwrap();
    //     let mut x = n.clone();
    //     let t = task.clone();
    //
    //     tokio::spawn(async move {
    //         println!("task parallel: {:?}", x.load(Ordering::Relaxed) < task.parallel);
    //         if x.load(Ordering::Relaxed) < task.parallel {
    //             x.fetch_add(1, Ordering::SeqCst);
    //             t.run().await;
    //             x.fetch_sub(1, Ordering::SeqCst);
    //         }
    //     });
    // }
}
