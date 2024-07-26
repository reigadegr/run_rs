use std::{fs, thread};
use std::process::Command;
use std::time::Duration;
use crate::{utils};

fn bun_killer() {
    thread::spawn(move || {
        loop {
            let _ = Command::new("taskkill")
                .arg("/F").arg("/IM").arg("bun.exe")
                .output()
                .unwrap();
            // sleep 1s
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


    if !fs::metadata(".web/package.json").is_ok() || !fs::metadata(".web").is_ok() {
        let _ = utils::run_command(format!("conda activate {} && reflex init", env_name));
    } else {
        let _ = utils::run_command("reflex init".to_string());
    }
    let _ = utils::run_command(format!("{} config set strict-ssl false", "npm.cmd"));
    let _ = utils::run_command(format!("{} config set strict-ssl false", "pnpm.cmd"));
    if !fs::metadata(".web/node_modules").is_ok() {
        let _ = utils::run_command("cd .web && pnpm i && cd ..".to_string());
        let _ =utils::run_command("cd .web && pnpm add autoprefixer@10.4.14 postcss@8.4.31 postcss-import@16.1.0 @emotion/react@11.11.1 axios@1.6.0 json5@2.2.3 next@14.0.1 next-sitemap@4.1.8 next-themes@0.2.1 react@18.2.0 react-dom@18.2.0 react-focus-lock@2.11.3 socket.io-client@4.6.1 universal-cookie@4.0.4 react-syntax-highlighter@15.5.0 @radix-ui/react-dialog@1.0.5 react-markdown@8.0.7 vaul@0.9.1 remark-math@5.1.1 lucide-react@0.359.0 @chakra-ui/react@2.6.1 rehype-katex@6.0.3 @chakra-ui/system@2.5.7 remark-gfm@3.0.1 rehype-raw@6.1.1 react-loading-icons@1.1.0 react-error-boundary@4.0.13 framer-motion@10.16.4 remark-unwrap-images@4.0.0 @radix-ui/themes@3.0.0 && cd ..".to_string());
        let _ = utils::run_command("cd ..".to_string());
    }
    bun_killer();
    let _ = utils::run_command(format!("conda activate {} && reflex run --loglevel debug", env_name));
    let _ = utils::run_command("reflex run --loglevel debug".to_string());
}
