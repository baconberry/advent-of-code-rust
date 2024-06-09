#![allow(unused)]
use anyhow::Result;

use crate::prelude::*;

mod col_utils;
mod day_10_1;
mod day_11_1;
mod day_11_2;
mod day_12_1;
mod day_12_2;
mod day_13_1;
mod day_14;
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
pub mod prelude;
mod re_utils;
mod trebuchet;
mod trebuchet_2;
mod utils;

pub fn process_lines(lines: Vec<String>, day: DayProblem) -> Result<usize> {
    match day {
        DayProblem::Day13(d) => day_13_1::process(lines, d),
        DayProblem::Day14(d) => day_14::process(lines, d),
        _ => panic!("Not implemented {:?}", day),
    }
}
