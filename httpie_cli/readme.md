# 实现一个 类似于 curl 的命令行工具，实现 python 中的 HTTPie 工具
本次实现 建议 post get 请求

## 使用到的 crates
- clap 命令行解析工具
- reqwest 请求处理库  features json 支持将body
- colored 命令终端美化、高亮
- anyhow 错误处理
- jsonxf 格式化 JSON
- mime 处理 mime 类型
- tokio 处理异步

## 几个使用到的 宏
- ```#[derive(Parser, Debug, Clone)]```
#[derive(Parser, Debug, Clone)] 宏，为 枚举或者结构体 自动派生（derive）Parser, Debug, Clone 特性（traits）的实现

- ``` #[clap(value_parser = parse_url)]```
自定义解析逻辑：允许你为 url 字段提供一个自定义的解析函数，而不是使用 clap 默认的字符串解析逻辑。

验证和转换：在命令行参数被解析时，parse_url 函数将被调用来验证 URL 的格式，并将其转换为程序内部使用的格式。

错误处理：如果提供的 URL 无法被 parse_url 函数解析，你可以在该函数中返回一个错误，clap 将使用这个错误信息来通知用户。

- ```#[tokio::main]```
允许将 main 定义 async 函数

- ```#[cfg(test)]```
函数仅在 test 模式下执行
一般用于单元测试
使用 cargo test 执行

## 开发阶段
执行
$ cargo build --quiet && target/debug/httpie post httpbin.org/post a=1 b=2
或
$ cargo run -- post httpbin.org/post a=1 b=2
检测
控制台输出：
Opts { subcmd: Post(Post { url: "httpbin.org/post", body: ["a=1", "b=2"] }) }
证明 url parse 等方法调用成功，Opts 成功构建



# tokei src/main.rs
tokei 代码行数统计工具
全局安装 cargo install tokei
执行
tokei /path/to/directory
或者直接在当前文件下执行 tokei

# 命令行工具生效
1. cargo build --release
构建出生产包
2. 将生产包 copy 到在 $PATH 中的一个文件夹中 例如 /Users/yihu/
3. $ open -e ~/.zshrc 打开 .zshrc 文件夹，添加 export PATH="/Users/yihu/release:$PATH" 保存
4. $ source ~/.zshrc 重新加载配置
5. $ httpie_cli 即可执行
6. $ httpie_cli post https://httpbin.org/post greeting=hola name=yi.hu 即可检验成果
## 或者移动好文件夹后直接执行以下命令即可yih
/Users/yihu/release/httpie_cli post https://httpbin.org/post greeting=hola name=yi.hu

# 优化
尝试使用 syntect 库 来优化语法高亮，syntect更加强大