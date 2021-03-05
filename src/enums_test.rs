use std::borrow::{Borrow, BorrowMut};
use std::any::Any;
use std::convert::TryInto;
use std::fmt::{Pointer, Debug};
use std::fs::File;
use std::io::Error;
use log::debug;

#[derive(Debug)]
enum Color {
    Empty,
    Red(String),
    Blue(String),
    Make {x:String, y:String}
}


// option implements iterator traits!!!
pub fn test_enum() {
    let option = Option::Some(String::from("hello enum"));
    for i in option {
        println!("{}", i);
    }
    let color = Color::Blue(String::from("blue"));

    println!("{:#?}", color);
}


pub fn open_file() {
    let result = File::open("foo");
    if result.is_ok() {
        let f = result.unwrap();
        let m = f.metadata().unwrap().accessed().unwrap();
    } else {
        match result {
            Err(e) => println!("ops!, {}", e),
            _ => {}
        }
    }
}

pub fn receives_closure<F>(closure: F)
    where
        F: Fn(i32) -> i32,
{
    let result = closure(1);
    debug!("closure(1) => {}", result);
}

pub fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 4
}