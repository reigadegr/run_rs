use std::{fs, thread};
use crate::{repalce_str, utils};

pub fn run_pnpm_commands(yarn: &str) {
    //删除当前目录下的.npmrc
    let _ = fs::remove_file(".npmrc");
    //添加判断，如果是windows系统才执行

    if cfg!(target_os = "windows") {
        let _ = repalce_str("package.json", "NODE_OPTIONS=--max-old-space-size=4096 vite", "set NODE_OPTIONS=--max-old-space-size=4096 && vite");
        let _ = repalce_str("package.json", "rimraf dist && NODE_OPTIONS=--max-old-space-size=8192 vite build", "rimraf dist && set NODE_OPTIONS=--max-old-space-size=8192 && vite build");
    }
    let _ = utils::run_command(format!("{} config set registry https://registry.npmmirror.com", "npm.cmd"));
    let _ = utils::run_command(format!("{} config set strict-ssl false", "npm.cmd"));
    //使用系统命令，检查安装，如果没有安装则安装
    let _ = utils::run_command((yarn.to_owned() + " -v || npm install -g " + yarn).to_string());

    let _ = utils::run_command(format!("{} config set registry https://registry.npmmirror.com", yarn));
    let _ = utils::run_command(format!("{} config set strict-ssl false", yarn));
    let _ = utils::run_command(format!("{} install", yarn));
    // let _ = utils::run_command(format!("{} add vue-demi", yarn));
    // let _ = utils::run_command(format!("{} add tippy.js", yarn));

    // 将 `yarn` 转换为有 'static 生命周期的字符串
    let yarn_owned = yarn.to_string();
    // 启动新线程
    let handle = thread::spawn(move || {
        println!("开始打包产品");
        let _ = utils::run_command_no_msg(format!("{} build", yarn_owned));
        println!("打包完毕");
    });
    let _ = utils::run_command(format!("{} dev", yarn));
    let _ = utils::run_command(format!("{} serve", yarn));
    let _ = utils::run_command("vite".to_string());
    // 等待新线程完成
    handle.join().unwrap();
}