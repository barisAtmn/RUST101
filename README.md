### HELLO RUST ###

==> It is system programming language.
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

==> cargo new {project_name} *> cargo run *> cargo doc

==> const --> you can use it outside of function. It is global immutable variable. (Inline)

==> let   --> immutable variable

==> let mut --> mutable variable

==> There is no GC. Data is dropped directly after scope!!!

==> Memory safety is guaranteed in compile time.

==> Snake case is suggested

==> Function doesn't have to be above of other function using it !!!

==> println! ==> It is macro. They end with !

==> Module system
```
src/
└── lib.rs  --> the hello rust library  
└── main.rs --> the hello rust binary 
```

==> Even binary function in another module is private.

==> use ==> it is like import from JVM world

==> use std // it is like standart
use std::collections::HashMap;

==> crates.io ==> Rust package registry

==> Cargo.toml ==> dependencies go there!

==> Scalars are typically contrasted with compounds, such as arrays, maps, sets, structs, etc. A scalar is a 
"single" value - integer, boolean, perhaps a string - while a compound is made up of multiple scalars (and possibly references to other compounds).
- Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

==> usize - it like pointer

==> Ownership makes Rust different from others.
3 Rules;
- Each value has an owner. 
- There is only one owner
- Value gets dropped if its owner goes out of scope

==> Heap is slower than Stack as it needs to refresh memory cache.