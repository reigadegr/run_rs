use std::{fs, thread};
use std::process::Command;
use std::time::Duration;
use crate::{utils};

fn bun_killer() {
    thread::spawn(move || {
        loop {
            Command::new("taskkill")
                .arg("/F").arg("/IM").arg("bun.exe")
                .output().unwrap();
            // sleep 1s
            thread::sleep(Duration::from_secs(1));
        }
    });
}

pub fn reflex_start() {
    if !fs::metadata("rxconfig.py").is_ok() {
        println!("当前目录下不存在rxconfig.py文件");
        return;
    }
    // let _ = utils::run_command("taskkill /F /IM node.exe".to_string());
    println!("reflex项目，开始使用npm安装依赖");

    let _ = utils::run_command(format!("{} config set registry https://registry.npmmirror.com", "npm.cmd"));

    if !fs::metadata(".web/package.json").is_ok() || !fs::metadata(".web").is_ok() {
        let _ = utils::run_command("reflex init".to_string());
    }
    let _ = utils::run_command(format!("{} config set strict-ssl false", "npm.cmd"));
    // let _ = utils::run_command("reflex script keep-chakra".to_string());
    bun_killer();
    // let _ = utils::run_command(format!("conda activate {} && reflex run --loglevel debug", env_name));
    let _ = utils::run_command("reflex run --loglevel debug".to_string());
}
