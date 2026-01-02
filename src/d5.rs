fn read_input() -> (Vec<(u128, u128)>, Vec<u128>) {
    let input = std::fs::read_to_string("inputs/d5.txt").unwrap();

    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut parsing_ids = false;

    for line in input.lines() {
        if line.is_empty() {
            parsing_ids = true;
            continue;
        }

        if !parsing_ids {
            let mut it = line.split('-');
            let lo: u128 = it.next().unwrap().parse().unwrap();
            let hi: u128 = it.next().unwrap().parse().unwrap();
            ranges.push((lo, hi));
        } else {
            ids.push(line.parse::<u128>().unwrap());
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    ranges = merge_ranges(ranges);

    (ranges, ids)
}

pub fn d5p1() -> String {
    let (ranges, ids) = read_input();
    let mut total: u128 = 0;
    for id in ids {
        if ranges.iter().any(|(lo, hi)| lo <= &id && hi >= &id) {
            total += 1;
        }
    }
    total.to_string()
}

pub fn d5p2() -> String {
    let (ranges, _) = read_input();
    let mut total: u128 = 0;
    for (a, b) in ranges {
        total += b - a + 1;
    }
    total.to_string()
}

pub fn merge_ranges(ranges: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    let mut merged = Vec::new();

    for range in ranges {
        if merged.is_empty() {
            merged.push(range);
        } else {
            let last = merged.last_mut().unwrap();
            // If current range overlaps with the last merged range, merge them
            if range.0 <= last.1 + 1 {
                // Extend the end of the last range if needed
                if range.1 > last.1 {
                    last.1 = range.1;
                }
            } else {
                // No overlap, add current range as new
                merged.push(range);
            }
        }
    }

    merged
}
