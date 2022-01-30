

fn main() {

    let mut v: Vec<i32> = Vec::new();
    let v1  = vec![1,2,3];
    v.push(32);
    v.push(25);
    println!("Hello, world!, {}",v1[0]);
    // yin yong
    match v.get(2) {
        Some(kk) => println!("the third is {} ",kk),
        None => println!("no value")
    }
    for i in &v1 {
        println!("the number is {}",i);
    }
    for kk in &mut v {
        *kk = *kk + 1;
        println!("the mut is {}",kk);
    }

    let row = vec![
        SpreadsheetCell::Float(3.2),
        SpreadsheetCell::Int(31),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}



