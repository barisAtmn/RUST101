#[warn(unused_imports)]
use hello::{hi,create_random};
use hello::types::{types, condition, loop_test, os_env_ar};
use hello::ownership::{test_owner};
use hello::references_borrowing::{ref_pass, p_ref_pass};

const  X : i32= 3;

fn main() {

    let name = "baris";
    let age = 29;
    let mut city = "tallin";
    println!("Hello, world!, {}. Age is :  {} , Living in : {}", name, age, city);

    city = "Tallin";

    println!("Hello, world!, {}. Age is :  {} , Living in : {}", name, age, city);

    {
        let  y : i32= 13;
        let name = "Baris";
        println!("{}, {} , {}", X, y, name)
    }

    println!("{}", X);

    let _type_shadow = "type";
    let _type_shadow = true;

    println!("{}", _type_shadow);

    println!("{}", do_stuff(3, 2));

    hello::hi();


    println!("{}", hello::create_random());

    hello::types::types();
    hello::types::condition();
    hello::types::loop_test();
    hello::types::os_env_ar();

    hello::ownership::test_owner();

    // borrowing -- reference passing
    let s1 = String::from("abc");
    hello::references_borrowing::ref_pass(&s1);
    println!("{}",s1);

    let mut s2 = String::from("Hello");
    println!("first value : {}",s2);
    hello::references_borrowing::p_ref_pass(&mut s2);
    println!("second value :{}",s2);

}

fn do_stuff(first_element: i32 , second_element: i32) -> i32 {
    first_element * second_element
}
