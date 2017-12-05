mod checksum;
mod io;
mod inverse_captcha;
mod jump;
mod passphrase;
mod spiral_memory;

fn inverse_captcha_main() {
    let sum = inverse_captcha::inverse_captcha(io::read_one_line());
    println!("Sum: {}", sum);
}

fn checksum_main() {
    let numbers = io::read_int_arrays_from_file("input/2_puzzle.txt".to_string());
    let sum = checksum::checksum(numbers, checksum::divisible_values, checksum::quotient);
    println!("Sum: {}", sum);
}

fn spiral_memory_main(input: u32) {
    let value = spiral_memory::spiral_sum(input);
    println!("Value: {}", value);
}

fn passphrase_main() {
    let passphrases = io::read_string_arrays_from_file("input/4_puzzle.txt".to_string());
    let count = passphrase::valid_count(passphrases);
    println!("Valid count: {}", count);
}

fn jump_main() {
    let instructions = io::read_int_lines_from_file("input/5_puzzle.txt".to_string());
    jump::jump(instructions);
}

fn main() {
    jump_main();
}
