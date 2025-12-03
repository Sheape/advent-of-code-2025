use std::{cmp, collections::HashMap, fs, path::Path};

use crate::Solution;

pub struct Day2;

fn get_num_len(num: f64) -> u32 {
    num.log10().floor() as u32 + 1
}

fn repeat_num(num: u64, mut n: u32) -> u64 {
    let len = get_num_len(num as f64);
    let mut result = num;
    while n > 0 {
        result *= 10_u64.pow(len);
        result += num;
        n -= 1;
    }

    result
}

fn capture(num: u64, len: u32, nth: u32) -> u64 {
    num / (10_u64.pow(len - nth))
}

impl Solution for Day2 {
    fn part1() -> u64 {
        let input = Path::new("input/day2.txt");
        let contents = fs::read_to_string(input).expect("Something wrong with opening this file");
        let mut invalid_ids = 0_u64;
        contents.split(',').for_each(|range| {
            let Some((first_str, second_str)) = range.split_once('-') else {
                panic!("unexpected range");
            };

            let (first, second) = (
                first_str.trim().parse::<u64>().unwrap(),
                second_str.trim().parse::<u64>().unwrap(),
            );
            let (first_len, second_len) = (get_num_len(first as f64), get_num_len(second as f64));

            let max_len = cmp::max(first_len, second_len);

            let mask = 10_u64.pow(max_len / 2);
            let mut single = first / mask;
            loop {
                let dup = single * mask + single;
                if dup < first {
                    single += 1;
                    continue;
                }
                if (first..=second).contains(&dup) {
                    let len = get_num_len(dup as f64) / 2;
                    let (f, s) = (dup / 10_u64.pow(len), dup % 10_u64.pow(len));
                    if f == s {
                        invalid_ids += dup;
                    }
                } else if dup > second {
                    break;
                }
                single += 1;
            }
        });

        invalid_ids
    }

    fn part2() -> u64 {
        todo!()
    }
}
