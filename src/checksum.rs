pub fn checksum(numbers: Vec<Vec<u32>>, pair_finder: fn(Vec<u32>) -> (u32, u32), pair_transformation: fn(u32, u32) -> u32) -> u32 {
    let mut sum = 0;
    for row in numbers {
        let (max, min) = pair_finder(row);
        sum += pair_transformation(max, min);
    }
    sum
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

pub fn difference(first: u32, second: u32) -> u32 {
    if first > second {
        first - second
    } else {
        second - first
    }
}
