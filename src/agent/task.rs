use crate::agent::{check_ping_get_time, url_response_status_code};

#[derive(Clone)]
pub struct Task {
    pub parallel: usize,
}

impl Task {
    pub fn default() -> Self {
        Self {
            parallel: 1,

        }
    }

    pub fn new(max_parallel: usize) -> Self {
        Self {
            parallel: max_parallel,
        }
    }

    pub async fn run(&self) {
        let ping_result = check_ping_get_time("127.0.0.1");
        let curl_status = url_response_status_code("https://www.baidu.com");
        println!("ping result is:{:?} query url status:{:?} , ", ping_result, curl_status);
    }
}