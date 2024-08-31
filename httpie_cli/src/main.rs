use std::{collections::HashMap, env::Args, str::FromStr};

use anyhow::{Ok, anyhow, Result};
use clap::{command, Parser};
use reqwest::{Client, Url};
use serde_json::json;
#[derive(Parser, Debug, Clone)]
struct Get {
    /// http 请求的 url
    #[clap(value_parser = parse_url)]
    url: String
}

#[derive(Parser, Debug, Clone)]
struct Post {
    /// http 请求的 url
    #[clap(value_parser = parse_url)]
    url: String,
    /// http 请求的 body
    #[clap(value_parser = parse_kv_pair)]
    body: Vec<KvPair>
}

#[derive(Parser, Debug, Clone)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

/// A naive httpie implementation with Rust, can you imagine how easy is it?
#[derive(Parser, Debug, Clone)]
#[command(version = "1.0", about = None, author = "yi.hu")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand
}

// 校验 url 的合法性
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!("Failed to parse {}", s);
        Ok(
            Self { k: (split.next().ok_or_else(err)?).to_string(), v: (split.next().ok_or_else(err)?).to_string() }
        )
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let res = client.get(&args.url).send().await?;
    println!("{:?}", res.text().await?);
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let json_body = json!(body).to_string();
    let res = client.post(&args.url).body(json_body).send().await?;
    println!("{:?}", res.text().await?);
    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    let client = Client::new();
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}
