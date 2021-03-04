
struct RedFox {
    enemy: bool,
    life: u32,
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {"Meow?"}
}

// You can implement it for types as well.
impl Noisy for u8 {
    fn get_noise(&self) -> &str {"BYTE!"}
}

// generic function for trait
fn print_noise<T:Noisy>(item:T) {
    println!("{}", item.get_noise());
}