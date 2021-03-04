
pub fn ref_pass(s: &String) {
    println!("{}", s)
}

pub fn p_ref_pass(s: &mut String) {
    *s = String::from("Replacement")
}