use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

#[tokio::main]
async fn main() {
    //let (sd, rc) = mpsc::channel();
    // let bb = Arc::new(Mutex::new(1));
    let sum = 0;
    // let h1 = tokio::spawn(async move { f1(sum, sd).await });
    // let h2 = tokio::spawn(async move { f2(rc).await });
    let a = tokio::spawn(async {
        for i in 0..70 {
            println!("第一个携程{}", i)
        }
    });
    let b = tokio::spawn(async {
        for i in 0..50 {
            println!("第二个携程{}", i)
        }
    });
    let _ = tokio::join!(a, b);
}

// async fn f1(mut a: u32, sd: Sender<u32>) {
//     for _ in 0..50 {
//         a += 1;
//         sd.send(a).unwrap();
//         thread::sleep(Duration::from_micros(1));
//     }
// }

// async fn f2(rc: Receiver<u32>) {
//     loop {
//         match rc.recv_timeout(Duration::new(1, 0)) {
//             Ok(s) => println!("通道的值是:{}", s),
//             Err(_) => {
//                 println!("接受结束");
//                 return;
//             }
//         }
//     }
// }
