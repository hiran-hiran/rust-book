trait TreasureBox {
    // fn open(&self, key: i32) -> bool;
    fn open(&self, key: i32) -> bool {
        self.get_key_no() == key
    }
    fn check(&self);
    fn get_key_no(&self) -> i32;
}

struct JewelryBox {
    price: i32,
    key: i32,
}

impl TreasureBox for JewelryBox {
    // fn open(&self, key: i32) -> bool {
    //     self.key == key
    // }
    fn get_key_no(&self) -> i32 {
        self.key
    }

    fn check(&self) {
        println!("宝石箱の中には{}枚の金貨が入っています。", self.price);
    }
}

struct TrapBox {
    damage: i32,
    key: i32,
}
impl TreasureBox for TrapBox {
    // fn open(&self, _key: i32) -> bool {
    //     true // どの鍵でも開く
    // }
    fn check(&self) {
        println!("罠が仕掛けられていた。{}のダメージを受けた。", self.damage);
    }
    fn get_key_no(&self) -> i32 {
        self.key
    }
}

fn open_box(tbox: &impl TreasureBox, key: i32) {
    if !tbox.open(key) {
        println!("鍵が合わない。");
        return;
    }
    tbox.check();
}

pub fn main() {
    let box1 = JewelryBox { price: 100, key: 1 };
    let box2 = TrapBox { damage: 50, key: 2 };
    let box3 = JewelryBox { price: 200, key: 2 };
    let my_key = 2;

    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}
