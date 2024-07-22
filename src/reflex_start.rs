use std::fs;
use crate::{utils};

pub fn reflex_start(env_name: &str) {
    if !fs::metadata("rxconfig.py").is_ok() {
        println!("当前目录下不存在rxconfig.py文件");
        return;
    }
    println!("reflex项目，开始使用npm安装依赖");
    if !fs::metadata(".web").is_ok() {
        let _ = utils::run_command(format!("conda activate {} && reflex init", env_name));
    }

    let _ = utils::run_command(format!("cd .web && {} install && cd ..", "npm.cmd"));
    let _ = utils::run_command(format!("conda activate {} && reflex run", env_name));
}
