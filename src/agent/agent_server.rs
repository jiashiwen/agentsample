use std::time::Duration;

use crossbeam::channel::tick;

use crate::agent::{file_to_http_tasks, file_to_ping_tasks};

pub async fn start_ping_agent(ticker_sec: usize, exec_file: &str) {
    let ticker = tick(Duration::from_secs(ticker_sec as u64));
    loop {
        let _ = ticker.recv().unwrap();
        let tasks = file_to_ping_tasks(exec_file);
        if let Ok(vec_tasks) = tasks {
            for ping_tasks in vec_tasks.into_iter() {
                for host in ping_tasks.hosts.into_iter() {
                    let name = ping_tasks.name.clone();
                    tokio::spawn(async move {
                        host.run(name.as_str());
                    });
                }
            }
        }
    }
}

pub async fn start_curl_agent(ticker_sec: usize, exec_file: &str) {
    let ticker = tick(Duration::from_secs(ticker_sec as u64));
    loop {
        let _ = ticker.recv().unwrap();
        let tasks = file_to_http_tasks(exec_file);
        if let Ok(vec_tasks) = tasks {
            for curl_task in vec_tasks.into_iter() {
                tokio::spawn(async move {
                    curl_task.run();
                });
            }
        }
    }
}