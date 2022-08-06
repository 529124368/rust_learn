// macro_rules! sqr {
//     ($x:expr) => {{
//         println!("sfdfd");
//         $x * $x
//     }};
// }

use zimu_macro::zimuMacro;

#[derive(zimuMacro, Default)]
struct Test {
    name: String,
    sex: u8,
}

fn main() {
    let s = Test::new();
    println!("{}", s.get_name());
    println!("{}", s.get_sex());
    // println!("{}", sqr!(1 + 1));
}
