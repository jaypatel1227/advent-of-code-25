fn read_input() -> Vec<String> {
    std::fs::read_to_string("inputs/d3.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn d3p1() -> String {
    let input = read_input();
    let mut total: u128 = 0;
    for line in input {
        total += p1_find_largest(line);
    }
    total.to_string()
}

pub fn d3p2() -> String {
    let input = read_input();
    let mut total: u128 = 0;
    for line in input {
        total += p2_find_largest(line);
    }
    total.to_string()
}

fn p1_find_largest(val: String) -> u128 {
    let digits: Vec<u32> = val.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let first_digit: u32 = digits[0..digits.len() - 1].iter().max().unwrap().clone();
    let idx_to_start = digits.iter().position(|dig| dig == &first_digit).unwrap();
    let second_digit = digits[idx_to_start + 1..digits.len()]
        .iter()
        .max()
        .unwrap()
        .clone();
    (first_digit * 10 + second_digit) as u128
}

fn p2_find_largest(val: String) -> u128 {
    let digits: Vec<u32> = val.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut total: u128 = 0;
    let mut start_idx: usize = 0;

    for digits_remaining in (1..=12).rev() {
        let end_idx = digits.len() - digits_remaining + 1;
        let search_slice = &digits[start_idx..end_idx];
        let (max_digit, relative_idx) = get_largest_next_digit(search_slice);
        start_idx = start_idx + relative_idx + 1;
        total += max_digit as u128 * 10u128.pow((digits_remaining - 1) as u32);
    }
    total
}

fn get_largest_next_digit(digit_slice: &[u32]) -> (u32, usize) {
    let max_digit = *digit_slice.iter().max().unwrap();
    let idx = digit_slice.iter().position(|&d| d == max_digit).unwrap();
    (max_digit, idx)
}
