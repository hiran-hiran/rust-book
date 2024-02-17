struct Item(String, i32);

pub fn main() {
    let banana = Item("バナナ".to_string(), 100);
    let apple = Item(String::from("りんご"), 150);
    let kiwi = Item("キウイ".to_string(), 200);

    let items = vec![banana, apple, kiwi];
    let total = print_and_sum_items(&items);

    println!("合計金額: {total}");
}

fn print_and_sum_items(items: &Vec<Item>) -> i32 {
    let mut result = 0;

    for item in items {
        print_tuple(item);
        result += item.1;
    }

    result
}

fn print_tuple(item: &Item) {
    println!("{}は{}円です", item.0, item.1);
}
