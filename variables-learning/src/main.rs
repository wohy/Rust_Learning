use std::collections::btree_map::Keys;

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
        k: i32,
    }
    let (e, f, g, h, i);
    g = 3;
    // .. 表示匹配中间部分 多位，  _ 表示 匹配某一位
    // 执行后 h 赋值为 4， i 赋值为 5
    [h, .., i, _, _] = [4, 1, 2, 3, 6, 5, 7, 8];
    // 执行后 e 值为 1
    FirstStruct { e, f, .. } = FirstStruct {
        e: 1,
        f: 2,
        j: 3,
        k: 4,
    };
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
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );
    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));
    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, 42.0_f32];
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
    println!(
        "hello:{}, world:{}, hello1:{}, world1:{}, all:{}, s_str:{}",
        hello, world, hello1, world1, all, s_str
    )
}

// 引用、解引用
fn str_add(a: &mut String, b: &String) -> String {
    println!("解引用，a: {} b: {}", *a, *b);
    a.push_str(b);
    a.clone()
}
fn first_word(the_string: &String) -> &str {
    // split_whitespace 将返回一个迭代器，迭代器 通过 next 访问第一个元素。通过 nth 访问某个元素 返回一个 Option<&str>
    let mut all_words: std::str::SplitWhitespace = the_string.split_whitespace();

    if let Some(first) = all_words.next() {
        return first;
    } else {
        return "";
    }

    // match all_words.nth(0) {
    //     Some(word) => return word,
    //     None => return ""
    // }
}

// 元组学习
fn tuple_learning() {
    let tuples: (i32, f64, u8) = (12, 0.62, 9);
    let (x, y, z) = tuples;
    // x is 12, y is 0.62, z is 9
    println!("x is {}, y is {}, z is {}", x, y, z);
    println!("the second one is {}", tuples.1);
}
fn tuple_to_use(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

// 结构体、Vector(可变数组)、方法学习
fn _struct_learning() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    impl User {
        fn get_info(&self, field: &str) {
            match field {
                "active" => {
                    let is_active;
                    if self.active {
                        is_active = "有效"
                    } else {
                        is_active = "无效"
                    }
                    println!("该用户是{}", is_active)
                },
                "username" => {
                    println!("该用户名为{}", self.username)
                },
                "email" => {
                    println!("该用户邮箱为{}", self.email)
                },
                "sign_in_count" => {
                    println!("该用户登录了 {} 次",self.sign_in_count)
                }
                _ => println!("没有对应字段"),
            }
        }
        fn create_normal_user(name: &str, email: &str) -> Self {
            Self {
                active: true,
                username: name.to_string(),
                email: email.to_string(),
                sign_in_count: 1
            }
        }
        fn update_sign_count(&mut self) {
            self.sign_in_count += 1
        }
    }
    struct PhoneNumber {
        name: String,
        number: String,
    }

    let user1 = User {
        email: String::from("yi.hu@example.com"),
        username: String::from("yi.hu"),
        active: true,
        sign_in_count: 1
    };
    let user2 = User {
        username: String::from("xiaohuang"),
        email: String::from("123@qq.com"),
        ..user1
    };

    println!(
        "用户1，用户名：{}, 账号：{}, 登陆次数：{}, 是否有效用户：{}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );
    println!("用户2，用户名：{}, 账号：{}", user2.username, user2.email);
    user1.get_info("username");
    user1.get_info("active");

    let user3 = User::create_normal_user("xiaohu", "123@qq.com");
    user3.get_info("username");
    user3.get_info("email");

    let mut user4 = User::create_normal_user("xiaosi", "1234@qq.com");
    user4.update_sign_count();
    user4.get_info("sign_in_count");


    // 可变结构体 只能将结构体设置为可变，无法将结构体中某个字段设置为可变
    // let mut user1_phone_number = PhoneNumber {
    //     name: String::from("yi.hu"),
    //     country_code: "86",
    //     number: String::from("12345678901")
    // };

    let user1_phone_number = PhoneNumber {
        name: String::from("yi.hu"),
        number: String::from("12345678901"),
    };
    let user2_phone_number = PhoneNumber {
        name: String::from("xiao.ming"),
        number: String::from("09876543211"),
    };

    let mut phone_number_vec: Vec<PhoneNumber> = Vec::new();
    // let mut phone_number_vec2:Vec<PhoneNumber> = vec![user1_phone_number, user2_phone_number];
    phone_number_vec.push(user1_phone_number);
    phone_number_vec.push(user2_phone_number);

    fn get_phone_number_by_name(phone_number_collect: &[PhoneNumber], name: &String) -> String {
        for user_phone in phone_number_collect {
            // 判断语法
            if *name == user_phone.name {
                return user_phone.number.clone();
            }
        }
        "".to_string()
    }
    let user1_phone = get_phone_number_by_name(&phone_number_vec, &String::from("yi.hu"));
    println!("yi.hu 的电话为 {}", user1_phone);
}

// Vector(可变数组)学习
fn _vec_learning() {
    // 除如上创建 let mut phone_number_vec:Vec<PhoneNumber> = Vec::new() ，创建可变数组的方式外，还有其他方式创建 vec
    // 宏 方式创建
    let mut vec_1: Vec<i32> = vec![1, 2, 3];
    // 已知容量创建
    let mut vec_2: Vec<char> = Vec::with_capacity(10);
    // array 转化
    let mut _vec_3: Vec<i32> = [1, 2].to_vec();

    vec_2.extend(['a', 'b', 'c']);
    // 使用 get 获取元素，安全（有值的时候返回 Some(T)，无值的时候返回 None），但是较下标访问消耗更大。
    match vec_2.get(1) {
        Some(value) => println!("vec_2的第二个元素为: {}", value),
        None => println!("没有找到元素"),
    }
    // 获取长度和容量
    println!(
        "vec_2当前长度为：{}, 容量为：{}",
        vec_2.len(),
        vec_2.capacity()
    ); // 3 10
       // 扩容 追加容量
    vec_2.reserve(10);
    println!(
        "vec_2扩容后长度为：{}, 容量为：{}",
        vec_2.len(),
        vec_2.capacity()
    ); // 3 20

    for i in &mut vec_1 {
        *i += 10
    }
    println!("修改后 vec_1 的内容为 {:?}", vec_1); // [11, 12, 13]
                                                   // 插入元素
    vec_1.insert(1, 16);
    println!("修改后 vec_1 的内容为 {:?}", vec_1); // [11, 16, 12, 13]

    // 其他常用方法
    let mut v: Vec<i32> = vec![1, 2];
    assert!(!v.is_empty()); // 检查 v 是否为空

    v.insert(2, 3); // 在指定索引插入数据，索引值不能大于 v 的长度， v: [1, 2, 3]
    assert_eq!(v.remove(1), 2); // 移除指定位置的元素并返回, v: [1, 3]
    assert_eq!(v.pop(), Some(3)); // 删除并返回 v 尾部的元素，v: [1]
    assert_eq!(v.pop(), Some(1)); // v: []
    assert_eq!(v.pop(), None); // 记得 pop 方法返回的是 Option 枚举值
    v.clear(); // 清空 v, v: []

    let mut v1: Vec<i32> = [11, 22].to_vec(); // append 操作会导致 v1 清空数据，增加可变声明
    v.append(&mut v1); // 将 v1 中的所有元素附加到 v 中, v1: []
    v.truncate(1); // 截断到指定长度，多余的元素被删除, v: [11]
    v.retain(|x: &i32| *x > 10); // 保留满足条件的元素，即删除不满足条件的元素

    let mut v: Vec<i32> = vec![11, 22, 33, 44, 55];
    // 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
    let mut m: Vec<_> = v.drain(1..=3).collect();

    let v2: Vec<i32> = m.split_off(1); // 指定索引处切分成两个 vec, m: [22], v2: [33, 44]
    println!("v2 为 {:?}", v2)
}

// HashMap<K,V> 学习
fn hashmap_learning() {
    use std::collections::HashMap;

    let teams_list = vec![("中国队", 100), ("美国队", 10), ("日本队", 50)];

    let mut teams_map = HashMap::new();
    for team in &teams_list {
        // 若 team.0 为复杂类型 例如 String，不使用引用的话，所有权将转移给 teams_map。后续操作将无法访问
        teams_map.insert(team.0, team.1);
    }
    teams_map.insert("韩国队", 20);
    println!("{:?}", teams_map);

    fn get_score<'a>(the_map: &mut HashMap<&'a str, i32>, country: &'a str) -> i32 {
        // match the_map.get(country) {
        //     Some(&score) => {
        //         println!("{}得分为 {}", country, score);
        //         score // 返回对分数的引用的值
        //     }
        //     None => {
        //         let score = 0; // 插入前的默认分数
        //         the_map.insert(country, score); // 插入新条目
        //         println!("{}得分为 {}", country, score);
        //         score // 返回新插入的分数
        //     }
        // }
        // 使用 if let 简化。 match 只匹配一种特殊情况及其他情况。可使用 if let 简化
        if let Some(score) = the_map.get(country) {
            println!("{}得分为 {}", country, score);
            *score // 返回对分数的引用的值
        } else {
            let score = 0; // 插入前的默认分数
            the_map.insert(country, score); // 插入新条目
            println!("{}得分为 {}", country, score);
            score // 返回新插入的分数
        }
    }
    get_score(&mut teams_map, "英国队");

    // get_score 类似
    fn get_score_by_entry<'a>(the_map: &mut HashMap<&'a str, i32>, country: &'a str) {
        let the_score = the_map.entry(country).or_insert(0);
        println!("{}得分为 {}", country, the_score);
    }
    get_score_by_entry(&mut teams_map, "德国队");

    for (key, value) in &teams_map {
        println!("{}: {}", key, value);
    }
    println!("最终分数记录为：{:?}", teams_map);
}

// 枚举学习 if let、match 结合使用
fn _enum_learning() {
    // #[derive(Debug)] 是一个属性宏，它的作用是自动派生（derive）Debug trait 对结构体或枚举类型的实现。
    // Debug trait 允许你使用格式化宏（如 println!）以调试格式打印变量的值
    #[derive(Debug)]
    enum PokerSuit {
        Clubs(u8),
        Spades(u8),
        Diamonds(char),
        Hearts(char),
    }
    // 通过 :: 访问对应枚举实例
    // let clubs: PokerSuit = PokerSuit::Clubs;
    // let spades: PokerSuit = PokerSuit::Spades;
    // let diamonds: PokerSuit = PokerSuit::Diamonds;
    // let hearts: PokerSuit = PokerSuit::Hearts;
    // println!("获取到 PokerSuit 中枚举 Clubs、Spades、Diamonds、Hearts 的实例分别为 {:?}、{:?}、{:?}、{:?}", clubs, spades, diamonds, hearts);

    // 允许传入  u8 类型入参进行实例化
    // 同一个枚举类型下的不同成员还能持有不同的数据类型
    let clubs_2: PokerSuit = PokerSuit::Clubs(2);
    let spades_3: PokerSuit = PokerSuit::Spades(3);
    let diamonds_a: PokerSuit = PokerSuit::Diamonds('A');
    let hearts_k: PokerSuit = PokerSuit::Hearts('K');
    fn get_color(the_poker: &PokerSuit) {
        // match 与结构体的结合
        match the_poker {
            PokerSuit::Clubs(_) => println!("♣️"),
            PokerSuit::Spades(_) => println!("♠️"),
            PokerSuit::Diamonds(_) => println!("♦️"),
            PokerSuit::Hearts(_) => println!("♥️"),
        }
    }
    fn is_hearts(poker: &PokerSuit) -> bool {
        if let PokerSuit::Hearts(_) = poker {
            true
        } else {
            false
        }
    }
    // fn is_hearts_get_value(poker: &PokerSuit) -> &str {
    //     if let PokerSuit::Hearts(value) = poker {
    //         return &value.to_string()
    //     } else {
    //         ""
    //     }
    // }

    fn is_hearts_get_value(poker: &PokerSuit) -> String {
        if let PokerSuit::Hearts(value) = poker {
            // 将 char 转换为 String，然后返回 String 的切片作为 &str
            return value.to_string();
        } else {
            // 如果不是 Hearts，返回一个空的 &str
            String::new()
        }
    }

    get_color(&clubs_2);
    get_color(&spades_3);
    get_color(&diamonds_a);
    get_color(&hearts_k);
    if is_hearts(&hearts_k) {
        println!("这张牌是红桃花色")
    }
    let value = is_hearts_get_value(&hearts_k);
    println!("值为 {}", value);


    // 如下 Option<T> 则是 rust 实现的一个枚举类，可以避免 空指针错误，值为空时 会走到 None 中进行处理，只有值可以走到 Some 逻辑中去
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // 例如
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         None => None,
    //         Some(i) => Some(i + 1),
    //     }
    // }
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);
}

fn main() {
    // variable_learning();
    let mut a = String::from("Hello");
    let b = String::from(" Rust");
    let res = str_add(&mut a, &b);
    // Hello Rust
    println!("{}", res);
    let word = first_word(&res);
    println!("空格前的第一个元素为 {}", word); // 空格前的第一个元素为 Hello

    tuple_learning();
    let (s, len) = tuple_to_use(String::from("12345678"));
    println!("the length of '{}' is {}", s, len);

    // struct_learning();
    // vec_learning();
    hashmap_learning();

    // enum_learning();
}
