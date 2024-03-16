fn add<T>(a: T, b: T) -> T {
    b // traitを指定しなくてもOK
}
// fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }
// fn add<T>(a: T, b: T) -> T
// where
//     T: std::ops::Add<Output = T>,
// {
//     a + b
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn main() {
    // println!("{}", add(1, 2));
    // println!("{}", add(22.2, 2.2));
    // println!("{}", add('a', 'b'));

    let pt1 = Point { x: 1, y: 2 };
    let pt2 = Point { x: "aaa", y: "bbb" };
    println!("{:?}", pt1);
    println!("{:?}", pt2);
}
