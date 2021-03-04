### HELLO RUST ###

- It is system programming language.
```
- Systems programming, or system programming, is the activity of programming computer system software.
The primary distinguishing characteristic of systems programming when compared to application programming is that application 
programming aims to produce software which provides services to the user directly (e.g. word processor), 
whereas systems programming aims to produce software and software platforms which provide services to other software, 
are performance constrained, or both (e.g. operating systems, computational science applications, game engines, industrial automation, 
and software as a service applications).

- Systems programming requires a great degree of hardware awareness. Its goal is to achieve efficient use of available resources, either 
because the software itself is performance critical or because even small efficiency improvements directly transform into significant 
savings of time or money.

```

- cargo new {project_name} *> cargo run *> cargo doc

- const --> you can use it outside of function. It is global immutable variable. (Inline)

- let   --> immutable variable

- let mut --> mutable variable

- There is no GC. Data is dropped directly after scope!!!

- Memory safety is guaranteed in compile time.

- Snake case is suggested

- Function doesn't have to be above of other function using it !!!

- println! ==> It is macro. They end with !

- Module system
```
src/
└── lib.rs  --> the hello rust library  
└── main.rs --> the hello rust binary 
```

- Even binary function in another module is private.

- use ==> it is like import from JVM world

- use std // it is like standart
use std::collections::HashMap;

- crates.io ==> Rust package registry

- Cargo.toml ==> dependencies go there!

- Scalars are typically contrasted with compounds, such as arrays, maps, sets, structs, etc. A scalar is a 
"single" value - integer, boolean, perhaps a string - while a compound is made up of multiple scalars (and possibly references to other compounds).
- Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

- usize - it like pointer

- Ownership makes Rust different from others.
3 Rules;
-- Each value has an owner. 
-- There is only one owner
-- Value gets dropped if its owner goes out of scope

- Heap is slower than Stack as it needs to refresh memory cache.

- x / &x (immutable reference to x) / &mut x (mutable reference to x) 

- i32 / &i32 (immutable type reference) / &mut i32 (mutable type reference) 

- & ==> referencing

- * ==> dereferencing
    
```
    x : &mut i32
    *x // a mutable i32
    
    y : &i32
    *y // a immutable i32
```

- You can have unlimited immutable references. No lock mechanism needed.

- Class == Struct 

- String and str
{
  - String is the dynamic heap string type, like Vec: use it when you need to own or modify your string data.
  - str is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory.
  - This means that str most commonly appears as &str: a reference to some UTF-8 data, normally called a 
    "string slice" or just a "slice". A slice is just a view onto some data, and that data can be anywhere
    
--------------------------------------------------------------------------------------------------------------------
    
  - A Rust String is like a std::string; it owns the memory and does the dirty job of managing memory.
  - A Rust &str is like a char* (but a little more sophisticated)
  - The size the str takes up cannot be known at compile time and depends on runtime information

--------------------------------------------------------------------------------------------------------------------

  - A slice,[T], is a view into a block of memory. Whether mutable or not, a slice always borrows and that is why 
    it is always behind a pointer, &.

--------------------------------------------------------------------------------------------------------------------
  - In easy words, String is datatype stored on heap (just like Vec), and you have access to that location.
  - &str is a slice type. That means it is just reference to an already present String somewhere in the heap.
  - &str doesn't do any allocation at runtime. So, for memory reasons, you can use &str over String. But, keep in mind 
    that when using &str you might have to deal with explicit lifetimes.
    

}

- fat pointers (pointer + associated metadata)
```
The fat pointer is 3 * 8 bytes (wordsize) long consists of the following 3 elements:
  - Pointer to actual data on the heap, it points to the first character
  - Length of the string (# of characters)
  - Capacity of the string on the heap

--------------------------------------------------------------------------------------------------------------------
// on 64 bit architecture:
println!("{}", mem::size_of::<&str>()); // 16
println!("{}", mem::size_of::<String>()); // 24

```

- Traits ==> Interface

- You can implement traits for types as well.
```
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

fn test() {
  print_noise(5_u8);
}
```

--> 