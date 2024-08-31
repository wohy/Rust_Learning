# 简单实现将 html 转化为 markdown

- 依赖安装
cargo add reqwest
cargo add html2md
cargo add tokio

# reqwest
`reqwest` 是 Rust 中一个功能丰富的 HTTP 客户端库，它提供了多种特性来扩展其功能。
reqwest 特性
可以查阅 https://docs.rs/crate/reqwest/0.12.7/features

## 异步请求 和 阻塞请求
The reqwest::Client is asynchronous. For applications wishing to only make a few HTTP requests, 
the reqwest::blocking API may be more convenient.
这个说明是关于 `reqwest` 提供了两种不同的 API 来发起 HTTP 请求：

1. **异步 API (`reqwest::Client`)**：
   - `reqwest::Client` 是一个异步的 HTTP 客户端，用于在 Rust 中执行异步的 HTTP 请求。
   - 异步 API 允许你在等待网络响应时不阻塞当前线程，可以提高应用程序的性能，特别是在 I/O 密集型任务中。
   - 异步 API 需要使用 Rust 的异步编程特性，如 `.await` 和异步运行时（例如 `tokio` 或 `async-std`）。

2. **阻塞 API (`reqwest::blocking`)**：
   - `reqwest::blocking` 是 `reqwest` 库提供的同步 API，用于执行阻塞的 HTTP 请求。
   - 使用阻塞 API 时，当前线程会在等待响应时被阻塞，直到请求完成。
   - 阻塞 API 对于只执行少量 HTTP 请求的应用程序可能更方便，尤其是当你不需要处理复杂的异步逻辑时。

说明的意思是，如果你的应用程序只需要进行少量的 HTTP 请求，或者你希望简化代码逻辑，不想使用异步编程模型，那么使用 `reqwest` 的阻塞 API 可能是一个更好的选择。这样可以避免处理异步代码的复杂性，同时仍然能够有效地执行 HTTP 请求。

以下是使用 `reqwest` 阻塞 API 的一个简单示例：

```rust
use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    let url = "https://www.rust-lang.org/";
    let response = client.get(url).send().unwrap();

    println!("Response status: {}", response.status());
    println!("Response text: {}", response.text().unwrap());
}
```

在这个示例中，我们使用 `reqwest::blocking::Client` 创建了一个同步的 HTTP 客户端，并使用 `send()` 方法发送了一个 GET 请求。由于这是一个阻塞调用，程序会在等待响应时暂停执行，直到请求完成。这种方法简单直接，适合简单的 HTTP 请求场景。

## 主要 features
以下是一些常用的 `reqwest` 特性及其在 `Cargo.toml` 中的设置方法：

1. **json**：
   - 用于支持 JSON 请求和响应的自动序列化和反序列化。
   - 依赖：`reqwest/json`。

2. **blocking**：
   - 用于启用同步 API。
   - 依赖：`reqwest/blocking`。

3. **native-tls**：
   - 使用 Rust 原生的 TLS 支持。
   - 依赖：`reqwest/native-tls`。

4. **rustls-tls**：
   - 使用 `rustls` 库作为 TLS 后端。
   - 依赖：`reqwest/rustls-tls`。

5. **native-tls-vendored**：
   - 使用 vendored 的（捆绑的）版本 Native TLS。
   - 依赖：`reqwest/native-tls-vendored`。

6. **rustls-tls-vendored**：
   - 使用 vendored 的版本 `rustls`。
   - 依赖：`reqwest/rustls-tls-vendored`。

7. **socks**：
   - 支持 SOCKS5 代理。
   - 依赖：`reqwest/socks`。

8. **gzip**：
   - 支持自动处理 gzip 压缩。
   - 依赖：`reqwest/gzip`。

9. **brotli**：
   - 支持自动处理 Brotli 压缩。
   - 依赖：`reqwest/brotli`。

10. **deflate**：
    - 支持自动处理 deflate 压缩。
    - 依赖：`reqwest/deflate`。

11. **trust-dns**：
    - 使用 `trust-dns` 库进行 DNS 解析。
    - 依赖：`reqwest/trust-dns`。

在 `Cargo.toml` 中设置特性的示例：

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "blocking"] }

# 如果需要使用特定的 TLS 特性，可以选择其中一个
# reqwest = { version = "0.11", features = ["native-tls"] }
# reqwest = { version = "0.11", features = ["rustls-tls"] }
# reqwest = { version = "0.11", features = ["native-tls-vendored"] }
# reqwest = { version = "0.11", features = ["rustls-tls-vendored"] }
```

请注意，某些特性可能是互斥的，例如，TLS 特性中，你只能选择 `native-tls` 或 `rustls-tls` 之一。另外，如果你选择了 vendored 的 TLS 版本，就不需要额外的系统依赖。

在使用这些特性时，请确保阅读 `reqwest` 的官方文档，了解它们具体的用途和如何在你的代码中使用它们。例如，如果你需要在应用程序中处理 JSON 数据，你可以使用 `json` 特性，然后在你的异步函数中使用 `.json().await` 方法来自动解析响应体为 JSON。




# tokio
`tokio` 是 Rust 中一个非常流行的异步运行时，它提供了多种特性来扩展其功能。

## 使用 tokio 特性

https://docs.rs/crate/tokio/latest/features#macros

如果正在使用 Tokio 运行时，你可以使用 tokio::main 宏来创建一个异步的 main 函数。需要在 Cargo.toml 中添加 Tokio 作为依赖项，并确保启用相应的特性：
Cargo.toml 设置：
tokio = { version = "1.39.3", features = ["full"] }
不确定 对应 features 建议直接使用 full

## 其他常用 features
以下是一些常用的 `tokio` 特性及其在 `Cargo.toml` 中的设置方法：

1. **full**：
   - 启用所有 Tokio 的组件，包括 IO、TCP、UDP、信号、时间器等。

2. **macros**：
   - 启用宏支持，如 `#[tokio::main]` 宏。

3. **rt**：
   - 启用 Tokio 的多线程运行时。

4. **rt-multi-thread**：
   - 启用多线程运行时。

5. **io-driver**：
   - 启用 IO 驱动器。

6. **tcp**：
   - 启用 TCP 相关的功能。

7. **udp**：
   - 启用 UDP 相关的功能。

8. **signal**：
   - 启用对 Unix 信号的处理。

9. **sync**：
   - 启用同步原语，如互斥锁、信号量等。

10. **time**：
    - 启用时间相关的功能，如延迟、超时等。

11. **fs**：
    - 启用文件系统相关的异步操作。

12. **process**：
    - 启用进程相关的异步操作。

13. **net**：
    - 启用网络相关的功能，包括 TCP 和 UDP。

14. **stream**：
    - 启用流相关的功能。

15. **test-util**：
    - 启用测试工具。

在 `Cargo.toml` 中设置特性的示例：

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }

# 或者选择性启用特定特性
# tokio = { version = "1", features = ["macros", "rt", "io-driver", "tcp", "sync"] }
```

请注意，`tokio` 的版本号应与你使用的版本相匹配。`"1"` 是截至知识更新日期的最新主要版本。

使用 `tokio` 特性的示例：

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Hello from Tokio!");
    
    // 使用 time 特性来创建一个延迟
    sleep(Duration::from_secs(1)).await;
    
    println!("Tokio delay completed!");
}
```

在这个示例中，我们使用了 `#[tokio::main]` 宏来创建一个异步的 `main` 函数，它将使用 Tokio 运行时。我们还使用了 `sleep` 函数来创建一个异步延迟。

要使用 `tokio` 的其他特性，如文件系统操作或网络编程，你需要根据需要启用相应的特性，并使用它们提供的 API。例如，如果你需要进行 TCP 通信，你可以启用 `tcp` 特性，并使用 `tokio::net::TcpStream`。

确保阅读 `tokio` 的官方文档，了解如何在你的应用程序中使用这些特性。随着 `tokio` 的发展，可能会有新的特性和改进，因此文档是了解最新信息的最佳资源。



# unwrap() 函数的作用
在 Rust 中，`unwrap()` 是 `Result` 和 `Option` 类型的一个方法，用于处理可能的错误或缺失的值。以下是 `unwrap()` 的作用和使用场景：

1. **结果提取**：
   - 对于 `Result` 类型，`unwrap()` 方法尝试提取 `Ok` 变体中的值。如果 `Result` 是 `Err`，程序将 panic 并显示错误信息。

2. **值获取**：
   - 对于 `Option` 类型，`unwrap()` 方法尝试获取 `Some` 变体中的值。如果 `Option` 是 `None`，程序同样将 panic 并显示错误信息。

3. **简化代码**：
   - `unwrap()` 可以简化错误处理代码，特别是在你确信某个位置的值是有效的时候。

4. **调试和开发**：
   - 在开发和调试过程中，使用 `unwrap()` 可以快速获取值，而不必编写完整的错误处理逻辑。

5. **错误传播**：
   - 当你希望将错误传播给调用者，而不是在当前层级处理时，`unwrap()` 可以触发 panic，由上层调用者通过 `match` 或其他方式处理。

6. **默认行为**：
   - 在某些情况下，`unwrap()` 可以用作默认行为，当没有其他合适的错误处理方式时使用。

尽管 `unwrap()` 在某些情况下很有用，但它不是错误处理的最佳实践，因为它会导致程序在遇到错误时 panic。在生产代码中，推荐使用更健壮的错误处理方式，如 `match` 语句、`if let`、`unwrap_or`、`expect` 等，这些方法可以提供更优雅的错误处理逻辑。

使用 `unwrap()` 时，你应该非常清楚代码的上下文，并确信 `Result` 或 `Option` 的值是有效的，或者你愿意让程序在无效值的情况下崩溃。在大多数情况下，使用 `unwrap()` 应该谨慎，并考虑到潜在的错误处理需求。




# 优化
使用 clap 解析通过命令行传入 目标网站 html，和目标输出的 md 文件