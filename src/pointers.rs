
pub fn heap() -> i32 {
    // e -> smart pointer
    let e = Box::new(7);
    // return value
    return *e;
}