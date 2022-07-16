fn main() {
    let man = Man::new();
    man.get_name();
}

struct Man {
    name: String,
    age: u32,
}
impl Man {
    fn get_name(&self) {
        println!("我的名字是:{}\n我的年龄是:{} ", self.name, self.age);
    }
    fn new() -> Man {
        Man {
            name: (String::from("小明")),
            age: (12),
        }
    }
}
