use std::collections::HashMap;

const DATA: &str = "C,C,A,A,A,B,B,B,C,B,A,C,B,A,C,C,C,C,A";

pub fn main() {
    // vote_count();
    tsuki_map();
}

fn tsuki_map() {
    let tsuki = [
        "睦月",
        "如月",
        "弥生",
        "卯月",
        "皐月",
        "水無月",
        "文月",
        "葉月",
        "長月",
        "神無月",
        "霜月",
        "師走",
    ];

    let mut data: HashMap<&str, usize> = HashMap::new();

    tsuki.iter().enumerate().for_each(|(i, x)| {
        // println!("{:?}", x);
        data.insert(x, i + 1);
    });

    println!("{:?}", data);
}

fn vote_count() {
    let mut data: HashMap<&str, i32> = HashMap::new();
    data.insert("A", 0);
    data.insert("B", 0);
    data.insert("C", 0);

    DATA.split(',').for_each(|x| {
        data.insert(x, data[x] + 1);
    });

    println!("{:?}", data);
}
