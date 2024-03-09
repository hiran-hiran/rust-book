pub fn main() {
    let string = "隣の客はよく柿食う客だ";

    // println!("{}", &string[0..3]); // 3byteずつ探すのはダルい
    // for (i, s) in string.chars().enumerate() {
    //     println!("{s}, {i}");
    // }

    let result = string.find('あ');
    match result {
        Some(i) => println!("{}バイトめに見つかりました", i),
        None => println!("見つかりませんでした"),
    }
}
