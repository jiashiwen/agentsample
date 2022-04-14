use anyhow::{anyhow, Result};
use async_once::AsyncOnce;
use aws_sdk_s3::model::Object;

use crate::configure::get_config;
use crate::s3::Jd_S3_Client;

lazy_static::lazy_static! {
    static ref GLOBAL_JDCLOUD_S3_Client: AsyncOnce<Jd_S3_Client> = AsyncOnce::new(async {
          let cfg = get_config().unwrap();
          let s3_client = Jd_S3_Client::new(
                          cfg.jdcloud.s3_endpoint,
                          cfg.jdcloud.ak,
                          cfg.jdcloud.sk,
                          cfg.jdcloud.region,
                         ).await;
        s3_client
    });
}

//获取任务描述文件
pub async fn get_task_desc_files() -> Result<()> {
    let cfg = get_config()?;
    let objectslist = GLOBAL_JDCLOUD_S3_Client.get().await.get_obj_list(
        cfg.task_config.task_desc_s3_bucket.clone(),
        cfg.task_config.task_desc_s3_prefix.clone()).await?;

    let file_list = objectslist.contents();

    match file_list {
        None => { Err(anyhow!("no objects")) }
        Some(objects) => {
            for item in objects {
                if let Some(key) = item.key.clone() {
                    let _ = GLOBAL_JDCLOUD_S3_Client.get().await.clone().download_obj(
                        cfg.task_config.task_desc_s3_bucket.clone(),
                        key,
                        cfg.task_config.task_desc_local_path.clone()).await;
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::configure::set_config;

    use super::*;

    //cargo test agent::task_get_desc::test::test_get_task_desc_files -- --nocapture
    #[test]
    fn test_get_task_desc_files() {
        set_config("");
        // set_config("/Users/jiashiwen/rustproject/agentsample/config.yml");
        let rt = tokio::runtime::Runtime::new().unwrap();
        // 使用 block_on 调用 async 函数
        let shared_config = rt.block_on(async {
            let res = get_task_desc_files().await;
            println!("{:?}", res);
        });
    }
}

