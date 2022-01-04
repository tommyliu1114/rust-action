fn main() {
    let mut s = String::from("wcc");
    s.push_str(" hwwh");
    println!("Hello, world! {}",s);
    //move 
    let s1 = s;
   // println!("s is {}",s);
   println!("s is {}",s1);
    //clone 仅仅针对heap上数据，栈上的数据stack不需要，廉价的创造
    let s3 = s1.clone();
    println!("s1 is {}, s3 is {}",s1,s3);
    //copy train 和 drop trait互斥；
    //copy-trait的类型：整形，浮点，bool，char，字段都为copy的tuple
}
