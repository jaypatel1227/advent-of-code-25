fn read_inputs() -> Vec<String> {
    std::fs::read_to_string("inputs/d1p1.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn d1p1() -> String {
    "".into()
}

pub fn d1p2() -> String {
    "".into()
}
