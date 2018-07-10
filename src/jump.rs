pub fn jump(instructions: Vec<i32>) -> i32 {
    let mut instructions = instructions.clone();
    let mut steps = 0;
    let mut i = 0i32;
    loop {
        if i as usize >= instructions.len() || i < 0 {
            break;
        }
        let jump = instructions[i as usize];
        instructions[i as usize] = modifier(instructions[i as usize]);
        steps += 1;
        i = i + jump;
    }
    steps
}

fn modifier(value: i32) -> i32 {
    if value >= 3 {
        value - 1
    } else {
        value + 1
    }
}
