#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn calc_test1() {
        assert_eq!(100 * 2, 200);
        assert_eq!((1 + 2) * 3, 9);
    }

    #[test]
    fn calc_test2() {
        // 簡単な計算のテストその2
        assert_eq!(2 * 3, 6);
        // わざとテストに失敗してみる
        // assert_eq!(2 * 3, 7);
    }
}

mod tests_array {
    #[test]
    fn array_test() {
        let a1 = [100, 200, 300];
        let a2 = [100, 200, 300];
        assert_eq!(a1, a2);

        let a3 = ["リンゴ".to_string(), "バナナ".to_string()];
        let a4 = [String::from("リンゴ"), String::from("バナナ")];
        assert_eq!(a3, a4);
    }
}
mod tests_vec {
    #[test]
    fn vec_test() {
        let v1 = vec!["apple", "banana", "mango"];
        let mut v2: Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banana");
        v2.push("mango");
        assert_eq!(v1, v2);
    }
}

#[derive(Debug, PartialEq)]
struct GItem {
    name: String,
    price: i64,
}

mod tests_struct {
    use super::*;

    #[test]
    fn item_test() {
        // 構造体を初期化 --- (*3)
        let apple1 = GItem {
            name: String::from("リンゴ"),
            price: 2400,
        };
        let mut apple2 = GItem {
            name: "リンゴ".to_string(),
            price: 0,
        };
        apple2.price = 2400;

        // 構造体のフィールドを比較 --- (*4)
        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);

        // 構造体全体を直接比較 --- (*5)
        assert_eq!(apple1, apple2);
    }
}
