fn main() {
    
    use std::collections::HashMap;
    let mut items: HashMap<String, String> = HashMap::new();

    items.insert(String::from("One"), String::from("Book"));
    items.insert(String::from("Two"), String::from("Keyboard"));
    items.insert(String::from("Three"), String::from("Sunglasses"));

    let Keyboard = items.get("Two");
    println!("{:?}", Keyboard);

    items.remove("Three");

    println!("{:?}", items.get("Three"));
}
