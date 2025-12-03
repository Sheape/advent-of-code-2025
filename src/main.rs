mod day_1;
mod solution;

use day_1::Day1;
pub use solution::Solution;

fn main() {
    println!("DAY 1 [PART 1]: Actual password is {}", Day1::part1());
    println!("DAY 1 [PART 2]: Actual password is {}", Day1::part2());
}
