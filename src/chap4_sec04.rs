pub fn main() {
    // error_iter()
    let prime_iter = PrimeIterator::new(12);
    for p in prime_iter {
        println!("{}", p);
    }
}

// 問題があるプログラム
fn error_iter() {
    // 配列に文字列を代入
    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
        "Tomato".to_string(),
    ];

    // 配列を繰り返し画面に表示したいが...
    // for a in &array {
    // ここで所有権が移動する
    //     println!("{}", a);
    // }

    for a in array.iter() {
        println!("{}", a);
    }
    // これでもいい
    // for a in &array {
    //     println!("{}", a);
    // }
    println!("len={}", array.len()); // ←エラー
}

struct PrimeIterator {
    n: u8,
}
impl PrimeIterator {
    // 関連関数 (associated functions)
    fn new(n: u8) -> Self {
        PrimeIterator { n }
    }
    // メソッド
    fn is_prime(&self) -> bool {
        for i in 2..self.n {
            if self.n % i == 0 {
                return false;
            }
        }
        true
    }
}
impl Iterator for PrimeIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n += 1;
            if std::u8::MAX == self.n {
                return None;
            }
            if self.is_prime() {
                return Some(self.n);
            }
        }
    }
}
