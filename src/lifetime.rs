
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

pub fn lt() {
    let a;
    let a2;
    {
        let b = String::from("hello");
        a = b; // b gives ownership of memory!
        // a = &b; // borrowed value does not live long enough
        // b is cleaned with parenthesis. After it, a would point a empty object. It causes Dangling pointers
        // AND RUST DOESNT ALLOW IT.

    }
    println!("{}",a);
    {
        let b = "hello";
        a2 = b;
    }
    println!("{}",a2);

    let pa = "LIFE TIME";
    let pa2 = 10;
    st2(&pa);
    st3(&pa2);
}

fn st2(param1: &str) -> &str {
    println!("{}",param1);
    param1
}

fn st3(param1: &i32) -> &i32 {
    println!("{}",param1);
    param1
}