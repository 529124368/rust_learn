use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    let randnum = rand::thread_rng().gen_range(0,100);
    println!("现在随机数是:{}",randnum);
    loop {
        let mut buf= String::new();
        io::stdin().read_line(&mut buf).expect("有错误");
        let buf: u32= buf.trim().parse().expect("转换错误");
        println!("你输入的数字是:{}",buf);
        match buf.cmp(&randnum) {
            Ordering::Equal => {println!("相等");break;},
            Ordering::Greater=>{println!("大于")},
            Ordering::Less=>{println!("小于")},
        }
    }

}
 