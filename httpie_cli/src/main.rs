use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Ok, Result};
use clap::{command, Parser};
use colored::Colorize;
use mime::Mime;
use reqwest::{
    header::{self, CONTENT_TYPE},
    Client, Response, Url,
};
#[derive(Parser, Debug, Clone)]
struct Get {
    /// http 请求的 url
    #[clap(value_parser = parse_url)]
    url: String,
}

#[derive(Parser, Debug, Clone)]
struct Post {
    /// http 请求的 url
    #[clap(value_parser = parse_url)]
    url: String,
    /// http 请求的 body
    #[clap(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Parser, Debug, Clone)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

/// A naive httpie implementation with Rust, can you imagine how easy is it?
// derive 宏，为 Opts 自动派生（derive）Parser, Debug, Clone 特性（traits）的实现
// Parser 特性是 clap 库提供的，它允许 Opts 结构体用作命令行解析的目标。
// Debug 特性允许你使用 {:?} 格式化字符串打印 Opts 实例的调试信息。
// Clone 特性允许你复制 Opts 实例。
#[derive(Parser, Debug, Clone)]
// 这是 clap 的属性宏，用于定义命令的元数据。
#[command(version = "1.0", about = None, author = "yi.hu")]
struct Opts {
    // 使用 clap 属性宏，指示 subcmd 为子命令，SubCommand 用于存储子命令的枚举
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 校验 url 的合法性
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

/// 为 KvPair 结构体实现 FromStr 特性，使用后可以通过 str.parse 将字符串转换为 KvPair 的格式
impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 迭代器 Iterator 格式
        let mut split = s.split("=");
        let err = || anyhow!("Failed to parse {}", s);
        Ok(Self {
            // 使用 next 可以类似链式取出元素，将返回 Ok(T) 或者 None，从第一个元素开始。
            // 取出第一个元素 作为 key，将其转化为 Ok(T)/Err(E), 使用 ？来处理错误返回
            k: (split.next().ok_or_else(err)?).to_string(),
            // 取出第二个元素 作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
// PartialEq 允许使用 assert_eq 对比是否相等
#[derive(Debug, Clone, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let res = client.get(&args.url).send().await?;
    Ok(print_response(res).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let res = client.post(&args.url).json(&body).send().await?;
    Ok(print_response(res).await?)
}

// 打印服务器 版本号 + 状态
fn print_status(resp: &Response) {
    // blue 来自 colored::Colorize
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的 HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!("\n");
}

// 打印服务器返回的 body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        // 如果 为 APPLICATION_JSON 格式的返回，使用 pretty print 打印
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => println!("{}", body),
    }
}

// 将服务器返回的 content-type 转化为 mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

// 打印整个响应
async fn print_response(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let m = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(m, &body);
    Ok({})
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERD-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    println!("{:?}", opts);
    let client = Client::builder().default_headers(headers).build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}

// 仅在 test 模式下执行，单元测试 cargo test 执行
#[cfg(test)]
mod tests {
    use super::*;

    // 测试验证 parse_url 功能是否正常
    #[test]
    fn parse_url_works() {
        assert!(parse_url("abcd").is_err());
        assert!(parse_url("http://abc.com").is_ok());
        assert!(parse_url("https://httpbon.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into()
            }
        );
        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into()
            }
        );
    }
}
