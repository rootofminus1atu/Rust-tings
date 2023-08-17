pub fn display() {
    let n = 20;

    let table = generate_table(n);

    println!("Multiplication table up to {}:\n{}", n, table)
}

fn generate_table(n: u32) -> String {
    let mut table = String::new();
    let indent = get_indent_size(n);

    for i in 1..=n {
        for j in 1..=n {
            table.push_str(&format!("{:width$}", i * j, width=indent));
        }
        table.push('\n')
    }

    table
}

fn get_indent_size(n: u32) -> usize {
    let biggest = n * n;
    let biggest_len = biggest.to_string().len() as usize;

    biggest_len + 1
}