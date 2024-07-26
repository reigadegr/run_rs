use std::{fs, thread};
use std::process::Command;
use std::time::Duration;
use crate::{utils};

fn bun_killer() {
    thread::spawn(move || {
        loop {
            // 执行系统命令
            let _ = Command::new("taskkill")
                .arg("/F").arg("/IM").arg("bun.exe")
                .output()
                .unwrap();
            // 休眠一秒
            thread::sleep(Duration::from_secs(1));
        }
    });
}
pub fn reflex_start(env_name: &str) {
    if !fs::metadata("rxconfig.py").is_ok() {
        println!("当前目录下不存在rxconfig.py文件");
        return;
    }
    println!("reflex项目，开始使用npm安装依赖");

    if !fs::metadata(".web").is_ok() {
        let _ = utils::run_command(format!("conda activate {} && reflex init", env_name));
    }

    if !fs::metadata(".web/package.json").is_ok() {
        let _ = utils::run_command(format!("conda activate {} && reflex init", env_name));
    }
    let _ = utils::run_command(format!("{} config set strict-ssl false", "npm.cmd"));
    bun_killer();
    let _ = utils::run_command(format!("conda activate {} && reflex run --loglevel debug", env_name));
    let _ = utils::run_command("reflex run --loglevel debug".to_string());
}
