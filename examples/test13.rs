// macro_rules! sqr {
//     ($x:expr) => {{
//         println!("sfdfd");
//         $x * $x
//     }};
// }

use zimu_macro::zimuMacro;

#[derive(zimuMacro)]
struct test;

fn main() {
    test::hello_world();
    // println!("{}", sqr!(1 + 1));
}
