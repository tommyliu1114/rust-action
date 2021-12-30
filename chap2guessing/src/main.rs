use std::io;

fn main() {
    println!("guess!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("read line error");
    println!("your guess is : {} ",guess);

}
