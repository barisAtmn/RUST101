
// string will point to `static memory which lives through the whole program
struct Person {
    name: &'static str,
    surname : &'static str,
    age : u8,
    nation : &'static str
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

    // same like this in OOP!
    fn is_younger(&self, age_param:u8) -> bool {
        self.age < age_param
    }

}

pub fn struct_test1() {
    let john:Person = Person::new();
    println!("{}, {}, {}, {}", john.name, john.age, john.nation, john.surname );

    println!("{}", john.is_younger(9));

    let baris:Person = Person {
        name: "baris",
        surname: "ataman",
        age: 29,
        nation: "TR"
    };
    println!("{}, {}, {}, {}", baris.name, baris.age, baris.nation, baris.surname );
}

