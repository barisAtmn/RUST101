
// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

pub fn st() {
    let s: &'static str = "hello world";
    println!("This is {}", LANGUAGE);

    // Syntax of &str[stored in Stack]:
    let name: &str = "Rust";
    // Syntax for String[stored in Heap]:
    let name1: String = String::from("Rust");
}
