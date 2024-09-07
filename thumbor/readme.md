# 实现一个类似于 python 中 Thumbor 的图片服务器， Thumbor 被广泛运用于各种需要动态调整图片尺寸的场合里
https://thumbor.readthedocs.io/en/latest/
例如：
- ```http://<thumbor-server>/300x200/smart/thumbor.readthedocs.io/en/latest/_images/logo-thumbor.png```

Thumbor 可以对这个图片最后的 URL 使用 smart crop 剪切，并调整大小为 300x200 的尺寸输出，用户访问这个 URL 会得到一个 300x200 大小的缩略图。

本次实现就是实现其核心功能，对图片进行动态转换

# 问题： 将想要做的操作都体现在 url 上，可能使 url 会非常长
- 解决：
使用 **Protocol Buffers（简称：ProtoBuf）**，一种开源跨平台的序列化数据结构的协议。其对于存储资料或在网络上进行通信的程序是很有用的。这个方法包含一个接口描述语言，描述一些数据结构，并提供程序工具根据这些描述产生代码，这些代码将用来生成或解析代表这些数据结构的字节流
当用 ProtoBuf 生成一个 image spec 后，可以将其转化为字节流，字节流无法放在 url 上，可以使用 base64 转码

# 使用 protobuf
在 Rust 中使用 Protocol Buffers (protobuf) 涉及几个步骤，包括定义 `.proto` 文件、生成 Rust 代码、添加依赖项到你的 `Cargo.toml` 文件，以及在你的 Rust 项目中使用生成的代码。以下是详细的步骤：

### 1. 定义 `.proto` 文件

首先，你需要定义一个 `.proto` 文件来指定你的数据结构。例如，创建一个名为 `message.proto` 的文件：

```protobuf
syntax = "proto3";

package mypackage;

message MyMessage {
  string field1 = 1;
  int32 field2 = 2;
}
```

### 2. 安装 `protoc` 编译器

确保你已经安装了 Protocol Buffers 编译器 `protoc`。你可以从 [Protocol Buffers GitHub 仓库](https://github.com/protocolbuffers/protobuf/releases) 下载预编译的二进制文件或使用包管理器安装。

### 3. 生成 Rust 代码

使用 `protoc` 编译器和 `protoc-gen-rust` 插件来生成 Rust 代码。首先，你需要安装 `protoc-gen-rust` 插件：

```sh
cargo install protobuf-codegen
```

然后，使用以下命令生成 Rust 代码：

```sh
protoc --rust_out=. message.proto
```

这将生成一个 `message.rs` 文件，其中包含 Rust 代码，你可以在你的项目中使用。

### 4. 添加依赖项到 `Cargo.toml`

在你的 `Cargo.toml` 文件中添加 `protobuf` 依赖项：

```toml
[dependencies]
protobuf = "2.25.0"
```

确保使用最新版本的 `protobuf` crate。

### 5. 使用生成的代码

在你的 Rust 项目中，你可以使用生成的代码来序列化和反序列化消息：

```rust
extern crate protobuf;

use mypackage::MyMessage;

fn main() {
    let mut message = MyMessage {
        field1: "Hello".to_owned(),
        field2: 42,
    };

    // 序列化
    let serialized = message.write_to_bytes().unwrap();
    println!("Serialized message: {:?}", serialized);

    // 反序列化
    let deserialized = MyMessage::read_from_bytes(&serialized).unwrap();
    println!("Deserialized message: {:?}", deserialized);
}
```

### 6. 构建和运行

构建你的项目：

```sh
cargo build
```

运行你的项目：

```sh
cargo run
```

### 注意事项

- 确保 `.proto` 文件的路径正确，并且 `protoc` 命令可以找到它。
- 如果你的项目中有多个 `.proto` 文件或复杂的依赖关系，你可能需要更复杂的构建脚本。
- `protoc-gen-rust` 插件可能需要更新以匹配 `protobuf` crate 的版本。

通过这些步骤，你可以在 Rust 项目中使用 Protocol Buffers 来有效地序列化和反序列化数据。



# 使用 abi.proto 配置
1. 创建 build.rs 可以在编译 cargo 项目时，做额外的编译处理
这里我们使用 prost_build 把 abi.proto 编译到 src/pb 目录下
2.  $ mkdir src/pb
    $ cargo build
    pb 下会自动生成一个 abi.rs 文件
3. pb 下创建 mod.rs 将 abi 作为 一个 crates

# main 函数使用 axum 快速搭建一个服务
配置好对应的 route 为， 域名（localhost:3000） + /image（路径） + /:spec (参数spec) + /:url (参数url)
spec 将要对图片做的处理, 即一个 protobuf 字符串
url 图片的 url
监听该路由上的 get 请求，实现一个 handler 方法(generate) 对该请求进行处理

```rs
mod pb;
use pb::*;
use axum::{ extract::Path, handler::get, http::StatusCode, Router};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::convert::TryInto;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String
}

async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec.as_str().try_into().map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/image/:spec/:url", get(generate));
    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
```

# 服务搭建好后
cargo run 启动服务
服务启动后 浏览器地址栏输入：
http://localhost:3000/image/CgoKCAjYBBCgBiADCgY6BAgUEBQKBDICCAM/https%3A%2F%2Fimages.pexels.com%2Fphotos%2F2470905%2Fpexels%2Dphoto%2D2470905.jpeg%3Fauto%3Dcompress%26cs%3Dtinysrgb%26dpr%3D2%26h%3D750%26w%3D1260

get 请求该服务，将输出 decode后的 图片url 以及 想要对图片进行的处理 spec 对象:
```
url: https://images.pexels.com/photos/2470905/pexels-photo-2470905.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=750&w=1260
spec: ImageSpec {
    specs: [
        Spec {
            data: Some(
                Resize(
                    Resize {
                        width: 600,
                        height: 800,
                        rtype: Normal,
                        filter: CatmullRom,
                    },
                ),
            ),
        },
        Spec {
            data: Some(
                Watermark(
                    Watermark {
                        x: 20,
                        y: 20,
                    },
                ),
            ),
        },
        Spec {
            data: Some(
                Filter(
                    Filter {
                        filter: Marine,
                    },
                ),
            ),
        },
    ],
}
```
到这里服务处理好了， url 及参数解析成功,之后则通过 ImageSpec 对 图片进行处理

# 通过 ImageSpec 对 图片进行处理
## 实现 retrieve_image 方法，用于原图片缓存
这里会使用 LRU cache 将原图片缓存起来
## 实现图片处理引擎 engine
## cargo build --release 编译
## RUST_LOG=info target/release/thumbor 运行
成功了！这就是我们的 Thumbor 服务根据用户的请求缩小到 500x800、加了水印和 Marine 滤镜后的效果。
第一次访问 通过 reqwest 请求原图片，后续直接冲 LRU 缓存中读取