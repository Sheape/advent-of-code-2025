use std::{fs, path::Path};

use crate::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part1() -> u64 {
        let input = Path::new("input/day1.txt");
        let contents = fs::read_to_string(input).expect("Something wrong with opening this file");
        let mut cur_iter = 50_i32;
        let mut total = 0;
        contents.lines().for_each(|line| {
            let (cmd, num_str) = line.split_at(1);
            let num = num_str.parse::<i32>().unwrap();
            match cmd {
                "L" => cur_iter -= num,
                "R" => cur_iter += num,
                _ => panic!("Unexpected number"),
            }
            cur_iter = cur_iter.rem_euclid(100);
            if cur_iter == 0 {
                total += 1;
            }
        });

        total
    }

    fn part2() -> u64 {
        let input = Path::new("input/day1.txt");
        let contents = fs::read_to_string(input).expect("Something wrong with opening this file");
        let mut cur_iter = 50_i32;
        let mut total = 0_u64;
        contents.lines().for_each(|line| {
            let (cmd, num_str) = line.split_at(1);
            let num = num_str.parse::<i32>().unwrap();
            let before = cur_iter;
            match cmd {
                "L" => cur_iter -= num,
                "R" => cur_iter += num,
                _ => panic!("Unexpected number"),
            }

            if cur_iter < 0 && before != 0 {
                total += 1;
            }

            if cur_iter != 0 {
                let excess = (cur_iter.unsigned_abs() - 1) / 100;
                total += excess as u64;
            }

            cur_iter = cur_iter.rem_euclid(100);
            if cur_iter == 0 {
                total += 1;
            }
        });

        total
    }
}
