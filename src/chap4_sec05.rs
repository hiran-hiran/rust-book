pub fn main() {
    // exec_count()
    exec_coin()
}

struct Counter {
    value: i64,
}

impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }
    fn inc(&mut self) {
        self.value += 1;
        println!("value: {}", self.value);
    }
}

fn count(counter: Option<&mut Counter>) {
    match counter {
        None => println!("No counter"),
        Some(c) => c.inc(),
    }
}

fn exec_count() {
    let mut a = Counter::new();
    let b = None;
    count(Some(&mut a));
    count(Some(&mut a));
    count(b);
}

enum Coin {
    Coin1(isize),
    Coin5(isize),
    Coin10(isize),
    Coin50(isize),
    Coin100(isize),
    Coin500(isize),
}
impl Coin {
    fn calc_price(&self) -> isize {
        match *self {
            Coin::Coin1(v) => v,
            Coin::Coin5(v) => v * 5,
            Coin::Coin10(v) => v * 10,
            Coin::Coin50(v) => v * 50,
            Coin::Coin100(v) => v * 100,
            Coin::Coin500(v) => v * 500,
        }
    }
}
fn exec_coin() {
    let wallet = vec![
        Coin::Coin1(1),
        Coin::Coin5(2),
        Coin::Coin10(3),
        Coin::Coin50(4),
        Coin::Coin100(5),
        Coin::Coin500(6),
    ];
    let total = wallet.iter().fold(0, |sum, coin| sum + coin.calc_price());
    println!("total: {}", total);
}
