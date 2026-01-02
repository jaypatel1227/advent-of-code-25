fn read_input() -> Vec<Vec<char>> {
    std::fs::read_to_string("inputs/d4.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn d4p1() -> String {
    let input = read_input();
    let mut total: u128 = 0;
    let mut neighbors: [i8; 8] = [0; 8];
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] != '@' {
                continue;
            }
            fetch_neighbors(
                &input,
                &mut neighbors,
                x as i64,
                y as i64,
                (input.len(), input[y].len()),
            );
            if neighbors.iter().sum::<i8>() < 4 {
                total += 1;
            }
        }
    }
    total.to_string()
}

pub fn d4p2() -> String {
    let mut input = read_input();
    let mut total: u128 = 0;
    let mut to_remove: Vec<(usize, usize)> = vec![];
    while true {
        let mut neighbors: [i8; 8] = [0; 8];
        let mut point: (usize, usize) = (0, 0);
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] != '@' {
                    continue;
                }
                fetch_neighbors(
                    &input,
                    &mut neighbors,
                    x as i64,
                    y as i64,
                    (input.len(), input[y].len()),
                );
                if neighbors.iter().sum::<i8>() < 4 {
                    total += 1;
                    to_remove.push((x, y));
                }
            }
        }

        if to_remove.len() == 0 {
            break;
        }
        while to_remove.len() > 0 {
            point = to_remove.pop().unwrap();
            input[point.1][point.0] = '.';
        }
    }
    total.to_string()
}

pub fn fetch_neighbors(
    input: &Vec<Vec<char>>,
    neighbors: &mut [i8; 8],
    x: i64,
    y: i64,
    size: (usize, usize),
) {
    let mut count = 0;
    neighbors.fill(0);
    for a in x - 1..=x + 1 {
        for b in y - 1..=y + 1 {
            if a < 0 || b < 0 {
                continue;
            }
            if a >= size.1 as i64 || b >= size.0 as i64 {
                continue;
            }
            if a == x && b == y {
                continue;
            }
            neighbors[count] = (input[b as usize][a as usize] == '@') as i8;
            count += 1;
        }
    }
}
