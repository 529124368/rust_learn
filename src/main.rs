use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn test(num :i32) -> (i32, String){
    if num == 5 {
        println!("不错哦")
    }else {
        println!("{}",num+10)
    }
    let mut flg =0;
    while flg < num {
        flg +=1;
    }
    (flg,"sdfsf".to_string())
}

fn what(n :&mut String)->usize{
    n.push_str("asdasdsa");
    let len = n.len();
    len
}


#[derive(Debug)]
struct Aera {
    width :u32,
    height:u32,
}
impl Aera {
    fn getarea(&self) ->u32 {
        self.width*self.height
    }
}

fn main() {
    const  FG : &str = "sfsdf";
    let he :u32 = "12".parse().expect("error");
    let jieguo = 12.0/5.0;
    println!("{},{},{}",FG,he,jieguo);
    let boxss = [3;5];
    let ee = (123,123,"sdfsf",123.123,'s');
    println!("{}:{}:{}:{}:{}:{}",ee.0,ee.1,ee.2,ee.3,ee.4,boxss[0]);
    //
    let num = test(100);
    println!("{},{}",num.0,num.1);
    //
    let ar = [1,2,3,4,5,6,456,456];

    for a in  ar.iter(){
        println!("{}",a) 
    }
    //
    let mut we = String::from("sdf");
    let s =  what(&mut we);
    println!("{}",s);
    println!("{}",we);


    //
    let tinar = Aera{
        width:12,
        height:12,
    };
    print!("{}",tinar.getarea());
    print!("{:#?}",tinar);
    //
    let randnum = rand::thread_rng().gen_range(0,100);
    println!("现在随机数是:{}",randnum);
    loop {
        let mut buf= String::new();
        io::stdin().read_line(&mut buf).expect("有错误");
        let buf: u32= match buf.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("你输入的数字是:{}",buf);
        match buf.cmp(&randnum) {
            Ordering::Equal => {println!("相等");break;},
            Ordering::Greater=>{println!("大于")},
            Ordering::Less=>{println!("小于")},
        }
    }
}
 