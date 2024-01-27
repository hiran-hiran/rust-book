pub fn caesar(text: &str, shift: i32) {
    let code_a = 'a' as i32;
    let code_z = 'z' as i32;

    let mut result = String::new();
    for ch in text.chars() {}

    text.chars().for_each(|ch| {
        let mut code = ch as i32;
        if code_a <= code && code <= code_z {
            code = (code - code_a + shift + 26) % 26 + code_a;
        }

        result.push((code as u8) as char);
    });

    println!("{result}");
}
