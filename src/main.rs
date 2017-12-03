mod checksum;
mod io;
mod inverse_captcha;

fn inverse_captcha_main() {
    let sum = inverse_captcha::inverse_captcha(io::read_one_line());
    println!("Sum: {}", sum);
}

fn checksum_main() {
    let numbers = io::read_int_arrays_from_file("input/2.txt".to_string());
    let sum = checksum::checksum(numbers);
    println!("Sum: {}", sum);
}

fn main() {
    inverse_captcha_main();
}
