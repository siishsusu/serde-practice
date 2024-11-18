use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    birthdate: String,
}

fn main() {
    // Створення екземпляра структури User
    let user = User {
        name: "Іван".to_string(),
        email: "ivan@example.com".to_string(),
        birthdate: "2000-01-01".to_string(),
    };

    // Серіалізація в JSON
    let json = serde_json::to_string(&user).expect("Помилка серіалізації");
    println!("Серіалізований JSON: {}", json);

    // Десеріалізація з JSON
    let deserialized_user: User = serde_json::from_str(&json).expect("Помилка десеріалізації");
    println!("Десеріалізований користувач: {:?}", deserialized_user);
}