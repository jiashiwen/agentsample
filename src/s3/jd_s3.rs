use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Error, Result};
use async_once::AsyncOnce;
use aws_sdk_s3::{Client, Endpoint};
use aws_sdk_s3::output::ListObjectsOutput;
use aws_types::Credentials;
use aws_types::region::Region;
use http::Uri;

#[derive(Clone)]
pub struct Jd_S3_Client {
    pub endpoint: String,
    pub access_key: String,
    pub secret_access_key: String,
    pub region: String,
    pub s3_client: Client,
}

impl Jd_S3_Client {
    pub async fn new(endpoint_str: String, ak: String, sk: String, region: String) -> Self {
        // let endpoint = Endpoint::mutable(Uri::from(endpoint_str.clone().as_str()));
        let uri = endpoint_str.parse().unwrap();
        let endpoint = Endpoint::mutable(Uri::from(uri));
        let c = Credentials::new(ak.as_str(),
                                 sk.as_str(),
                                 None,
                                 None,
                                 "Static");
        let shared_config = aws_config::from_env().
            credentials_provider(c).
            region(Region::new(region.clone())).load().await;

        let mut s3_config_builder = aws_sdk_s3::config::Builder::from(&shared_config);
        s3_config_builder = s3_config_builder.endpoint_resolver(endpoint);
        let client = aws_sdk_s3::Client::from_conf(s3_config_builder.build());
        Self {
            endpoint: endpoint_str,
            access_key: ak,
            secret_access_key: sk,
            region,
            s3_client: client,
        }
    }

    pub async fn get_obj_list(&self, bucket: String, prefix: String) -> Result<ListObjectsOutput> {
        let obj_list = self.s3_client.list_objects()
            .bucket(bucket)
            .prefix(prefix)
            .send().await.map_err(|e| {
            Error::new(e)
        })?;
        Ok(obj_list)
    }

    pub async fn download_obj(self, bucket: String, key: String, path: String) -> Result<()> {
        let resp = self.s3_client
            .get_object()
            .bucket(bucket)
            .key(key.clone()).send().await?;
        let data = resp.body.collect().await?;

        let bytes = data.into_bytes();
        let v: Vec<_> = key.split('/').collect();

        if let Some(filename) = v.last() {
            let mut store_to = path;
            if !store_to.as_str().ends_with("/") {
                store_to.push_str("/");
            }
            store_to.push_str(filename);

            let store_path = Path::new(store_to.as_str());
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(store_path)?;
            file.write(&*bytes);
        } else {
            return Err(anyhow!("no file found"));
        }

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    //cargo test s3::jd_s3::test::test_jdcloud_s3_client -- --nocapture
    #[test]
    fn test_jdcloud_s3_client() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        // 使用 block_on 调用 async 函数
        let shared_config = rt.block_on(async {
            let client = Jd_S3_Client::new("http://s3.cn-north-1.jdcloud-oss.com".to_string(),
                                           "4107B314B15BCE99A1C781DFCF119F59".to_string(),
                                           "8877CD432EB5738EFF0FA01F630201C9".to_string(),
                                           "cn-north-1".to_string()).await;
            let obj_list = client
                .get_obj_list("pingdata".to_string(), "pingdata/5aa4d3ed66bfb7cb8938563d2bb517cd".to_string())
                .await;

            for item in obj_list.unwrap().contents().unwrap() {
                println!("{:?}", item.key());
            }
        });
    }

    //cargo test s3::jd_s3::test::test_jdcloud_s3_download -- --nocapture
    #[test]
    fn test_jdcloud_s3_download() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(
            async {
                let client = Jd_S3_Client::new("http://s3.cn-north-1.jdcloud-oss.com".to_string(),
                                               "4107B314B15BCE99A1C781DFCF119F59".to_string(),
                                               "8877CD432EB5738EFF0FA01F630201C9".to_string(),
                                               "cn-north-1".to_string()).await;
                let res = client.download_obj("tsp-picture".to_string(),
                                              "tsp-picture/46b6d4e6-1446-4da2-a10b-91fd3d73cebb.jpg".to_string(),
                                              "/tmp/".to_string()).await;

                println!("{:?}", res);
            }
        );
    }
}