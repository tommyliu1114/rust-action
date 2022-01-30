fn main() {
    println!("Hello, world!");
    let mut smu = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s1 = String::from("initial string contents");
    
    smu.push_str("ni hao ");
    smu.push('a');
    println!("smu is {}",smu);

    // 
    let sp1 = String::from("emoji ");
    let sp2 = String::from(" himoji");
    let sp3 = sp1 + &sp2;
    //println!("summary is {} ",sp1);
    println!("summary is {} ",sp2);
    println!("summary is {} ",sp3);
    //string 索引
    let slen = String::from("你好").len();
    println!("length is : {}",slen);
    

}
