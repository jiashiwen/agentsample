use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::channel::Receiver;
use tokio::time::Instant;
use crate::curl_response_status_code;

#[derive(Clone)]
pub struct Task {
    pub parallel: usize,
    // pub conter: Arc<AtomicUsize>,
    // tiker: Receiver<Instant>,
}

impl Task {
    pub fn default() -> Self {
        Self {
            parallel: 1,
            // conter: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn new(max_parallel: usize) -> Self {
        Self {
            parallel: max_parallel,
            // conter: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub async fn run(&self) {
        let curl_status = curl_response_status_code("https://www.baidu.com");
        println!("query url status:{:?} , ", curl_status);
    }
}