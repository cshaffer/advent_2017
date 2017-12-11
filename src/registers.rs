use std::collections::HashMap;

fn parse_comparison(operator: String) -> (fn(&i32, &i32) -> bool) {
    if operator == ">".to_string() {
        i32::gt
    } else if operator == "<".to_string() {
        i32::lt
    } else if operator == "==".to_string() {
        i32::eq
    } else if operator == ">=".to_string() {
        i32::ge
    } else if operator == "<=".to_string() {
        i32::le
    } else {
        i32::ne
    }
}

fn parse_operator(operator: String) -> (fn(i32, i32) -> i32) {
    if operator == "inc".to_string() {
        i32::saturating_add
    } else {
        i32::saturating_sub
    }
}

pub fn run_instructions(input: Vec<String>) -> HashMap<String, i32> {
    let instructions:Vec<Vec<String>> = input.iter().map(|s| s.split_whitespace().map(|x| String::from(x)).collect()).collect();
    let mut registers = HashMap::new();
    for instruction in instructions {
        let register = instruction[0].clone();
        let value:i32 = if !registers.contains_key(&register) {
            0
        } else {
            *registers.get(&register).unwrap()
        };
        let conditional_register = instruction[4].clone();
        let conditional_value:i32 = if !registers.contains_key(&conditional_register) {
            0
        } else {
            *registers.get(&conditional_register).unwrap()
        };
        let comparison_value = i32::from_str_radix(&instruction[6], 10).unwrap();
        if parse_comparison(instruction[5].clone())(&conditional_value, &comparison_value) {
            let operand = i32::from_str_radix(&instruction[2], 10).unwrap();
            let new_value = parse_operator(instruction[1].clone())(value, operand);
            registers.insert(register, new_value);
        }
    }
    registers
}

pub fn find_largest_value(registers: HashMap<String, i32>) -> i32 {
    let mut max = i32::min_value();
    for val in registers.values() {
        if val > &max {
            max = *val;
        }
    }
    max
}
