pub fn calculate_score(stream: String) -> u32 {
    let mut score = 0;
    let mut current_depth = 0;
    let mut ignoring_next_char = false;
    let mut in_garbage = false;
    for character in stream.chars() {
        if ignoring_next_char {
            ignoring_next_char = false;
            continue;
        }
        if character == '{' && !in_garbage {
            current_depth += 1;
            score += current_depth;
        } else if character == '<' && !in_garbage {
            in_garbage = true;
        } else if character == '>' && in_garbage {
            in_garbage = false;
        } else if character == '}' && !in_garbage {
            current_depth -= 1;
        } else if character == '!' {
            ignoring_next_char = true;
        }
    }
    score
}
