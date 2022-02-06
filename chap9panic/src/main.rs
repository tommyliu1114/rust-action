use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
fn main() {
    println!("Hello, world!");
    //panic!("this abort");
    let f = File::open("hello.txt");
    /*
    let f1 = match f {
        Ok(file) => file,
        Err(er) => panic!("error opening file {:?}",er)
    };
    */
    /*
    let f1 = match f {
        Ok(file) => file,
        Err(er) => match er.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e2) => panic!("error create file {:?}",e2),
            },
            other_error => panic!("error opening file")
        }
    };  
    */ 
    let f2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("error createting file {:?}",err)
            })
        }else{
            panic!("error openig file {:?}",error)
        }
    });
    let f3 = File::open("hello1.txt").expect("no exists file ");

}

fn read_username_from_file() -> Result<String,io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String,io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    
}

fn read_username_from_file3() -> Result<String,io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
    
}