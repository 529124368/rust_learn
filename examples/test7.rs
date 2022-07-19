use std::{
    fs::File,
    io::{self, Read},
};

use md5::{Digest, Md5};

fn bacjFile() -> Result<String, io::Error> {
    let mut str = String::new();
    File::open("tests.txt")?.read_to_string(&mut str)?;
    Ok(str)
}
#[derive(Debug)]
enum Std {
    text(String),
    name(String),
    typess(u8),
}
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::Cons;
use List::Nil;
#[derive(Debug)]
struct L(usize);
fn main() {
    let a = L(12);
    println!("{:?}", a);
    let aa = 5;
    let bb = Box::new(aa);
    println!("{}", bb);
    let chian = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:#?}", chian);
    println!("@@@@@@@@@@@@@@@@");
    let mut boxss = vec![
        Std::text("sfsfs".to_string()),
        Std::text("sfsfs".to_string()),
        Std::text("sfsfs".to_string()),
    ];
    let mut strbox = String::new();
    for v in &mut boxss {
        strbox += &format!("{:?}", v);
    }
    println!("{:?}", strbox);
    //
    let s = match bacjFile() {
        Ok(f) => f,
        Err(err) => err.to_string(),
    };
    println!("{}", s);
    let mut pass = Md5::new();
    pass.update(b"adfwefewfege");
    let result = pass.finalize();
    println!("{:X}", result);
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
