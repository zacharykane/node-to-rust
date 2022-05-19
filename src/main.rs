use std::{collections::HashMap, fs::read_to_string};

fn main() {
    // a borrowed string slice
    // not a String
    let greeting = "hello world";
    greet(greeting);

    // a String!
    let source = read_to_string("./Cargo.toml").unwrap();
    let mut files = HashMap::new();
    // using the raw source will error
    // without being Cloned or Copied we've given over source
    // to the first insert() and are not able to give it away again
    files.insert("Cargo", source.clone());
    files.insert("Cargo2", source);

    // makes two references to the original files
    let files_ref = &files;
    let files_ref2 = &files;
    print_borrowed_map(files_ref);
    print_borrowed_map(files_ref2);

    let mut map = HashMap::new();
    map.insert("key1", "Value 1");
    map.insert("key2", "Value 2");
    println!("{}", map.get("key3").unwrap_or(&" default "));

    everything_is_an_expression();
}

fn greet(greeting: &str) {
    println!("{}", greeting);
}

fn print_borrowed_map(map: &HashMap<&str, String>) {
    println!("{:#?}", map);
    println!("{:#?}", map.get("Cargo"));
    println!("{:#?}", map.get("CarNO"));
}

fn everything_is_an_expression() {
    // such as control blocks!
    let apples = 12;
    // implicit returns let each conditional block
    // return their last line (no semicolon)
    let message = if apples > 10 {
        "Lots of apples!"
    } else if apples > 5 {
        "Some apples."
    } else {
        "Not many apples really."
    };

    println!("{}", message);
}
