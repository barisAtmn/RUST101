#[warn(unused_imports)]
use hello::{hi,create_random};
use hello::types::{types, condition, loop_test, os_env_ar};
use hello::ownership::{test_owner};
use hello::references_borrowing::{ref_pass, p_ref_pass};
use hello::struct_test::struct_test1;
use hello::collections_test::{vector_test, hash_map_test};
use hello::enums_test::{test_enum, open_file, receives_closure, returns_closure};
use hello::derive_test::derive_debug_test;
use hello::closures::*;


const  X : i32= 3;

fn main() {

    let name = "baris";
    let age = 29;
    let mut city = "tallinn";
    println!("Hello, world!, {}. Age is :  {} , Living in : {}", name, age, city);

    city = "Tallinn";

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

    hello::struct_test::struct_test1();

    vector_test();
    hash_map_test();

    test_enum();

    derive_debug_test();

    open_file();

    closure_test();

    println!("{}", functional_programming_closure());

    println!("{}", functional_programming_closure2());

    let closure = |_| {3};
    receives_closure(closure);

    returns_closure();
}

fn do_stuff(first_element: i32 , second_element: i32) -> i32 {
    first_element * second_element
}
