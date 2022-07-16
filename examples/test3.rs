fn main() {
    {
        let a = 232;
        println!("{}", a);
        getwhat();
    }
    {
        let a = String::from("sdfsdfds");
        let s = a.clone();
        let ss = s;
        println!("{}", ss);
        println!("{}", ss);
        let b = &a;
        let c = b;
        println!("{}", b);
        drop(b);
        println!("{}", b);
        assert_eq!(&a, b);
        getwhat();
    }
}

fn getwhat() {
    let aa = 123;
    let b = aa;
    println!("{}", aa);
}
