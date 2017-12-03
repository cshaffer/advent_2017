pub fn checksum(numbers: Vec<Vec<u32>>, pair_finder: fn(Vec<u32>) -> (u32, u32), pair_transformation: fn(u32, u32) -> u32) -> u32 {
    let mut sum = 0;
    for row in numbers {
        let (max, min) = pair_finder(row);
        sum += pair_transformation(max, min);
    }
    sum
}

fn divisible(first: u32, second: u32) -> bool {
    if first == 0 || second == 0 {
        return false
    }
    if first > second {
        first % second == 0
    } else {
        second % first == 0
    }
}

pub fn divisible_values(row: Vec<u32>) -> (u32, u32) {
    let mut i = 0;
    let mut j = 1;
    while i < row.len() {
        if divisible(row[i], row[j]) {
            return (row[i], row[j])
        }
        j += 1;
        if j == row.len() {
            i += 1;
            j = i + 1;
        }
    }
    (0, 0)
}

pub fn extreme_values(row: Vec<u32>) -> (u32, u32) {
    let mut max = 0;
    let mut min = 0;
    for number in row {
        if number > max  || max == 0 {
            max = number;
        }
        if number < min  || min == 0 {
            min = number;
        }
    }
    (max, min)
}

pub fn quotient(first: u32, second: u32) -> u32 {
    if first == 0 || second == 0 {
        return 0
    }

    if first > second {
        first / second
    } else {
        second / first
    }
}

pub fn difference(first: u32, second: u32) -> u32 {
    if first > second {
        first - second
    } else {
        second - first
    }
}
