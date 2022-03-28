use std::process::Command;
use std::time::Duration;
use regex::Regex;
use anyhow::Result;
use curl::easy::Easy;

fn ping_get_time(text: &str) -> Option<&str> {
    let regex = Regex::new(r"time=(.*) ms");
    return match regex {
        Ok(re) => {
            let res = re.captures(text)
                .and_then(|cap| {
                    let time = cap.get(0);
                    time.map(|t| t.as_str())
                });
            res
        }
        Err(_) => { None }
    };
}

fn cmd_ping(addr: &str) -> Result<String> {
    let cmd = format!("ping {} -c 3 -t 10", addr);
    let ping_cmd = Command::new("sh").arg("-c")
        .arg(cmd.as_str()).output()?;

    let str = std::str::from_utf8(&*ping_cmd.stdout).map_err(|e| {
        anyhow::Error::new(e)
    })?;
    Ok(str.to_string())
}

fn check_ping(addr: &str) -> bool {
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

pub fn curl_response_status_code(uri: &str) -> Result<u32> {
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
