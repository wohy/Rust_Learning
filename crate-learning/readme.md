# The Rust community’s crate registry
crates.io
类似于 npm

cargo add xxxx 下载外部包

通过 use 引入 作用域中

# crate 初探 --- rand 猜数字实现

# crate
我们有了一个只包含 src/main.rs 的包，意味着它只含有一个名为 my-project 的二进制 crate。

如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个二进制的和一个库的，且名字都与包相同。

通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。

# 路径
## 相对路径
相对某个模块下路径
```rs
use garden::vegetables::Asparagus
```

## 绝对路径
从 根节点开始
```rs
use crate::garden::vegetables::Asparagus
```

## 使用嵌套路径引入
```rs
use std::cmp::Ordering;
use std::io;
```
等同于
```rs
use std::{cmp::Ordering, io}
```

```rs
use std::io;
use std::io::Write;
```
等同于
```rs
use std::io::{self, Write};
```

还可以使用 * 引入目录下所有的公有项


# 创建一个库
通过执行 cargo new --lib garden， 来创建一个新的名为 garden 的库
