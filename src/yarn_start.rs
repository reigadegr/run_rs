use std::{fs, thread};
use crate::{pnpm_start::run_pnpm_commands, utils};

pub fn run_yarn_commands(yarn: &str) {
    //检查当前目录下是否存在package.json
    if !fs::metadata("package.json").is_ok() {
        println!("当前目录下不存在package.json文件");
        return;
    }
    //检查package.json内是否含有pnpm字符串，如果有，则使用变量遮蔽的方式把yarn替换为pnpm
    let package_json = fs::read_to_string("package.json").unwrap();
    if package_json.contains("only-allow pnpm") {
        println!("检测到package.json内含有only-allow pnpm字符串，将使用pnpm进行安装");
        run_pnpm_commands("pnpm.cmd");
        return;
    }
//删除当前目录下的.npmrc
    let _ = fs::remove_file(".npmrc");
    //使用系统命令，检查yarn安装，如果没有安装则安装
    let _ = utils::run_command("yarn -v || npm install -g yarn".to_string());
    // println!("开始移除node_modules");
    // if let Err(e) = fs::remove_dir_all("node_modules") {
    //     println!("Failed to remove node_modules: {}", e);
    // }
    // println!("移除完毕");
    // let yarn = "npm";
    let _ = utils::run_command(format!("{} config set registry https://registry.npm.taobao.org", yarn));
    let _ = utils::run_command(format!("{} config set strict-ssl false", yarn));
    // 执行系统命令yarn cache clean
    // let _ = utils::run_command(format!("{} cache clean", yarn));
    // let _ = utils::run_command(format!("{} set version stable", yarn));
    println!("开始安装依赖");

    let _ = utils::run_command(format!("{} install", yarn));
    println!("开始启动");
    let _ = utils::run_command(format!("{} dev", yarn));
    // 将 `yarn` 转换为有 'static 生命周期的字符串
    let yarn_owned = yarn.to_string();

    // 启动新线程
    let handle = thread::spawn(move || {
        println!("开始打包产品");
        let _ = utils::run_command_no_msg(format!("{} build", yarn_owned));
        println!("打包完毕");
    });

    // 等待新线程完成
    handle.join().unwrap();
}