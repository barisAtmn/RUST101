
// string will point to `static memory which lives through the whole program
struct Person {
    name: &'static str,
    surname : &'static str,
    age : u8,
    nation : &'static str
}

pub fn struct_test1() {
    let john:Person = Person::new();
    println!("{}, {}, {}, {}", john.name, john.age, john.nation, john.surname );

    Person::test();

    let baris:Person = Person {
        name: "baris",
        surname: "ataman",
        age: 29,
        nation: "TR"
    };
    println!("{}, {}, {}, {}", baris.name, baris.age, baris.nation, baris.surname );
}

// constructor with default values. dont use str :)) this is just test.
// you can use Person instead of Self also
impl Person {
    fn new() -> Self {
        Self {
            name: "baris",
            surname: "ataman",
            age: 29,
            nation: "TR"
        }
    }

    fn test() {
        println!("{}", Person::new().surname);
    }

}