pub fn get_three_hits(map: &str, jump: &(u32, u32)) -> u64 {
    let mut threes = 0;
    let mut column = 0;

    let mut lines = map.lines();
    while let Some(line) = lines.next() {
        // Find three
        if line.chars().nth(column % line.len()) == Some('#') {
            threes += 1;
        }

        // Update column position
        column += jump.1 as usize;

        // Jump n rows if necessary
        if jump.0 - 1 > 0 {
            lines.nth((jump.0 - 2) as usize);
        }
    }

    return threes;
}
