fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("the length of {} is {}.",s1,len);
    let mut s2 = String::from("hi,");
    let s2_len = mut_calculate_length(& mut s2);
    println!("the length of {} is {}",s2,s2_len);
}
fn calculate_length(s:&String) -> usize {
    s.len()
}

fn mut_calculate_length(s:& mut String) -> usize {
    s.push_str("lewis");
    s.len()
}