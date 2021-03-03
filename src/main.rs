#[warn(unused_imports)]
use hello::*;

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
}

fn do_stuff(first_element: i32 , second_element: i32) -> i32 {
    first_element * second_element
}
