pub fn count_garbage(stream: String) -> u32 {
    let mut garbage_characters = 0;
    let mut ignoring_next_char = false;
    let mut in_garbage = false;
    for character in stream.chars() {
        if ignoring_next_char {
            ignoring_next_char = false;
            continue;
        }
        if (character == '{' || character == '}') && !in_garbage {
            continue;
        } else if character == '<' && !in_garbage {
            in_garbage = true;
        } else if character == '>' && in_garbage {
            in_garbage = false;
        } else if character == '!' {
            ignoring_next_char = true;
        } else if in_garbage {
            garbage_characters += 1;
        }
    }
    garbage_characters
}
