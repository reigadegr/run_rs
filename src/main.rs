mod utils;
mod yarn_start;
mod pnpm_start;
mod reflex_start;
use std::{fs, io};

fn main() -> io::Result<()> {
    println!("欢迎使用本自动化脚本，可以运行vue和springboot程序");
    println!("作者：github@reigadegr");
    let yarn = "yarn.cmd";
    // let yarn = "pnpm.cmd";
    //王旭路径
    // let yarn = "C:\\Users\\wangxu\\AppData\\Roaming\\npm\\yarn.cmd";
    //白家驹路径
    // let yarn = "C:\\Users\\92138\\AppData\\Roaming\\npm\\yarn.cmd";
    //李昊
    // let yarn ="C:\\Users\\l1571\\AppData\\Roaming\\npm\\yarn.cmd";
    // let yarn ="D:\\nodejs\\node_global\\yarn.cmd";
    yarn_start::run_yarn_commands(yarn);

    run_spring_boot();
    println!("finished");
    let _ = utils::run_command("cmd".to_string());
    reflex_start::reflex_start("py12");
    Ok(())
}

fn run_spring_boot() {
    //检查当前目录下是否存在pom.xml
    if !fs::metadata("pom.xml").is_ok() {
        println!("当前目录下不存在pom.xml文件");
        return;
    }
    println!("开始移除target");
    let _ = fs::remove_dir_all("target");
    println!("移除完毕");

    let _ = utils::run_command("mvn clean install spring-boot:run".to_string());
}

fn repalce_str(contents: &str, target: &str, replacement: &str) -> io::Result<()> {
    // 读取package.json文件
    let contents1 = fs::read_to_string(contents)?;


    // 定义要替换的原始值和新值

    // 替换字符串
    let new_contents = contents1.replace(target, replacement);

    // 将新内容写回package.json文件
    fs::write(contents, new_contents)?;

    // 替换后的字符串
    // println!("Updated package.json content:\n{}", new_contents);

    Ok(())
}




