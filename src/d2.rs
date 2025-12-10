fn read_input() -> Vec<(u128, u128)> {
    std::fs::read_to_string("inputs/d2.txt")
        .unwrap()
        .replace('\n', "")
        .split(',')
        .map(|s| {
            let parts: Vec<&str> = s.split('-').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

pub fn d2p1() -> String {
    let input = read_input();
    let mut total: u128 = 0;
    for (start, end) in input {
        for i in start..=end {
            let num_digits = i.ilog10() + 1;
            if num_digits % 2 == 1 {
                continue;
            }
            if i / 10u128.pow(num_digits / 2) == i % 10u128.pow(num_digits / 2) {
                total += i;
            }
        }
    }
    total.to_string()
}

pub fn d2p2() -> String {
    let input = read_input();
    let mut total: u128 = 0;
    for (start, end) in input {
        for i in start..=end {
            let num_digits = (i.ilog10() + 1) as u128;
            let divisors = get_divisors(num_digits);
            for divisor in divisors {
                if divisor == num_digits {
                    continue;
                }
                let chunks = split_digits(i, divisor as u32);
                if chunks.iter().all(|n| n == &chunks[0]) {
                    total += i;
                    break;
                }
            }
        }
    }
    total.to_string()
}

fn split_digits(num: u128, chunk_size: u32) -> Vec<u128> {
    let total_digits = num.ilog10() + 1;
    let num_chunks = total_digits / chunk_size;
    let divisor = 10u128.pow(chunk_size);

    (0..num_chunks)
        .map(|i| (num / 10u128.pow(chunk_size * (num_chunks - 1 - i))) % divisor)
        .collect()
}

fn get_divisors(num: u128) -> Vec<u128> {
    let mut divisors = Vec::new();
    let mut i = 1;
    while i * i <= num {
        if num.is_multiple_of(i) {
            divisors.push(i);
            if i * i != num {
                divisors.push(num / i);
            }
        }
        i += 1;
    }
    divisors
}
