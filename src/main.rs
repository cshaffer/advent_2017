use std::io;

fn sum_of_matching_digits(v: Vec<u32>, offset: usize) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    let mut j = offset + i;
    while i < v.len() {
        if v[i] == v[j] {
            sum += v[i];
        }
        i += 1;
        j += 1;
        if j == v.len() {
            j = 0;
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
    let offset = digits.len() / 2;
    let sum = sum_of_matching_digits(digits, offset);
    println!("Sum: {}", sum);
}
