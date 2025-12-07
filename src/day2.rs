use std::{fs::File, io::Read};

pub fn is_repeating_num(num: i64) -> bool {
    let str_num = num.to_string();
    if !str_num.len().is_multiple_of(2) {
        return false;
    }

    let (h1, h2) = str_num.split_at(str_num.len() / 2);

    h1 == h2
}

pub fn read_and_parse_ids() -> i64 {
    let mut contents = String::new();
    File::open("./input/day2.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    contents
        .split(",")
        .map(|id| {
            let (b, t) = id.split_once("-").unwrap();
            let bottom = b.trim().parse().unwrap();
            let top = t.trim().parse().unwrap();
            (bottom..top).filter(|&x| is_repeating_num(x)).sum::<i64>()
        })
        .sum()
}
