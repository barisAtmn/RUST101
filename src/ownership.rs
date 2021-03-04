
// ownership makes Rust safe!
pub fn test_owner() {
    let s1 = String::from("abc");
    let s2 = s1;
    println!("you cant use s1 anymore as you gave ownership to s2!!!");
    println!("{}", s2);

}