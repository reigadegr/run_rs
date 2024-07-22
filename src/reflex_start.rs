use std::fs;
use crate::{utils};

pub fn reflex_start(env_name: &str) {
    if !fs::metadata("rxconfig.py").is_ok() {
        println!("当前目录下不存在package.json文件");
        return;
    }
    let _ = utils::run_command(format!("conda acivate {} && reflex run", env_name));
}
