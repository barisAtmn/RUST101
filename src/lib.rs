pub mod types;
pub mod ownership;
pub mod references_borrowing;
pub mod struct_test;
pub mod traits;
pub mod collections_test;
pub mod enums_test;
pub mod derive_test;

use rand::{thread_rng, Rng};

pub fn hi() {
    println!("Hi")
}

pub fn create_random() -> i32 {
   return thread_rng().gen_range(1,1000000);
}