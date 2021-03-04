use std::collections::HashMap;

pub fn vector_test() {
    let mut c = vec![2,4];
    c.push(2);
    c.push(3);
    println!("{}", c[1]);
    println!("{}", c[3]);

    println!("{:?}", c.binary_search(&3));
}

pub fn hash_map_test() {
    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(1, true);
    h.insert(1, true);
    h.insert(2, true);
    h.insert(3, false);
    println!("hash_map first element : {}", h[&1]);
    println!("hash_map capacity: {}", h.capacity());
    println!("{:?}", h.contains_key(&3));
    // unwrap --> it is like pattern matching
    let have_tree = h.remove(&3).unwrap();
    println!("{}", have_tree);
}
