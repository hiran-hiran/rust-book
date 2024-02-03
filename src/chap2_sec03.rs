use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn main() {
    // array();
    vector();
}

fn array() {
    let mut numbers = [0; 99];
    for i in 1..=99 {
        numbers[i - 1] = i
    }

    let mut rng = thread_rng();
    numbers.shuffle(&mut rng);

    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 {
                print!(" *");
            } else {
                print!("{:3}", numbers[i]);
            }
        }
        println!("");
    }
}

fn vector() {
    let mut numbers = (1..=99).collect::<Vec<u32>>();

    let mut rng = thread_rng();
    numbers.shuffle(&mut rng);

    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 {
                print!(" *");
            } else {
                print!("{:3}", numbers[i]);
            }
        }
        println!("");
    }
}
