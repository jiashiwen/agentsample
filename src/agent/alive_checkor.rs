use std::process::Command;
use std::time::Duration;

use anyhow::Result;
use curl::easy::Easy;
use regex::Regex;

fn check_ping_bool(addr: &str) -> bool {
    let cmd = format!("ping {} -c 3 -t 10", addr);
    let ping_cmd = Command::new("sh").arg("-c")
        .arg(cmd.as_str()).output().unwrap();

    let str = std::str::from_utf8(&*ping_cmd.stdout).unwrap();

    for l in str.lines() {
        if l.contains(',') {
            let res = l.split(',').collect::<Vec<&str>>();
            let recive_line = res[1].trim();
            let col_one = recive_line.split(' ').collect::<Vec<&str>>()[0];
            if col_one.parse::<usize>().is_ok() {
                if col_one.eq("0") {
                    return false;
                }
            }
        }
    }
    true
}

pub fn check_ping_get_time(addr: &str) -> Option<String> {
    let ping_res = cmd_ping(addr);
    match ping_res {
        Ok(txt) => {
            ping_get_time(txt)
        }
        Err(_) => { None }
    }
}

//取任意返回时间有值部分，精确到小数点后两位
fn ping_get_time(text: String) -> Option<String> {
    let regex = Regex::new(r"time=(.*) ms");
    let text_time = match regex {
        Ok(re) => {
            let res = re.captures(text.as_str())
                .and_then(|cap| {
                    let time = cap.get(0);
                    time.map(|t| t.as_str())
                });
            res
        }
        Err(_) => { None }
    };

    return match text_time {
        None => { None }
        Some(str) => {
            let first_col = str.split(' ').collect::<Vec<&str>>()[0];
            let number = first_col.split('=').collect::<Vec<&str>>()[1];
            let f = number.parse::<f32>().unwrap();
            let digits = format!("{:.2}", f);
            return Some(digits);
        }
    };
}

fn cmd_ping(addr: &str) -> Result<String> {
    let cmd = format!("ping {} -c 4 -t 10", addr);
    let ping_cmd = Command::new("sh").arg("-c")
        .arg(cmd.as_str()).output()?;

    let str = std::str::from_utf8(&*ping_cmd.stdout).map_err(|e| {
        anyhow::Error::new(e)
    })?;
    Ok(str.to_string())
}

fn curl_cmd_response(curl_cmd: &str) -> Result<String> {
    let curl_cmd = Command::new("sh").arg("-c")
        .arg(curl_cmd).output()?;
    let str = std::str::from_utf8(&*curl_cmd.stdout).map_err(|e| {
        anyhow::Error::new(e)
    })?;

    Ok(str.to_string())
}

pub fn url_response_status_code(uri: &str) -> Result<u32> {
    let mut easy = Easy::new();
    easy.timeout(Duration::from_secs(10));
    easy.url(uri).map_err(|e| {
        anyhow::Error::new(e)
    })?;
    easy.perform().map_err(|e| {
        anyhow::Error::new(e)
    })?;
    let resp = easy.response_code().map_err(|e| {
        anyhow::Error::new(e)
    })?;
    Ok(resp)
}

pub fn check_http_code(code: &str) -> bool {
    code == "200"
        || code == "301"
        || code == "302"
        || code == "401"
        || code == "403"
}


#[cfg(test)]
mod test {
    use super::*;

    //cargo test agent::alive_checkor::test::test_url_response_status_code -- --nocapture
    #[test]
    fn test_url_response_status_code() {
        let res = url_response_status_code(r"https://baidu.com");
        println!("res is {:?}", res);
    }

    //cargo test agent::alive_checkor::test::test_curl_cmd_response -- --nocapture
    #[test]
    fn test_curl_cmd_response() {
        let res = curl_cmd_response(r"curl -I -m 10 -o /dev/null -s -w %{http_code} www.baidu.com");
        println!("res is {:?}", res);
    }

    //cargo test agent::alive_checkor::test::test_check_http_code -- --nocapture
    #[test]
    fn test_check_http_code() {
        let res = check_http_code("201");
        println!("res is {:?}", res);
    }
}
