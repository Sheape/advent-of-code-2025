mod day_1;
mod day_2;
mod solution;

use day_1::Day1;
use day_2::Day2;
pub use solution::Solution;

fn main() {
    println!("DAY 1 [PART 1]: Actual password is {}", Day1::part1());
    println!("DAY 1 [PART 2]: Actual password is {}", Day1::part2());
    println!("DAY 2 [PART 1]: Total Invalid IDs {}", Day2::part1());
    println!("DAY 2 [PART 2]: Total Invalid IDs {}", Day2::part2());
}
