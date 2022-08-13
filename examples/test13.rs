use lsz_macro::lszMacro;

#[derive(lszMacro, Default)]
struct Test {
    name: String,
    sex: u8,
}
fn main() {
    let mut ss = Vec::new();
    ss.push("a");
    ss.push("b");
    println!("{}", ss[1]);
    let mut s = Test::new();
    s.set_sex(12);
    s.set_name("你好啊哦啊".to_string());
    println!("{}", s.get_name());
    println!("{}", s.get_sex());
}
