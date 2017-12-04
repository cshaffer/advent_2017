use std::collections::HashMap;

pub fn manhattan_distance(destination: u32) -> u32 {
    let (x, y) = travel_spiral(destination);
    let distance = (x.abs() + y.abs()) as u32;
    distance
}

fn travel_spiral(destination: u32) -> (i32, i32) {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut dx = 0i32;
    let mut dy = -1i32;
    let mut position = 0u32;

    loop {
        position += 1;
        if position == destination {
            return (x, y)
        }
        if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }
        x += dx;
        y += dy;
    }
}

pub fn spiral_sum(input: u32) -> u32 {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut dx = 0i32;
    let mut dy = -1i32;
	let mut grid = HashMap::new();
    let coords = [(1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)];

    loop {
        let mut next_value = 0;
        for offset in coords.iter() {
            let &(ox, oy) = offset;
            let value = grid.get(&(x + ox, y + oy));
            if let Some(i) = value {
                next_value += i;
            }
        }
        if next_value > input {
            return next_value
        }
        if (x, y) == (0, 0) {
            grid.insert((0, 0), 1);
        } else {
            grid.insert((x, y), next_value);
        }
        if (x == y) || (x < 0 && x == -y) || (x > 0 && x == 1-y) {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }
        x += dx;
        y += dy;
    }
}
