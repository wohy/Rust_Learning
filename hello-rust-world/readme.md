# Cargo.toml
cargo 特有的项目数据描述文件。它存储了项目的所有元配置信息，如果 Rust 开发者希望 Rust 项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建 Cargo.toml

## package
对当前项目信息的描述
```rs
[package]
name = "hello-rust-world"
version = "0.1.0"
edition = "2021"
```
- name：项目名称
- version： 项目当前版本
- edition：定义了我们使用的 Rust 大版本。``Rust edition 2021`` 的版本


## dependencies
申明当前项目的依赖
```rs
[dependencies]
rand = "0.3"
hammer = { version = "0.5.0"}
color = { git = "https://github.com/bjz/color-rs" }
geometry = { path = "crates/geometry" }
```
主要通过各种依赖段落来描述该项目的各种依赖项：
- 基于 ``Rust`` 官方仓库 ``crates.io``，通过版本说明来描述
- 基于项目源代码的 ``git`` 仓库地址，通过 ``URL`` 来描述
- 基于本地项目的绝对路径或者相对路径，通过类 ``Unix 模式``的路径来描述


# Cargo.lock
Cargo.lock 文件是 cargo 工具根据同一项目的 toml 文件生成的项目依赖详细清单，因此我们一般不用修改它
如果项目是一个 可运行项目 则需要上传 Cargo.lock 文件
如果只是一个依赖文件，则可以将文件添加到 .gitignore 中

# 项目运行
``$ cargo run``默认是运行 debug 模式
``$ cargo run --release`` 生产发布模式下的运行

## 热启动 
### 使用 cargo watch
cargo watch 是一个 Rust 工具，它可以监视你的源代码文件的更改，并在检测到更改时自动重新编译项目。首先，你需要安装 cargo-watch 命令：
```sh
cargo install cargo-watch
```
然后，在项目根目录下运行以下命令：
```sh
cargo watch -x run
```
这将监视你的项目文件，并在文件更改时执行 cargo run 命令。