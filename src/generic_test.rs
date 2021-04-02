use std::fmt::Debug;
use std::ops::Add;

struct Point<T> {
    a:T,
    b:T,
}
pub fn gn() {
    let a = Point{a:3, b:4};
    println!("{}, {}",a.a,a.b);
    let b = String::from("hello");
    let c = String::from("world");
    let d = 3;
    let e = 4 ;
    gn_f(b,&c, d);
    println!("{}, {}", d, c);
    sum(3,4);
}

fn sum<T: Add<Output=T>+Debug>(param:T, param2: T) {
    println!("{:?}", param + param2)
}

#[allow(dead_code)]
fn sum2<T>(param:T, param2: T)
where T : Add<Output=T>+Debug {
    println!("{:?}", param + param2)
}

fn gn_f<T:Debug>(param:T, param2: &T, param3: i32) {
    println!("=========================");
    println!("{:?},{:?}, {}", param, param2, param3);
}