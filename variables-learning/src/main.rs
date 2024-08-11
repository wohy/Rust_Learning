fn _variable_learning() {
    // let a = 1;
    // a = 3; //error[E0384]:  cannot assign twice to immutable variable `a`
    let mut a = 1;
    println!("a初始值为{}", a);
    a = 3;
    println!("a修改后值为{}", a);

    // 不可被使用变量声明
    let _b = 2;

    // 解构赋值
    let (c, mut d) = (4, 5);
    // 4, 5
    println!("{}, {}", c, d);
    d = 4;
    // 4
    println!("{}", d);

    // assert_eq! 是一个宏，用于断言两个值是否相等。如果两个值相等，测试会继续执行；
    // 如果不相等，则测试会失败，并在控制台打印出两个值的预期和实际值，然后程序会 panic。
    assert_eq!(c, d);
    println!("断言通过 c 等于 d");

    // 结构体 也可通过解构给内容赋值
    struct FirstStruct {
        e: i32,
        f: i32,
        j: i32,
        k: i32
    }
    let (e, f, g, h, i);
    g = 3;
    // .. 表示匹配中间部分 多位，  _ 表示 匹配某一位
    // 执行后 h 赋值为 4， i 赋值为 5
    [h, .., i, _, _] = [4, 1, 2, 3, 6, 5, 7, 8];
    // 执行后 e 值为 1
    FirstStruct { e, f, .. } = FirstStruct { e: 1, f: 2, j: 3, k: 4 };
    assert_eq!([e, f, g, h, i], [1, 2, 3, 4, 5]);
    println!("断言成功e, f, g, h, i的值分别对应1, 2, 3, 4, 5");

    // 常量声明
    // 常量不允许改变 且声明是需要指明类型。如果全局作用域下定义，可在全局使用
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS 的值为 {}", MAX_POINTS);


    // 变量遮蔽
    // 与  mut 不同，遮蔽将生成一个新的 变量，只是变量名相同而已，进行了一次新的地址分配。而 mut 是修改同一地址上的值
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;
    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        // 12
        println!("The value of x in the inner scope is: {}", x);
    }
    // 6
    println!("The value of x is: {}", x);

    // rust 对类型要求严格 无法将原本声明并被赋值为某个类型的 mut 变量，修改为另一个类型
    // 报错 error[E0308]: mismatched types
    // expected `&str`, found `usize`
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // 合法
    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len();
    print!("{}", spaces);

    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;
    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);
    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));
    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];
    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);


    // 所有权转移
    let parent = String::from("123");
    let child = parent;
    // // 报错 error[E0382]: borrow of moved value: `parent`
    // println!("{}", parent);
    // 123
    println!("{}", child);

    // 使用深拷贝 赋值
    let parent2 = String::from("456");
    let child2 = parent2.clone();
    println!("parent2: {}, chilid2: {}", parent2, child2);


    // String / str
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let hello1 = &s[..5];
    let world1 = &s[6..];
    let all = &s[..];
    let s_str = s.as_str();
    println!("hello:{}, world:{}, hello1:{}, world1:{}, all:{}, s_str:{}", hello, world, hello1, world1, all, s_str)

}

// 引用、解引用
fn str_add(a: &mut String, b: &String) -> String {
    println!("解引用，a: {} b: {}", *a, *b);
    a.push_str(b);
    a.clone()
}

// 元组学习
fn tuple_learning() {
    let tuples: (i32, f64, u8) = (12, 0.62, 9);
    let (x, y, z) =  tuples;
    // x is 12, y is 0.62, z is 9
    println!("x is {}, y is {}, z is {}", x, y, z);
    println!("the second one is {}", tuples.1);
}
fn tuple_to_use(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}


// 结构体学习

fn struct_learning() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        email: String::from("yi.hu@example.com"),
        username: String::from("yi.hu"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        username: String::from("xiaohuang"),
        email: String::from("123@qq.com"),
        ..user1
    };
    println!("用户1，用户名：{}, 账号：{}, 登陆次数：{}, 是否有效用户：{}", user1.username, user1.email, user1.sign_in_count, user1.active);
    println!("用户2，用户名：{}, 账号：{}", user2.username, user2.email);
}





fn main() {
    // variable_learning();
    let mut a = String::from("Hello");
    let b = String::from(" Rust");
    let res = str_add(&mut a, &b);
    // Hello Rust
    println!("{}", res);

    tuple_learning();
    let (s, len) = tuple_to_use(String::from("12345678"));
    println!("the length of '{}' is {}", s, len);


    struct_learning()
}
