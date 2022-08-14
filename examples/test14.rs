fn main() {
    let mut s = "$%7fsdfs".to_string();
    {
        s = "a".to_string();
    }
    {
        s = "s".to_string();
    }
    {
        s = "d".to_string();
    }
    println!("{s}");
}
