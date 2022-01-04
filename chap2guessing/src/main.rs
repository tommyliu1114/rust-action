use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("guess!");
    let secret = rand::thread_rng().gen_range(1,89);
    println!("your gduess is : {} ",secret);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("read line error");
    println!("your gduess is : {} ",guess);
    let guess_n :u32 = guess.trim().parse().expect("please input a number");
    match guess_n.cmp(&secret){
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("you got the number : {} ",guess)
    }

}
