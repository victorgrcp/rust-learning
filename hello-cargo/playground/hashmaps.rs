use std::collections::HashMap;

fn print_value(reviews: &HashMap<String, String>, key: &String) {
    println!("Key -> {}, Value -> {:?}", key, reviews.get(key));
}

fn remove_pair(reviews: &mut HashMap<String, String>, obsolete: &str) {
    // Remove book review
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);
}

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

    let key_1: String = String::from("Programming in Rust");
    println!("{:#?}", reviews);
    print_value(&reviews, &key_1);
    remove_pair(&mut reviews, "Ancient Roman History");
    println!("{:#?}", reviews);
}