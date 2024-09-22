use std::{borrow::Borrow, cell::RefCell, rc::Rc, sync::Arc};




#[derive(Debug)]
struct Node {
    id: usize,
    // 使用 Rc<RefCell<T>>  让节点可以被修改
    downstream: Option<Rc<RefCell<Node>>>

}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_down_stream(&mut self, downstream: Rc<RefCell<Node>>) {
        self.downstream = Some(downstream)
    }

    pub fn get_down_stream(&self) -> Option<Rc<RefCell<Node>>> {
        // Option<Rc<Node>> 没有实现 copy trait 所有需要使用 as_ref 来借用，避免所有权转移到闭包中
        self.downstream.as_ref().map(|v| v.clone() )
    }
}



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


    // 有向无环图实现
    let mut node1 = Node::new(1); 
    let mut node2 = Node::new(2); 
    let mut node3 = Node::new(3); 
    let node4 = Node::new(4); 
    node3.update_down_stream(Rc::new(RefCell::new(node4))); 
    node1.update_down_stream(Rc::new(RefCell::new(node3))); 
    node2.update_down_stream(node1.get_down_stream().unwrap());
    println!("node1: {:?}\nnode2: {:?}", node1, node2);

    // Rc 是一个只读的引用计数器，无法直接做为可变引用
    // let node5 = Node::new(5);
    // let node3 = node1.get_down_stream().unwrap();
    // node3.update_down_stream(Rc::new(node5)); // 改变 node3 的 downstream 为 node5

    let data = RefCell::new(1); 
    { 
        // 如果将大括号移除，可以通过编译，但是运行时会报错。不能同时有活跃的可变借用和不可变借用。
        // 内部可变性，可以绕过编译器检查，但是运行时依旧绕不开检查
        // 运行时报错，会直接 panic

        // 获得 RefCell 内部数据的可变借用 
        let mut v = data.borrow_mut(); 
        *v += 1; 
    }
    println!("data: {:?}", data.borrow()); // 2

    // 使用 RefCell 解决
    let node5 = Node::new(5);
    let node3 = node1.get_down_stream().unwrap();
    // (*node3).borrow_mut().downstream = Some(Rc::new(RefCell::new(node5)));
    node3.borrow_mut().update_down_stream(Rc::new(RefCell::new(node5)));
    println!("node1: {:?}\nnode2: {:?}", node1, node2);


    let arr = Arc::new(vec![1]); 
    let arr1 = arr.clone();
    //  move, in order to give ownership of values to a thread。 arr1 的所有权转移到了闭包中
    std::thread::spawn(move || { 
        println!("the arr is {:?}", arr1); 
    }).join().unwrap();
    // arr 和 arr1 都共享一块地址
    println!("the arr is {:?}", arr)


}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    // 这里的 data 和 &data 指向的是同一个 Vec<u32> 实例，但是它们的类型不同。data 是一个 &Vec<u32> 类型的引用，而 &data 是一个 &&Vec<u32> 类型的引用。
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data); // addr of value: 0x16fa5e7f0, addr of ref: 0x16fa5e6d8
    data.iter().fold(0, |acc, x| acc + x)
}