use std::io;
use rand::Rng;
fn main() {
    println!("guess!");
    let secret = rand::thread_rng().gen_range(1,89);
    println!("your gduess is : {} ",secret);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("read line error");
    println!("your gduess is : {} ",guess);

}
