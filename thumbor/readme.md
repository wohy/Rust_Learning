# 实现一个类似于 python 中 Thumbor 的图片服务器， Thumbor 被广泛运用于各种需要动态调整图片尺寸的场合里
https://thumbor.readthedocs.io/en/latest/
例如：
- ```http://<thumbor-server>/300x200/smart/thumbor.readthedocs.io/en/latest/_images/logo-thumbor.png```

Thumbor 可以对这个图片最后的 URL 使用 smart crop 剪切，并调整大小为 300x200 的尺寸输出，用户访问这个 URL 会得到一个 300x200 大小的缩略图。

本次实现就是实现其核心功能，对图片进行动态转换


# 问题1 将想要做的操作都体现在 url 上，可能使 url 会非常长
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