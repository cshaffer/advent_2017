use std::io;

fn sum_of_matching_digits(v: Vec<u32>) -> u32 {
    let mut sum = 0;
    for i in 0..v.len() {
        if (i == v.len() - 1 && v[i] == v[0]) ||
           (i < v.len() - 1 && v[i] == v[i + 1]) {
            sum += v[i];
        }
    }
    sum
}

fn parse_input(input: String) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let digits = parse_input(input);
    let sum = sum_of_matching_digits(digits);
    println!("Sum: {}", sum);
}
