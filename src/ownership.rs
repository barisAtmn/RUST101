
// ownership makes Rust safe!
pub fn test_owner() {
    let s1 = String::from("abc");
    let s2 = s1;
    println!("you cant use s1 anymore as you gave ownership to s2!!!");
    some_fn(&s2);
    println!("------------------------------");
    let s3 = &s2;
    println!("{}", *s3);
    println!("------------------------------");
    println!("{}", s3);
}

pub fn test_owner2() {
    let s1 = 3;
    let s2 = &s1;
    println!("------------------------------");
    println!("{}", *s2);
    println!("------------------------------");
    println!("{}", s2);
    println!("------------------------------");
    println!("{}", &s2);
}

pub fn some_fn(s :&String) {
    println!("{}", s);
}