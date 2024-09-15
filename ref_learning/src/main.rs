fn main() {
    let data: Vec<u32> = vec![1, 2, 3, 4];
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    // 其中 &data, data1 都是指向 Vec<u32> 的引用，而 &&data 引用的引用，&data1 是另一个引用的引用
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    ); // addr of value: 0x16fa5e7f0(0x16fa5e7f0), addr of data 0x16fa5e8c8, data1: 0x16fa5e808
    println!("sum of data1: {}", sum(data1)); // sum of data1: 10

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    ); // addr of items: [0x151605da0, 0x151605da4, 0x151605da8, 0x151605dac]
}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    // 这里的 data 和 &data 指向的是同一个 Vec<u32> 实例，但是它们的类型不同。data 是一个 &Vec<u32> 类型的引用，而 &data 是一个 &&Vec<u32> 类型的引用。
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data); // addr of value: 0x16fa5e7f0, addr of ref: 0x16fa5e6d8
    data.iter().fold(0, |acc, x| acc + x)
}