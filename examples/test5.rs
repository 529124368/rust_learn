fn main() {
    let varsion = "3.1.1".to_string();
    let name: String = "diablo".to_string();
    let camer: u8 = 0;
    let controller = &Controller {
        mouse: 1,
        keyboard: 2,
    };
    let engine = Game {
        varsion,
        name,
        camer,
        controller,
    };
    let engine1 = Game {
        varsion: "dsada".to_string(),
        name: "asdada".to_string(),
        ..engine
    };
    println!("{:#?}", engine1);
    println!("{:#?}", engine);
    //元祖结构体
    let point3 = vector3(12, 3242);
    println!("{:#?}", point3);
}

#[derive(Debug)]
struct Game<'a> {
    varsion: String,
    name: String,
    camer: u8,
    controller: &'a Controller,
}
#[derive(Debug)]
struct Controller {
    mouse: u8,
    keyboard: u8,
}
#[derive(Debug)]
struct vector3(i32, i32);
