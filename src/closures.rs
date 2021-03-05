
pub fn closure_test() {
    let add = |x,y|{x + y};
    let result = add(1,2);
    println!("{}", result);

    // it can get data from its scope!!!
    let empty_parameters = || {result};
    let result2 = empty_parameters();
    println!("{}",result2);

    // closure can get local variable ownership with move !!!
    let s:i32 = 3;
    let f = move || {
        println!("{}",s);
    };
    println!("{}",s.is_positive());
}

pub fn functional_programming_closure() -> i32 {
   let mut v = vec![2,4,6];
    let d = v.iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc:i32, x | acc + x);
    return d;
}

pub fn functional_programming_closure2() -> i32 {
    let v = vec![2,4,6];
    let d = v.iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc:i32, x | acc + x);
    return d;
}