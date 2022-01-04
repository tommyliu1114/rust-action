fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("the value of x is {} ",x);
    let mut y = 6;
    println!("the value of y is {}",y);
    y = 100 ;
    println!("the value of y is {}",y);
    //let xf = 2.0;
    //let yf: f32 = 4.1;
    //let y: char = '😄';
    //tuple 多类型，多个值，长度固定
    let c_tuple = (500,6.4,'a');
    println!("{},{},{}",c_tuple.0,c_tuple.1,c_tuple.2);
    let (f1,f2,f3) = c_tuple;
    println!("{},{},{}",f1,f2,f3);
    //数组类型，但类型，多个值，长度固定,存放在栈上
    let a: [i32;5] = [1,2,3,45,6];
    let b = [3;5];
    println!("arry is {:?},{:?}",a,b);
    println!("return {} ",another_fn(55));

    //loop ,while ,for
    let mut counter = 0;
    let l_r = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("value is {}",l_r);
    let mut number = 3;
    while number > 0 {
        number = number - 1;
    }
    println!("final num is {}",number);
    //for 
    let cc = [10,20,30,40,50];
    //let mut index = 0;
    for elem in cc.iter() {
        println!("value is : {}",elem);
    }

}

fn another_fn(x: i32) -> i32{
    if x >  5 {
        println!("bigger value is : {}",x);
    }else {
        println!("small value is : {}",x);
    }
    
    return x;
}