// use std::io;
// use rand::Rng;
// fn main() {
//     let mut guess_number = String::new();
//     loop {
//         let secret_number = rand::thread_rng().gen_range(1..=100);
//         println!("please input your guess number (range of 1 to 100 ), input 'exit' to exit");
//         io::stdin().read_line(&mut guess_number).expect("failed to read");
//         let input = guess_number.trim();
//         if input == "exit" {
//             println!("About to exit");
//             break;
//         }
//         if input.is_empty() {
//             println!("input invalid, please input a non-empty string");
//         }
//         if let Ok(num) = input.parse::<i32>() {
//             if num >= 1 && num <= 100 {
//                 if secret_number ==  num {
//                     println!("you guess right");
//                     break;
//                 } else {
//                     println!("you guess wrong, come on! once again, the secret number is {}", secret_number);
//                 }
//             } else {
//                 println!("please input guess number from 1 to 100");
//             }
//         } else {
//             println!("please input the number");
//         }
//         guess_number.clear();
//     }
// }


// 将模块引入当前作用域
// 绝对路径
use crate::garden::vegetables::Asparagus;
// 相对路径
use garden::flowers::Flowers as garden_flowers; // 如果导入引用存在重名的情况时，可以考虑使用 as ，使用别名代替

// 声明一个公有的模块 外部可以使用 use 引入
pub mod garden;

fn main() {
    let the_asparagus = Asparagus::new_asparagus("green", 12);
    let the_asparagus_size = the_asparagus.get_size();
    let the_asparagus_color = the_asparagus.get_color();
    println!("花园中芦笋的颜色为 {}，尺寸为 {}", the_asparagus_color, the_asparagus_size);
    let the_flowers = garden_flowers::new_flowers("玫瑰".to_string(), "red".to_string(), 12);
    println!("花园里还有 {} 朵 {} 色的 {}", the_flowers.get_nums(), the_flowers.get_color(), the_flowers.get_types());
}