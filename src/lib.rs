#![allow(unused)]
use anyhow::Result;

mod col_utils;
mod day_10_1;
mod day_11_1;
mod day_11_2;
mod day_12_1;
mod day_12_2;
mod day_13_1;
mod day_2_1;
mod day_2_2;
mod day_3_1;
mod day_3_2;
mod day_4_1;
mod day_4_2;
mod day_5_1;
mod day_5_2;
mod day_6_1;
mod day_6_2;
mod day_7_1;
mod day_7_2;
mod day_8_1;
mod day_8_2;
mod day_9_1;
mod day_9_2;
mod prelude;
mod re_utils;
mod trebuchet;
mod trebuchet_2;
mod utils;

pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    day_13_1::process_lines(lines)
}
