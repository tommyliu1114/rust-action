fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello world");
    let word_index = first_word(&s);
    //s.clear();
    println!("{}",word_index);
    let hello = &s[0 ..5];
    let word = &s[5 ..11];
    println!("{} , {}",hello,word);
    let slicef = slice_first_word(&s);
    println!("original is {},slices is {}",s,slicef);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return i
        }
    }
    s.len()
}

fn slice_first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0 .. i]
        }
    }
    return &s[..]
}