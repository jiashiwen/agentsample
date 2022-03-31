use std::fs;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TaskPingAlive {
    pub name: String,
    pub hosts: Vec<Host>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Host {
    pub instance_id: String,
    pub ip: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TaskHttp {
    pub app: String,
    pub curl: String,
    pub domain: String,
    pub url: String,
}

impl Host {
    pub fn run(&self, name: &str) {
        println!("name is:{:?};id is:{:?};ip is:{:?}", name, self.instance_id, self.instance_id);
    }
}

impl TaskHttp {
    pub fn run(&self) {
        println!("app is:{:?};curl is:{:?};domain is:{:?};url is:{:?}", self.app
                 , self.curl, self.domain, self.url);
    }
}

pub fn file_to_ping_tasks(path: &str) -> Result<Vec<TaskPingAlive>> {
    let content = fs::read_to_string(path).map_err(|e| {
        anyhow::Error::new(e)
    })?;
    let vec = from_str::<Vec<TaskPingAlive>>(content.as_str()).map_err(|e| {
        anyhow::Error::new(e)
    })?;
    Ok(vec)
}

pub fn file_to_http_tasks(path: &str) -> Result<Vec<TaskHttp>> {
    let content = fs::read_to_string(path).map_err(|e| {
        anyhow::Error::new(e)
    })?;
    let vec = from_str::<Vec<TaskHttp>>(content.as_str()).map_err(|e| {
        anyhow::Error::new(e)
    })?;
    Ok(vec)
}

#[cfg(test)]
mod test {
    use super::*;

    //cargo test agent::task_ping_alive::test::test_file_to_ping_tasks -- --nocapture
    #[test]
    fn test_file_to_ping_tasks() {
        let res = file_to_ping_tasks("ping.json");
        println!("res is {:?}", res);
    }

    //cargo test agent::task_ping_alive::test::test_file_to_http_tasks -- --nocapture
    #[test]
    fn test_file_to_http_tasks() {
        let res = file_to_http_tasks("curl.json");
        println!("res is {:?}", res);
    }
}
