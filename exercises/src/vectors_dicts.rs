use std::collections::HashMap;

pub fn display() {
    println!("Vectors and Dictionaries");
    let mut numbers = vec![3, 1, 2, 0, -1, 2, 1, 2];

    let median = median(&mut numbers);

    numbers.sort();
    println!("The median of {:?} is {}", numbers, median);
    println!("The mode of {:?} is {}", numbers, mode(&numbers));


    let words = vec!["first", "apple", "second", "banana", "third", "pear"];
    println!("Piglatin of {:?}", words);
    let pilating_words: Vec<String> = words.iter().map(|&w| piglatin(w)).collect();
    println!("is {:?}", pilating_words);
}

fn median(v: &mut Vec<i32>) -> f64 {
    v.sort();

    let mid = v.len() / 2;

    if v.len() % 2 == 0 {
        let left = v[mid - 1] as f64;
        let right = v[mid] as f64;
        (left + right) / 2.0
    } else {
        v[mid] as f64
    }
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut occurances: HashMap<i32, usize> = HashMap::new();

    for &i in v {
        let count = occurances.entry(i).or_insert(0);
        *count += 1;
    }

    occurances.iter().max_by_key(|&(_, count)| count).map(|(&val, _)| val).unwrap()
}

fn piglatin(s: &str) -> String {
    let mut chars = s.chars();

    let first_char = match chars.next() {
        Some(c) => c,
        None => return String::from(s),
    };

    if is_vowel(first_char){
        format!("{}-hay", s)
    } else {
        format!("{}-{}ay", chars.as_str(), first_char)
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}