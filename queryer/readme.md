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

# 使用模式匹配方式实现 类型转换 convert.rs
sqlparser 解析 sql 生成 ast
主要 结构 有一个 body 属性内包含了各种操作语句 Select 、 order_by 、 limit 、 offset 、 fetch
即可匹配出 body 的各个语句，通过 实现 From 和 TryFrom trait 将语句类型转化为符合 polars 读取出的 datasource 的 AST 类型
```rs
Ok(
    [
        Query(
            Query {
                with: None,
                body: Select(
                    Select {
                        distinct: false,
                        top: None,
                        projection: [
                            ExprWithAlias {
                                expr: Identifier(
                                    Ident {
                                        value: "a",
                                        quote_style: None,
                                    },
                                ),
                                alias: Ident {
                                    value: "a1",
                                    quote_style: None,
                                },
                            },
                            UnnamedExpr(
                                Identifier(
                                    Ident {
                                        value: "b",
                                        quote_style: None,
                                    },
                                ),
                            ),
                            UnnamedExpr(
                                Value(
                                    Number(
                                        "123",
                                        false,
                                    ),
                                ),
                            ),
                            UnnamedExpr(
                                Function(
                                    Function {
                                        name: ObjectName(
                                            [
                                                Ident {
                                                    value: "myfunc",
                                                    quote_style: None,
                                                },
                                            ],
                                        ),
                                        args: [
                                            Unnamed(
                                                Identifier(
                                                    Ident {
                                                        value: "b",
                                                        quote_style: None,
                                                    },
                                                ),
                                            ),
                                        ],
                                        over: None,
                                        distinct: false,
                                    },
                                ),
                            ),
                            Wildcard,
                        ],
                        from: [
                            TableWithJoins {
                                relation: Table {
                                    name: ObjectName(
                                        [
                                            Ident {
                                                value: "data_source",
                                                quote_style: None,
                                            },
                                        ],
                                    ),
                                    alias: None,
                                    args: [],
                                    with_hints: [],
                                },
                                joins: [],
                            },
                        ],
                        lateral_views: [],
                        selection: Some(
                            BinaryOp {
                                left: BinaryOp {
                                    left: BinaryOp {
                                        left: Identifier(
                                            Ident {
                                                value: "a",
                                                quote_style: None,
                                            },
                                        ),
                                        op: Gt,
                                        right: Identifier(
                                            Ident {
                                                value: "b",
                                                quote_style: None,
                                            },
                                        ),
                                    },
                                    op: And,
                                    right: BinaryOp {
                                        left: Identifier(
                                            Ident {
                                                value: "b",
                                                quote_style: None,
                                            },
                                        ),
                                        op: Lt,
                                        right: Value(
                                            Number(
                                                "100",
                                                false,
                                            ),
                                        ),
                                    },
                                },
                                op: And,
                                right: Between {
                                    expr: Identifier(
                                        Ident {
                                            value: "c",
                                            quote_style: None,
                                        },
                                    ),
                                    negated: false,
                                    low: Value(
                                        Number(
                                            "10",
                                            false,
                                        ),
                                    ),
                                    high: Value(
                                        Number(
                                            "20",
                                            false,
                                        ),
                                    ),
                                },
                            },
                        ),
                        group_by: [],
                        cluster_by: [],
                        distribute_by: [],
                        sort_by: [],
                        having: None,
                    },
                ),
                order_by: [
                    OrderByExpr {
                        expr: Identifier(
                            Ident {
                                value: "a",
                                quote_style: None,
                            },
                        ),
                        asc: Some(
                            false,
                        ),
                        nulls_first: None,
                    },
                    OrderByExpr {
                        expr: Identifier(
                            Ident {
                                value: "b",
                                quote_style: None,
                            },
                        ),
                        asc: None,
                        nulls_first: None,
                    },
                ],
                limit: Some(
                    Value(
                        Number(
                            "50",
                            false,
                        ),
                    ),
                ),
                offset: Some(
                    Offset {
                        value: Value(
                            Number(
                                "10",
                                false,
                            ),
                        ),
                        rows: None,
                    },
                ),
                fetch: None,
            },
        ),
    ],
)
```

# 从文件或者url中获取源数据 fetcher.rs

# 将源数据（csv 数据）读取为 DataFrame ，这里为 DataSet ， 自定义的 DataFrame 代理，面向工具使用方。 loader.ts

# lib.rs 中定义好 DataSet，定义好 query 函数，传入 sql 字符串输出，sql 执行后得到的内容