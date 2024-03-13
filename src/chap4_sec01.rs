struct Body {
    height: f64, // 身長cm
    weight: f64, // 体重kg
}
impl Body {
    fn calc_bmi(&self) -> f64 {
        let h = self.height / 100.0;
        // BMIの計算をして値を返す
        self.weight / h.powf(2.0)
    }
    // 乖離率を計算するメソッド --- (*4)
    fn calc_per(&self) -> f64 {
        self.calc_bmi() / 22.0 * 100.0
    }
}

struct Person {
    name: String,
    age: i32,
}
impl Person {
    // &self以外を引数に取る関数は::でアクセスできる
    fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }
}
pub fn main() {
    // let man = Body {
    //     height: 170.0,
    //     weight: 60.0,
    // };
    // println!("BMI={:.2}", man.calc_bmi());
    // println!("乖離率={:.1}%", man.calc_per());

    let taro = Person::new("太郎".to_string(), 18);
    println!("{}さんは{}才。", taro.name, taro.age);
}
