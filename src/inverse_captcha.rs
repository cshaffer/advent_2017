pub fn inverse_captcha(digits: String) -> u32 {
    sum_of_matching_offset_digits(&parse_string_of_digits(digits), length_over_two)
}

fn sum_of_matching_offset_digits(v: &Vec<u32>, offset_function: fn(&Vec<u32>) -> usize) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    let mut j = offset_function(v) + i;
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

fn parse_string_of_digits(input: String) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn just_one<A>(_: &Vec<A>) -> usize {
    1
}

fn length_over_two<A>(vector: &Vec<A>) -> usize {
    vector.len() / 2
}
