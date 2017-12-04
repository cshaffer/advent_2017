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
