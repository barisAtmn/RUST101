use rand::{thread_rng, Rng};

pub fn hi() {
    println!("Hi")
}

pub fn create_random() -> i32 {
   return thread_rng().gen_range(1,1000000);
}