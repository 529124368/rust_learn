use std::{collections::HashMap, vec};

fn main() {
    testlib::factorey::fac::prt();
    let a = Ip_address::v3("asdad".to_string());
    println!("{:#?}", a);
    let ss = "asdads";
    match ss {
        "asdasd" => println!("asdada"),
        _ => println!("adad"),
    };
    let name = "你好啊阿达大大".to_string();
    for v in name.chars() {
        println!("{}", v);
    }
    let kys = vec![
        String::from("num1"),
        String::from("num2"),
        String::from("num3"),
    ];
    let vs = vec![1, 2, 3];
    let maps: HashMap<_, _> = kys.iter().zip(vs.iter()).collect();
    let k = &"num1".to_string();
    let mut vvv = 0;
    if let Some(value) = maps.get(k) {
        vvv = **value;
    }
    println!("get value :{}", vvv);
    println!("##################");
    let mut newmaps = HashMap::new();
    newmaps.insert(String::from("1"), 13);
    newmaps.insert(String::from("2"), 13);
    newmaps.insert(String::from("3"), 13);
    newmaps.entry(String::from("3")).or_insert(123);
    for (_, v) in newmaps.iter_mut() {
        *v = *v + 1;
    }
    println!("{:#?}", newmaps);
}
#[derive(Debug)]
enum Ip_address {
    v4(String),
    v3(String),
}
