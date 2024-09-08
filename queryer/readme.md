# 实现一个不需要 spark 可以直接在 shell 中执行 sql 对 csv 数据源进行查询
cargo new queryer --lib 创建一个库

# 使用到的工具
- sql 解析器
使用 sqlparser-rs 将 sql 解析成一个 AST （抽象语法树）
- 将 csv 转化成 DataFrame （一个矩阵数据结构）
使用 polars 完成 csv 数据源到 DataFrame 的加载和进行后续的处理操作

# 宏编程
实际是将一棵语法树转化为另一个语法树，实际即为一个数据结构转化为另一个数据结构。
主要流程就是实现若干个 From 和 TryFrom 的过程（From 和 TryFrom 是 Rust 标准库中用于类型转换的特质（trait））

# 添加例子 熟悉 sqlparser 功能
在 cargo.toml 中，如下配置，example，编译器会自动在 examples 文件夹下寻找 dialect.rs 文件
```rs
[[example]]
name = "dialect"
```

实现 dialect.rs 文件后
cargo run --example dialect 执行例子

同样的方式可以 尝试使用一下 polars

# 实现自己的 sql 语法，让 datasource 可以是一个 url，即可以接受传入一个 csv 文件路径 

