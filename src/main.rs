// mod sec06;
// mod sec09;
mod sec10;
fn main() {
    // sec06::kuku()
    // sec09::caesar("hoge", 3);
    // sec09::caesar("krjh", -3);

    let mut primes = [0; 100];
    sec10::get_primes(&mut primes);
}
