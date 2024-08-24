pub struct Asparagus {
    color: String,
    size: i32
}

// 实现 结构体， pub 公有化 结构体方法。默认是私有的
impl Asparagus {
    pub fn new_asparagus(color: &str, size: i32) -> Self {
        Self {
            color: color.to_string(),
            size
        }
    }
    pub fn get_color(&self) -> &String {
        &self.color
    }
    pub fn get_size(&self) -> i32 {
        self.size
    }
}