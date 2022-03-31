use anyhow::Result;
use serde::de;
use serde_json::from_str;

// pub fn json_file_to_struct<'a, T>(filepath: &str) -> Result<T>
//     where
//         T: de::Deserialize<'a>, {
pub fn json_to_struct<'a, T>(content: &'a str) -> Result<T>
    where
        T: de::Deserialize<'a>, {
    let res = from_str::<T>(content).map_err(|e| {
        anyhow::Error::new(e)
    })?;
    Ok(res)
}


#[cfg(test)]
mod test {
    use crate::agent::TaskPingAlive;

    use super::*;

    //cargo test commons::json_operator::test::test_json_to_struct -- --nocapture
    #[test]
    fn test_json_to_struct() {
        let content = r#"
        [
          {
              "hosts":[
                  {
                      "instance_id":"i-c0rzu6vcvc",
                      "ip":"172.29.32.64"
                  },
                  {
                      "instance_id":"i-d0c6ep1m6m",
                      "ip":"172.29.32.34"
                  },
                  {
                      "instance_id":"i-zyce45h7ie",
                      "ip":"172.29.32.30"
                  }
              ],
              "name":"K8S商业平台"
          },
          {
              "hosts":[
                  {
                      "instance_id":"i-su5s44upp2",
                      "ip":"172.16.24.47"
                  },
                  {
                      "instance_id":"i-fokii3hnt5",
                      "ip":"172.16.24.46"
                  },
                  {
                      "instance_id":"i-twll3tsmp3",
                      "ip":"172.16.24.7"
                  },
                  {
                      "instance_id":"i-f4ppwxpqz1",
                      "ip":"172.16.24.16"
                  }
              ],
              "name":"直播"
          },
          {
              "hosts":[
                  {
                      "instance_id":"i-1jm8whxtu6",
                      "ip":"172.16.42.39"
                  },
                  {
                      "instance_id":"i-6tty916a4v",
                      "ip":"172.16.42.36"
                  }
              ],
              "name":"智能匹配"
          },
          {
              "hosts":[
                  {
                      "instance_id":"i-g4u18zjhd2",
                      "ip":"172.16.49.50"
                  }
              ],
              "name":"云会议室MMR"
          }]
        "#;


        let res = json_to_struct::<Vec<TaskPingAlive>>(content);
        println!("res is {:?}", res);
    }
}