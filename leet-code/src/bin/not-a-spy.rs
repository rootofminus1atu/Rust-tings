
fn main() {
    let area = simpsons_area(|x| (9.0 - x.powi(2)).sqrt(), 0.0, 3.0, 10);

    println!("Area: {}", area);
}

fn simpsons_area<F>(f: F, lower: f64, upper: f64, steps: i32) -> f64 
where F: Fn(&f64) -> f64
{
    let step_len = (upper - lower) / steps as f64;

    let fs = (0..=steps)
        .map(|p| p as f64 * step_len)
        .map(|p| f(&p))
        .collect::<Vec<_>>(); 

    let first = fs[0];
    let last = fs[fs.len() - 1];

    let odds: f64 = fs.iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, elem)| elem)
        .sum();

    let evens: f64 = fs.iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0 && *i != 0 as usize && *i != fs.len() - 1 as usize)
        .map(|(_, elem)| elem)
        .sum();


    let area = (step_len / 3.0) * (first + last + 4.0 * odds + 2.0 * evens);

    area
}
