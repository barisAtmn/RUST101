#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

pub fn derive_debug_test() {
    let name = "Baris";
    let age = 29;
    let baris = Person { name, age };

    // Pretty print
    println!("{:#?}", baris);
}
