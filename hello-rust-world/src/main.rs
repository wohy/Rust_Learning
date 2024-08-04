fn greet_to_rust() {
    let chinese: &str = "你好！rust 世界";
    let english: &str = "Hello! The world of Rust";
    let language: [&str; 2] = [chinese, english];

    // 实际 2021 版本 language.iter() 可以直接写为 language ， for...in... 会隐式的将 language 数组转化为迭代器
    for lang in language.iter() {
        // {} 为占位符，!表示为 宏操作符，println 表示为一种特殊函数
        println!("{}", &lang)
    }
}

fn hight_level_greet() {
    let persons_str:&str = "\
    name, age
    xiaoming, 18
    xiaohong, 20
    xiaofang, 25
    xiaohuang, 26
    error, data
    ";
    let all_lines: std::str::Lines = persons_str.lines();

    for (i, line) in all_lines.enumerate() {
        if i == 0 || line.trim().len() == 0 {
            continue;
        }
        
        // 处理后的的 fields 为 ["xiaoming", "18"], 
        // Vec<_> 旨在表示 fields 变量是一个向量，表示一个向量（Vector）的类型，其中 _ 是一个类型占位符，用于指明向量中元素的类型，但在这种情况下，它被用作一个泛型参数，而没有指定具体的类型
        // 具体类型尚未指定，需要编译器自动去推断
        // 这里使用了 Rust 中的闭包语法来定义 map 方法的行为。
        // 闭包 |field| field.trim() 是一个匿名函数，它接受一个参数 field 并返回调用 trim 方法后的结果，field 即为 line.split(',') 得到的每一项
        // |field| field.trim() 类似于 js 中箭头函数的 line.split(',').map((field) => field.trim))
        let fields: Vec<_> = line.split(',').map(|field| field.trim()).collect();

        // debug 打印 ，紧跟其后的输出（打印）只在 debug 模式下生效。 release 模式下将不执行
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            // {:?} 占位表示 未知类型、泛型的值
            eprintln!("debug: {:?} -> {:?}",line, fields);
        }
        let name: &str = fields[0];

    
        // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果可以成功转换，则把 f32 值赋给 age 变量
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 age 的值：
        //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
        //   2）同时 if let 还会做一次解构匹配，通过 Ok(age) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 age
        //
        if let Ok(age) = fields[1].parse::<f32>(){
            println!("Hello, my name is {}, I'm {} years old", name, age);
        }
    }
}

fn main() {
    greet_to_rust();
    hight_level_greet();
}
