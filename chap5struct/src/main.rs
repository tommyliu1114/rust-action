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