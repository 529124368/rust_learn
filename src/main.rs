use std::collections::HashMap;
mod tools;
fn test(num: i32) -> (i32, String) {
    if num == 5 {
        println!("不错哦")
    } else {
        println!("{}", num + 10)
    }
    let mut flg = 0;
    while flg < num {
        flg += 1;
    }
    (flg, "sdfsf".to_string())
}

fn what(n: &mut String) -> usize {
    n.push_str("asdasdsa");
    let len = n.len();
    len
}

#[derive(Debug)]
struct Aera {
    width: u32,
    height: u32,
}

impl Aera {
    fn getarea(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
enum IPaddress {
    Ipv4(String),
    Ipv6(String),
    Num(i32),
}
impl IPaddress {
    fn display(&self) {
        println!("{},{}", "sfsdf", "sfsdf")
    }
}

struct Ggg<T, U, V> {
    Name: T,
    Sex: U,
    Age: V,
}

fn main() {
    tools::example::print_info();
    tools::ceshi();
    //File::open("sfsdf.txt").expect("找不到文件");
    let mut maps = HashMap::new();
    maps.insert("dongwu".to_string(), 123);
    maps.insert("nihao".to_string(), 1);

    //
    let mut aa = Ggg {
        Name: "sfsdf",
        Sex: 12.0,
        Age: 12,
    };
    aa.Name = "wwwwwwww";
    aa.Sex = 12.444;
    aa.Age = 123;
    println!("{}", aa.Name);

    //
    for (k, v) in &maps {
        print!("{}:{}", k, v)
    }
    println!("{:?}", maps);

    let eee = "1,2,2,4,5,6".to_string();

    let bb: Vec<_> = eee.split(',').collect();
    let mut bbbbbb = HashMap::new();
    for v in &bb {
        let count = bbbbbb.entry(v).or_insert(0);
        *count += 1;
        println!("{}", v)
    }
    println!("{:?}", bbbbbb);

    // //
    // let a1 = "sdfsdfsdf".to_string();
    // let a2 = "sfs".to_string();
    // let re = format!("{}-{}",a1,a2);
    // println!("{}",re);
    // let we = 123;
    // if let 123 = we {
    //     println!("asdsf")
    // }

    // let bb = [IPaddress::Ipv4(String::from("safaf")),IPaddress::Ipv6(String::from("sfsdf")),IPaddress::Num(213)];
    // for (i,v) in  bb.iter().enumerate() {
    //     println!("{}",i);
    //     println!("{:?}",v);
    // }

    // let ip4 =IPaddress::Ipv4(String::from("sfdsfdsf"));
    // let ip6 =IPaddress::Ipv6(String::from("sfdsfdsfasdsad"));
    // println!("{:?},{:?}",ip4,ip6);
    // ip6.display();

    // const  FG : &str = "sfsdf";
    // let he :u32 = "12".parse().expect("error");
    // let jieguo = 12.0/5.0;
    // println!("{},{},{}",FG,he,jieguo);
    // let boxss = [3;5];
    // let ee = (123,123,"sdfsf",123.123,'s');
    // println!("{}:{}:{}:{}:{}:{}",ee.0,ee.1,ee.2,ee.3,ee.4,boxss[0]);
    // //
    // let num = test(100);
    // println!("{},{}",num.0,num.1);
    // //
    // let ar = [1,2,3,4,5,6,456,456];

    // for a in  ar.iter(){
    //     println!("{}",a)
    // }
    // //
    // let mut we = String::from("sdf");
    // let s =  what(&mut we);
    // println!("{}",s);
    // println!("{}",we);

    // //
    // let tinar = Aera{
    //     width:12,
    //     height:12,
    // };
    // print!("{}",tinar.getarea());
    // print!("{:#?}",tinar);
    // //
    // let randnum = rand::thread_rng().gen_range(0,100);
    // println!("现在随机数是:{}",randnum);
    // loop {
    //     let mut buf= String::new();
    //     io::stdin().read_line(&mut buf).expect("有错误");
    //     let buf: u32= match buf.trim().parse() {
    //         Ok(num)=>num,
    //         Err(_)=>continue,
    //     };
    //     println!("你输入的数字是:{}",buf);
    //     match buf.cmp(&randnum) {
    //         Ordering::Equal => {println!("相等");break;},
    //         Ordering::Greater=>{println!("大于")},
    //         Ordering::Less=>{println!("小于")},
    //     }
    // }
}
