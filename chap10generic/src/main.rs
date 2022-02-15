fn main() {
    let number_list = [34,50,25,100,65];
    let result = largetst(&number_list);
    print!("the largest number is {}",result);
    let integ = Point{x: 5,y: 10};
    let flog = Point{x: 1.0,y: 2.0};
}


fn largetst<T: std::cmp::PartialOrd> (list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T,U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

enum Result < T,E> {
    Ok(T),
    Err(E),
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
