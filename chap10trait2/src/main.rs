use chap10trait2::Summary;
use std::fmt::Display;
use chap10trait2::NewsArticle;
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

//trait where 

pub fn  notify2<T,U>(a: T,b: U) -> String 
where T: Summary + Display,
U: Summary{
    println!("breaking news! {}, b is : {} ",a.summarize(),b.summarize());
}

//traint as return 

pub fn notify4(s: &str) -> impl Summary {
    NewsArticle {
        headline: String::from("peguins win the stanley"),
        content: String::from("he once agin are the best hockey team in the nhl "),
        author: String::from("iceburgh"),
        location: String::from("pa,usa"),
    }
}
