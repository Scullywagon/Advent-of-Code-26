use std::{
    fs::File,
    io::{Read, read_to_string},
};

pub enum Rotation {
    Left(i32),
    Right(i32),
}

pub fn rotate_dial(mut current: i32, rotation: Rotation) -> i32 {
    match rotation {
        Rotation::Left(i) => current -= i,
        Rotation::Right(i) => current += i,
    }

    current %= 100;

    if current < 0 {
        current += 100
    }

    current
}

pub fn read_and_rotate() -> i32 {
    let mut contents = String::new();
    File::open("./input/day1.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    let orders: Vec<Rotation> = contents
        .lines()
        .map(|x| {
            let (d, n) = x.split_at(1);
            let mag = n.parse::<i32>().unwrap();
            match d {
                "L" => Rotation::Left(mag),
                "R" => Rotation::Right(mag),
                _ => panic!("invalid dir"),
            }
        })
        .collect();

    let mut dial = 50;
    let mut count = 0;
    for order in orders {
        dial = rotate_dial(dial, order);
        if dial == 0 {
            count += 1;
        }
    }
    count
}

