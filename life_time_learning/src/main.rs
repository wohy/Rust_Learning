fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // } // 这里 &x 的生命周期就结束了，所以为了运行时产生 悬空指针，编译将不通过
    // println!("r: {r}");

    // 显示声明生命周期 解决

    fn print_r<'a>() {
        let r;
        {
            let x:&'a i32 = &5; // x 的引用 生命周期将与 print_r 相同
            r = x
        }
        println!("r: {r}");
    }
    print_r(); // r: 5

}
