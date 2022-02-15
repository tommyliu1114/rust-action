use chap10trait2::Summary;
use std::fmt::Display;
fn main() {
    println!("Hello, world!");
}


//trait bound
pub fn notify<T: Summary>(item: T) {
    println!("breaking news! {} ",item.summarize())
}

pub fn notify1(item1: impl Summary + Display){
    println!("breaking news! {}",item1.summarize())
}