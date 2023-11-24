use chrono::{NaiveDate, Duration};
use rand::seq::SliceRandom;
use rand;
use rand::Rng;

pub fn random_choice<'a, T>(items: &'a [T]) -> Option<&'a T> {
    let mut rng = rand::thread_rng();
    items.choose(&mut rng)
}

pub fn random_int(lower_bound: i32, upper_bound: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower_bound..=upper_bound)
    
}


pub fn random_date(start: NaiveDate, end: NaiveDate) -> NaiveDate {
    let mut rng = rand::thread_rng();

    let days = (end - start).num_days();
    let random_days = rng.gen_range(0..=days); 
    
    start + Duration::days(random_days)
}



pub fn divide_with_strlen(list: Vec<String>, str_limit: i32) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut current_page = Vec::new();
    let mut current_length = 0;

    for item in list {
        let item_length = item.len() as i32;
        if current_length + item_length > str_limit {
            result.push(std::mem::take(&mut current_page));
            current_length = 0;
        }

        current_page.push(item);
        current_length += item_length;
    }

    if !current_page.is_empty() {
        result.push(current_page);
    }

    result
}