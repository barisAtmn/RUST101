fn main() {

    let name = "baris";
    let age = 29;
    let mut city = "tallin";
    println!("Hello, world!, {}. Age is :  {} , Living in : {}", name, age, city);

    city = "Tallin";

    println!("Hello, world!, {}. Age is :  {} , Living in : {}", name, age, city);
    const  X : i32= 3;
    {
        let  y : i32= 13;
        let name = "Baris";
        println!("{}, {} , {}", X, y, name)
    }

    println!("{}", X);

    let _type_shadow = "type";
    let _type_shadow = true;

    println!("{}", _type_shadow);


}
