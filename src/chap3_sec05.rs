pub fn main() {
    // let s = "こんにちは";
    // let e = "hello!";
    // println!("{:?}", s.chars().nth(1).unwrap());
    // println!("{:?}", s.bytes().nth(1).unwrap());
    // println!("{:?}", &s[0..3]); //UTF-8で日本語は3バイト, 英語は1バイト
    // println!("{}", &e[0..1]);

    echo("こんにちは");
    echo("こんばんは");

    let s = String::from("おはよう");
    echo(&s);
}

fn echo(s: &'static str) {
    println!("{}", s);
}
