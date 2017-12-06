use std::collections::HashMap;

pub fn realloc(memory: Vec<u32>) -> u32 {
    let mut memory = memory.clone();
    let mut current_step:u32 = 0;
    let mut seen_states = HashMap::new();

    loop {
        if seen_states.contains_key(&memory) {
            break;
        }
        seen_states.insert(memory.clone(), current_step);
        let max_index = find_index_of_max_value(memory.clone());
        let blocks = memory[max_index];
        memory[max_index] = 0;
        let next_index = if max_index + 1 == memory.len() {
            0
        } else {
            max_index + 1
        };

        memory = redistribute(memory, blocks, next_index);
        current_step += 1;
    }

    current_step - seen_states.get(&memory).unwrap()
}

fn redistribute(memory: Vec<u32>, blocks: u32, next_index: usize) -> Vec<u32> {
    let mut memory = memory.clone();
    if blocks == 0 {
        return memory
    } else {
        memory[next_index] += 1;
        let new_next_index = if next_index + 1 == memory.len() {
            0
        } else {
            next_index + 1
        };
        redistribute(memory, blocks - 1, new_next_index)
    }
}

fn find_index_of_max_value(memory: Vec<u32>) -> usize {
    let mut max_index = 0;
    let mut max_value = memory[0];
    let mut index = 1;
    while index < memory.len() {
        if memory[index] > max_value {
            max_index = index;
            max_value = memory[index];
        }
        index += 1;
    }
    max_index
}
