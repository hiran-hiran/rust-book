pub fn main() {
    let word = String::from("新しいテキスト");

    let text = show_message(word);

    println!("main: {}", text);
}

fn show_message(word: String) -> String {
    println!("show_message: {}", word);
    word
}
