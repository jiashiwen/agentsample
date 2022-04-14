use std::time::Duration;

use crossbeam::channel::tick;
use walkdir::WalkDir;

use crate::agent::{file_to_http_tasks, file_to_ping_tasks};
use crate::agent::task_get_desc::get_task_desc_files;
use crate::configure::get_config;

pub async fn start_agent(ticker_sec: usize) {
    let ticker = tick(Duration::from_secs(ticker_sec as u64));
    let cfg = get_config().unwrap();
    loop {
        let _ = ticker.recv().unwrap();
        // 下载描述文件
        get_task_desc_files().await;
        // 循环执行本地目录下的所有描述文件
        for entry in WalkDir::new(cfg.task_config.task_desc_local_path.clone())
        {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() {
                println!("{}", entry.path().display());
            }
        }
    }
}

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

#[cfg(test)]
mod test {
    use std::thread;

    use crate::checkers::check_local_desc_path;
    use crate::configure::set_config;

    use super::*;

    //cargo test agent::agent_server::test::test_start_agent -- --nocapture
    #[test]
    fn test_start_agent() {
        set_config("");
        check_local_desc_path();
        let agent_start_handle = thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            // 使用 block_on 调用 async 函数
            let shared_config = rt.block_on(async {
                start_agent(1).await;
            });
        });
        agent_start_handle.join().unwrap();
    }
}