pub fn hash(lengths: Vec<u32>) -> Vec<u32> {
    let mut list:Vec<u32> = (0..256).collect();
    let mut skip_size = 0;
    let mut current_position = 0;
    for length in lengths {
        list = reverse_sublist(list, current_position, length as usize);
        current_position = (current_position + length as usize + skip_size) % list.len();
        skip_size += 1;
    }
    list
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
