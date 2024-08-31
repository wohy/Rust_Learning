use std::{env, fs};
use clap::{Arg, ArgAction, Command};

async fn html_to_md() {
    // 普通方法
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 3 {
    //     println!("please input the -- <url> <target>");
    //     println!("example: cargo run -- https://www.rust-lang.org/ rust.md");
    //     return ;
    // }
    // // args[0] 指的是当前程序名称 target/debug/html2md_learning
    // let arg1 = &args[1];
    // let arg2 = &args[2];
    // println!("Converting {} html to markdown {}", arg1, arg2);

    // 使用 clap  执行命令  cargo run https://www.rust-lang.org/ rust.md 即可，需要查看帮助则输入 cargo run change -h
    let matches = Command::new("change")
    .version("0.1.0")
    .author("yi.hu")
    .about("change html to md")
    .arg(
        Arg::new("url")
        .alias("u")
        .required(true)
        .help("输入想要转换的目标html的url")
        .action(ArgAction::Set)
    )
    .arg(
        Arg::new("file_name")
        .alias("n")
        .required(true)
        .help("输入转换成功后输出的文件名")
        .action(ArgAction::Set)
    )
    .get_matches();

    let mut arg1 = String::new();
    let mut arg2 = String::new();
    if let Some(url) = matches.get_one::<String>("url") {
        arg1 = url.clone();
    }
    if let Some(file_name) = matches.get_one::<String>("file_name") {
        arg2 = file_name.clone()
    }

    let body: Result<reqwest::Response, reqwest::Error> = reqwest::get(arg1)
        .await;

    match body {
        Ok(res) => {
            println!("Converting html to markdown...");
            if let Ok(res_str) = res.text().await {

                let md = html2md::parse_html(&res_str);

                match fs::write(&arg2, md.as_bytes()) {
                    Ok(_) => println!("Converted markdown has been in {}.", &arg2),
                    Err(err) => {
                        println!("Converting html to markdown error: {}", err);
                    }
                };
            } else {
                println!("Converting html to markdown error");
            }
        }
        Err(err) => {
            println!("Converting html to markdown error: {}", err);
        }
    }
}

#[tokio::main]
async fn main() {
    html_to_md().await;
}
