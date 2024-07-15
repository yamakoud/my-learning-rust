use std::io;

#[derive(Debug)]
struct Contact {
    name: String,
    phone_number: String,
}

struct PhoneBook {
    contacts: Vec<Contact>,
}

impl PhoneBook {
    fn new() -> Self {
        PhoneBook {
            contacts: Vec::new(),
        }
    }

    fn add_contact(&mut self, name: String, phone_number: String) {
        // TODO: 新しい連絡先を追加する処理を実装
        let contact = Contact{
            name: name,
            phone_number: phone_number,
        };
        self.contacts.push(contact)
    }

    fn search_contact(&self, name: &str) -> Option<&Contact> {
        // TODO: 名前で連絡先を検索する処理を実装
        self.contacts.iter().find(|contact| contact.name == name)
    }

    fn display_all_contacts(&self) {
        // TODO: すべての連絡先を表示する処理を実装
        println!("--- all contact ---");
        for contact in &self.contacts {
            println!("name: {a}\nphone_number: {b}", a = contact.name, b = contact.phone_number);
        }
        println!("-------------------");
    }
}

pub fn main() {
    let mut phone_book = PhoneBook::new();

    loop {
        println!("\n電話帳アプリケーション");
        println!("1. 連絡先を追加");
        println!("2. 連絡先を検索");
        println!("3. すべての連絡先を表示");
        println!("4. 終了");
        println!("選択してください（1-4）:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("入力の読み取りに失敗しました");

        match choice.trim() {
            "1" => {
                // TODO: 連絡先追加の処理を実装
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                let mut phone_number = String::new();
                std::io::stdin().read_line(&mut phone_number).unwrap();
                let phone_number = phone_number.trim().to_string();

                phone_book.add_contact(name, phone_number);
            }
            "2" => {
                // TODO: 連絡先検索の処理を実装
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                phone_book.search_contact(&name);
            }
            "3" => phone_book.display_all_contacts(),
            "4" => break,
            _ => println!("無効な選択です。1から4の数字を入力してください。"),
        }
    }

    println!("プログラムを終了します。");
}