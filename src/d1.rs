fn read_inputs() -> Vec<String> {
    std::fs::read_to_string("inputs/d1.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn d1p1() -> String {
    let inputs = read_inputs();
    let mut total = 50;
    let mut count = 0;
    let mut rotation;
    for line in inputs {
        if line.starts_with("L") {
            rotation = -(line[1..line.len()].parse::<i32>().unwrap());
        } else if line.starts_with("R") {
            rotation = line[1..line.len()].parse::<i32>().unwrap();
        } else {
            panic!("Invalid input from the file.");
        }

        total = (total + rotation) % 100; // the rotation of the dial is addition mod 100
        if total == 0 {
            count += 1;
        }
    }
    count.to_string()
}

pub fn d1p2() -> String {
    let inputs = read_inputs();
    let mut total: i32 = 50;
    let mut count: i32 = 0;
    let mut rotation: i32;
    for line in inputs {
        if line.starts_with("L") {
            rotation = -(line[1..line.len()].parse::<i32>().unwrap());
        } else if line.starts_with("R") {
            rotation = line[1..line.len()].parse::<i32>().unwrap();
        } else {
            panic!("Invalid input from the file.");
        }

        count += (rotation / 100).abs(); // first add the the full rotations
        rotation %= 100;
        if rotation < 0 {
            if total != 0 && total <= rotation.abs() {
                count += 1;
            }
        } else if total + rotation >= 100 {
            count += 1;
        }
        total = (total + rotation) % 100; // the rotation of the dial is addition mod 100
        if total < 0 {
            total += 100;
        }
    }
    count.to_string()
}
