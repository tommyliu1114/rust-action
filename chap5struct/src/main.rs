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
    let reec2 = Rectangle {
        width: 20,
        length:32,
    };
    let reec3 = Rectangle {
        width: 40,
        length:12,
    };    
    let s2 = recArea(&reec);
    println!("area is {},{},{},{}",s,s1,s2,reec.recArea());
    println!("{:#?}",reec);
    println!("{}",reec.can_hold(&reec2))
}

// rust 调用方法，自动添加&，&mut ，* ，方便object匹配方法签名
#[derive(Debug)]
struct Rectangle {
    width:u32,
    length:u32,
}

impl Rectangle {
    fn recArea(&self) -> u32 {
        self.length * self.width
    } 
    fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
    //关联函数，不把self作为第一个参数的函数，构造器常用
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
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