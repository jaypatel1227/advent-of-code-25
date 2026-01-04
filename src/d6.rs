fn read_input_part_1() -> (Vec<Vec<u128>>, Vec<String>) {
    let binding = std::fs::read_to_string("inputs/d6.txt").unwrap();
    let mut input: Vec<&str> = binding.lines().collect();
    let operators = input
        .pop()
        .unwrap()
        .split_whitespace()
        .map(String::from)
        .collect();
    (
        input
            .into_iter()
            .map(|row| {
                row.split_whitespace()
                    .map(|s| s.parse::<u128>().unwrap())
                    .collect()
            })
            .collect(),
        operators,
    )
}

fn read_input_part_2() -> Vec<(char, Vec<u128>)> {
    let binding = std::fs::read_to_string("inputs/d6.txt").unwrap();
    let mut inputs: Vec<&str> = binding.lines().collect();
    let operator_line: Vec<char> = inputs.pop().unwrap().chars().collect();
    let lines: Vec<Vec<char>> = inputs.iter().map(|s| s.chars().collect()).collect();
    let mut out: Vec<(char, Vec<u128>)> = vec![];
    let mut current_op: char = ' ';
    let mut nums: Vec<u128> = vec![];

    for i in (0..operator_line.len()).rev() {
        let is_separator = operator_line[i].is_whitespace()
            && lines.iter().all(|row| {
                row.get(i).map_or(true, |c| c.is_whitespace())
            });

        if is_separator {
            if current_op != ' ' && !nums.is_empty() {
                out.push((current_op, nums.clone()));
                nums.clear();
                current_op = ' ';
            }
            continue;
        }

        if !operator_line[i].is_whitespace() {
            current_op = operator_line[i];
        }

        let num: String = lines
            .iter()
            .filter_map(|row| row.get(i))
            .filter(|c| !c.is_whitespace())
            .collect();

        if !num.is_empty() {
            nums.push(num.parse::<u128>().unwrap());
        }
    }
    if current_op != ' ' && !nums.is_empty() {
        out.push((current_op, nums));
    }
    out
}

pub fn d6p1() -> String {
    let (nums, ops) = read_input_part_1();
    let mut total: u128 = 0;
    for i in 0..ops.len() {
        if ops[i] == "*" {
            total += nums.iter().fold(1u128, |acc, list| acc * list[i]);
        } else {
            total += nums.iter().fold(0u128, |acc, list| acc + list[i]);
        }
    }
    total.to_string()
}

pub fn d6p2() -> String {
    let problems = read_input_part_2();
    let mut total: u128 = 0;

    for (op, numbers) in problems {
        let result = if op == '*' {
            numbers.iter().product::<u128>()
        } else {
            numbers.iter().sum::<u128>()
        };
        total += result;
    }

    total.to_string()
}
