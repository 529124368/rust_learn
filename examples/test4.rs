fn main() {
    let mut s = "你好".to_string();
    let mm = "asdadada";
    let slicestr = &mm[..=mm.len() - 1];
    let bb = [1, 2, 3, 4, 5, 6];
    let ss = &bb[1..];
    println!("{}", ss[0]);
    println!("{}", slicestr);
    println!("##############");
    println!("{}", s);
    change_string(&mut s);
    println!("{}", s);
    search_string(&s);
    println!("{}", s);
}
fn change_string(a: &mut String) {
    *a = "你好啊,兄弟,123,$$$".to_string() + "sfsfsfsdfasdasd";
}

fn search_string(a: &String) {
    let f: Vec<&str> = a.split(',').collect();
    for i in 0..f.len() {
        println!("{}", f[i]);
    }
    for v in a.split(',') {
        println!("{}", v);
    }
}
