use std::fs;
use crate::{utils};

pub fn mdbook_start(){
    if !fs::metadata("book.toml").is_ok() {
        println!("当前目录下不存在book.toml文件");
        return;
    }
    let _ = utils::run_command("mdbook serve --open".to_string());
}
