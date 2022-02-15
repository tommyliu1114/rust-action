use chap10trait2::Summary;

fn main() {
    println!("Hello, world!");
}


//trait bound
pub fn notify<T: Summary>(item: T) {
    println!("breaking news! {} ",item.summarize())
}