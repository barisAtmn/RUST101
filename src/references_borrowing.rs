
pub fn ref_pass(s: &String) {
    println!("{}", s)
}

pub fn p_ref_pass(s: &mut String) {
    *s = String::from("Replacement")
}

pub fn string_test(){
    let hello = String::from("String are cool");
    let any_char = &hello[5..6]; // = let any_char: &str = &hello[5..6];
    println!("{:?}",any_char);
}

pub fn shadowing() {
    let s: &str = "hello"; // &str
    let s: String = s.to_uppercase(); // String
    println!("{}", s) // HELLO
}