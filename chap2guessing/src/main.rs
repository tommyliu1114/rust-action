use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("guess!");
    let secret = rand::thread_rng().gen_range(1,89);
    println!("your gduess is : {} ",secret);
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("read line error");
        println!("your gddduess is aa : {} ",guess);
        let guess_n :u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("input wrong");continue} 
        };
        match guess_n.cmp(&secret){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {println!("you got the number : {} ",guess);break;}
        }
    }
 

}
