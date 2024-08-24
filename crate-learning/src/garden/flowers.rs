pub struct Flowers {
    types: String,
    color: String,
    nums: i32,
}

impl Flowers {
    pub fn new_flowers(types: String, color: String, nums: i32) -> Self {
        Self {
            types,
            color,
            nums
        }
    }
    pub fn get_types(&self) -> &String {
        &self.types
    }
    pub fn get_color(&self) -> &String {
        &self.color
    }
    pub fn get_nums(&self) -> i32 {
        self.nums
    }
}
