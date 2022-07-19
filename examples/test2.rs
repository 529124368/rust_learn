use std::{collections::HashMap, thread, time::Duration};

trait Man {
    fn say(&self, a: String) -> String;
    fn update_name(&mut self, a: String);
    fn display_name(&self) -> String;
}

struct Student {
    name: String,
    age: u32,
}
impl Man for Student {
    fn say(&self, a: String) -> String {
        a + &self.name
    }
    fn update_name(&mut self, a: String) {
        self.name = a
    }
    fn display_name(&self) -> String {
        let name;
        name = &self.name;
        name.to_string()
    }
}

impl Student {
    fn new() -> Student {
        Student {
            name: "sdfsdfsf".to_string(),
            age: 1321,
        }
    }
}

fn deal_stu(a: &Student) {
    println!("{}", a.name)
}

fn getMax<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b {
        return a;
    } else {
        return b;
    }
}
fn main() {
    let xxx = (123, 213, "sdfsf");
    println!("{},{},{}", xxx.0, xxx.1, xxx.2);
    let sss = Vec::<i32>::new();
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let ff = |a| -> i32 {
        let sum = 0;
        println!("sdfsdfdss");
        return sum + a;
    };
    let re = ff(121);
    println!("{}", re);
    //
    let mut stu = Student::new();
    deal_stu(&stu);
    println!("{}", stu.say(String::from("cao ni ")));
    stu.update_name("xiao hong ".to_string());
    println!("{}", stu.name);
    let stuna = stu.display_name();
    println!("{}", stuna);
    println!("{}", stu.name);
    //
    let aa = getMax("qwe", "qwes");
    println!("{}", aa);
    //

    let mut a = "asdasdasd".to_string();
    let b = "fdsadsa";
    what(&mut a, b);
    a.clear();
    println!("{}", a);

    let mut ss = vec![123, 123, 123, 123, 12, 312, 312, 321, 3];
    ss.remove(4);
    ss.push(2);

    for v in ss.iter_mut() {
        *v = *v + 1;
    }
    for i in 0..ss.len() {
        ss[i] += 100;
    }

    let mut bb = HashMap::new();
    bb.insert("asdasd", 123123);
    bb.insert("asdasd1", 123123);
    bb.insert("asdasd2", 123123);
    bb.insert("asdasd3", 123123);
    for (_, v) in bb.iter_mut() {
        *v = *v + 100;
    }
    println!("{:?}", bb);
    println!("{:?}", ss);
    handle.join().unwrap();
}

fn what<'a>(s: &'a mut String, b: &'a str) -> &'a str {
    //函数内部修改这个s参数的值
    *s = "@@@@@@@@@@@".to_string();
    s
}
