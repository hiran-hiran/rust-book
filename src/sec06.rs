pub fn kuku() {
    // for y in 1..10 {
    //     for x in 1..10 {
    //         print!("{:3},", x * y);
    //     }
    //     println!("");
    // }

    for y in 1..10 {
        let result = (1..10)
            .map(|x| format!("{:3}", x * y))
            .collect::<Vec<String>>()
            .join(",");

        println!("{}", result);
    }
}
