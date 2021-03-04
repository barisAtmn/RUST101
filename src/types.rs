use std::ffi::OsString;

pub fn types() {
    let arr:[i32;3] = [1,2,3];
    println!("{}",arr.last().get_or_insert(&1));
    println!("{:?}", arr);
}

pub fn condition() {
    let a = 5;
    let b = 6;
    let value = if a > b {3} else {4};
    println!("{}", value);
}

pub fn loop_test() {
    let a = 3;
    for var in 0..10 {
        if var == a {
            println!("var is 3")
        }
    }
}

pub fn os_env_ar() {
    let args:Vec<OsString>= std::env::args_os().collect();
    for arg in args {
        println!("{}", arg.to_str().get_or_insert(&"empty"));
    }
}

