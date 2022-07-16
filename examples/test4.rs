fn main() {
    let mut s = "你好".to_string();
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
