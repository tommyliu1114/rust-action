fn main() {
    println!("Hello, world!");
    let mut user1 = User{
        email: String::from("1@126.com"),
        active: true,
        username: String::from("lewis1"),
        singn_in_count:20,
    };
    //更新语法
    let user2 = User{
        email: String::from("s: &str"),
        username: String::from("s: &str"),
        ..user1
    };
    //所有权
    let s = area(32, 20);
    let s1 = areadim((32,20));
    let reec = Rectangle {
        width: 30,
        length:12,
    };
    let s2 = recArea(&reec);
    println!("area is {},{},{}",s,s1,s2);
    
}
struct Rectangle {
    width:u32,
    length:u32,
}

fn recArea(reec: &Rectangle) -> u32 {
    reec.length * reec.width
}
//tuple struct
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

struct User {
    username: String,
    email: String,
    singn_in_count: u64,
    active: bool,
}

fn area(width: u32,length: u32) -> u32{
    width * length
}
fn areadim(dim:(u32,u32)) -> u32 {
    dim.0 * dim.1
}