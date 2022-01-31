use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let mut scores = HashMap::new();
    scores.insert(String::from("hello world"), 23);
    scores.insert(String::from("nickname is nil"), 100);
    let teams = vec![String::from("blue"),String::from("yellow")];
    let initial_scoers = vec![10,23];
    let mut nscores: HashMap<_,_> = 
    teams.iter().zip(initial_scoers.iter()).collect();
    // 所有权
    let blue = String::from("blued");
    match nscores.get(&blue) {
        Some(s) => println!("{}" ,s),
        None => println!("team not exists")

    }

    for (k,v ) in &nscores {
        println! ("{}: {}",k,v);
    }
    let zbrown = String::from("brown");
    let nen = nscores.entry(&zbrown);
    nen.or_insert(&50);
    


}
