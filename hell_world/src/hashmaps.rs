use std::collections::HashMap;

pub fn display() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Yellow"), 20);  // overwrites "Yellow"

    println!("Scores: {:?}", scores);

    let ten = scores.get("Blue").unwrap();

    println!("Ten: {}", ten);



    scores.entry(String::from("Red")).or_insert(70);  // inserts "Red"
    scores.entry(String::from("Red")).or_insert(100);  // does not overwrite "Red"

    println!("Scores: {:?}", scores);


    let text = "hello world wonderful world";
    word_frequency_counter(text);
}

fn word_frequency_counter(text: &str) {

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let entry = map.entry(word);
        let count = entry.or_insert(0);
        *count += 1;
    }

    println!("Word frequency: {:?}", map);
}