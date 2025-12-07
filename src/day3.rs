use std::{fs::File, io::Read};

pub fn maximum_joltage(batteries: &str) -> i32 {
    let nums: Vec<i32> = batteries
        .chars()
        .map(|x| -> i32 { x.to_string().parse::<i32>().unwrap() })
        .collect();

    let mut first = nums[0];
    let mut second = nums[1];

    (1..nums.len()).for_each(|i| {
        let val = nums[i];
        if val > second {
            second = val;
        }
        if i != (nums.len() - 1) && val > first {
            first = val;
            second = -1;
        }
    });

    (first.to_string() + &second.to_string())
        .parse::<i32>()
        .unwrap()
}

pub fn read_batteries_and_sum() -> i32 {
    let mut content = String::new();
    File::open("./input/day3.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let lines = content.lines();

    lines.map(maximum_joltage).sum()
}
