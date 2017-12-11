pub fn hash(lengths: Vec<u32>) -> String {
    let mut list:Vec<u32> = (0..256).collect();
    let mut skip_size = 0;
    let mut current_position = 0;
    for _ in 0..64 {
        for length in lengths.clone() {
            list = reverse_sublist(list, current_position, length as usize);
            current_position = (current_position + length as usize + skip_size) % list.len();
            skip_size += 1;
        }
    }
    let dense_hash:Vec<u32> = list.chunks(16).map(|chunk| chunk.iter().fold(0u32, |acc, x| acc ^ x)).collect();
    dense_hash.iter().map(|x| format!("{:01$x}", x, 2)).collect::<Vec<String>>().join("")
}

fn reverse_sublist(list: Vec<u32>, index: usize, size: usize) -> Vec<u32> {
    let mut reversed:Vec<u32> = list.iter().cycle().skip(index).take(size).map(|x| *x).collect::<Vec<u32>>();
    reversed.reverse();
    let mut new_list = list.clone();
    let mut i = index;
    for number in reversed {
        new_list[i] = number;
        i += 1;
        if i == new_list.len() {
            i = 0;
        }
    }
    new_list
}
