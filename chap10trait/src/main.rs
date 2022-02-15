use chap10trait::Summary;
use chap10trait::Tweet;
fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };
    println!("1 nwe tweet: {} ",tweet.summarize());
}




