use std::io;

#[derive(Debug)]
struct Product {
    name: String,
    quantity: u32,
}

#[derive(Debug)]
enum InventoryError {
    ProductNotFound,
    InvalidInput,
}

struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory { products: Vec::new() }
    }

    fn add_product(&mut self, name: String, quantity: u32) {
        // TODO: 商品を追加する処理を実装
        let product = Product{
            name: name,
            quantity: quantity,
        };
        self.products.push(product);
    }

    fn update_quantity(&mut self, name: &str, quantity: u32) -> Result<(), InventoryError> {
        // TODO: 在庫数を更新する処理を実装
        if let Some(product) = self.products.iter_mut().find(|product| product.name==name) {
            product.quantity = quantity;
            Ok(())
        } else {
            Err(InventoryError::ProductNotFound)
        }

    }

    fn display_inventory(&self) {
        // TODO: 在庫リストを表示する処理を実装
        for product in &self.products {
            println!("name: {name}, quantity: {quantity}", name = product.name, quantity = product.quantity)
        }
    }
}

pub fn main() {
    let mut inventory = Inventory::new();

    loop {
        println!("\n在庫管理システム");
        println!("1. 商品を追加");
        println!("2. 在庫数を更新");
        println!("3. 在庫リストを表示");
        println!("4. 終了");
        println!("選択してください（1-4）:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("入力の読み取りに失敗しました");

        match choice.trim() {
            "1" => {
                let mut name = String::new();
                let mut quantity = String::new();
                println!("type name: ");
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                println!("type quantity: ");
                std::io::stdin().read_line(&mut quantity).unwrap();
                let quantity: u32 = quantity.trim().parse().unwrap();


                inventory.add_product(name.to_string(), quantity)
            }
            "2" => {
                let mut name = String::new();
                let mut quantity = String::new();
                println!("type name: ");
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                println!("type quantity: ");
                std::io::stdin().read_line(&mut quantity).unwrap();
                let quantity: u32 = quantity.trim().parse().unwrap();

                inventory.update_quantity(name, quantity).unwrap()
            }
            "3" => inventory.display_inventory(),
            "4" => break,
            _ => println!("無効な選択です。1から4の数字を入力してください。"),
        }
    }

    println!("プログラムを終了します。");
}
