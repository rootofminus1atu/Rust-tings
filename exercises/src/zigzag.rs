pub fn display() {
    let text = "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let rows = 9;
    let split = zigzag(text, rows);

    for row in &split {
        println!("{}", row);
    }

    print_zigzag_style(&split, rows);
}

fn print_zigzag_style(list: &Vec<String>, rows: usize) {
    let gap_size = rows - 2;
    let mut result_str = String::new();

    let (first, last) = (0 as usize, rows - 1);

    for (i, row_str) in list.iter().enumerate() {
        let modified_str = if i == first || i == last {
            let gap = " ".repeat(gap_size);
            insert_str(row_str, &gap)
        } else {
            let left_gap = " ".repeat(gap_size - i);
            let right_gap = " ".repeat(i - 1);
            insert_alternating_strs(row_str, &left_gap, &right_gap)
        };

        result_str.push_str(&modified_str);
        result_str.push('\n');
    }

    println!("{}", result_str);
}

fn insert_alternating_strs(word: &str, insert1: &str, insert2: &str) -> String {
    word.chars().enumerate().map(|(i, c)| {
        if i % 2 == 0 {
            format!("{}{}", c, insert1)
        } else {
            format!("{}{}", c, insert2)
        }
    }).collect()
}

fn insert_str(word: &str, insert: &str) -> String {
    word.chars().enumerate().map(|(_, c)| {
        format!("{}{}", c, insert)
    }).collect()
}


fn zigzag(text: &str, rows: usize) -> Vec<String> {
    let mut tracker = vec![String::new(); rows];

    for i in 0..text.len() {
        let row = calculate_row(rows, i);
        tracker[row].push(text.chars().nth(i).unwrap());
    }

    println!("{:?}", tracker);

    tracker
}

fn calculate_row(total_rows: usize, index: usize) -> usize {
    let cycle_length = total_rows * 2 - 2;
    let normalized_index = index % cycle_length;
    
    if normalized_index >= total_rows {
        return cycle_length - normalized_index;
    }
    
    normalized_index
}