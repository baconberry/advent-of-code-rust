use utils::read_lines;

mod day_2_1;
mod day_2_2;
mod day_3_1;
mod day_3_2;
mod day_4_1;
mod day_4_2;
mod trebuchet;
mod trebuchet_2;
mod utils;

#[allow(unused)]
fn main() {
    let lines = read_lines("input.txt".to_string());
    day_4_2::process_lines(lines);
}
