use std::io::stdin;

struct Body {
    weight: f64,
    height: f64,
}

pub fn main() {
    let height = input("身長を入力してください(cm): ");
    let weight = input("体重を入力してください(kg): ");

    let body = Body { weight, height };

    println!("あなたのBMIは、{}", calc_bmi(&body));
}

fn calc_bmi(body: &Body) -> f64 {
    body.weight / (body.height / 100.0).powf(2.0)
}

fn input(prompt: &str) -> f64 {
    println!("{prompt}");

    let mut s = String::new();
    stdin().read_line(&mut s).expect("入力エラー");
    s.trim().parse().expect("数値を入力してください")
}
