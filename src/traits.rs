
struct RedFox {
    enemy: bool,
    life: u32,
}

struct RedCat {
    name: &'static str,
}

impl Default for RedCat {
    fn default() -> Self {
        Self {
            name:"pisi"
        }
    }
}

impl Default for RedFox {
    fn default() -> Self {
        Self{
            enemy:true,
            life:3,
        }
    }
}

pub fn callF() {
    let re = RedCat::default();
    let rf = RedFox::default();
    print_noise(re);
    print_noise2(&rf);
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {"Meow?"}
}

impl Noisy for RedCat {
    fn get_noise(&self) -> &str {"Meow!"}
}

// You can implement it for types as well.
impl Noisy for u8 {
    fn get_noise(&self) -> &str {"BYTE!"}
}

// generic function for trait
fn print_noise<T:Noisy>(item:T) {
    println!("{}", item.get_noise());
}

fn print_noise2(item:&dyn Noisy) {
    println!("{}", item.get_noise());
}