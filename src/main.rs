use utils::read_lines;

mod day_2_1;
mod day_2_2;
mod day_3_1;
mod day_3_2;
mod day_4_1;
mod day_4_2;
mod day_5_1;
mod day_5_2;
mod prelude;
mod re_utils;
mod trebuchet;
mod trebuchet_2;
mod utils;

#[allow(unused)]
fn main() {
    let lines = read_lines("input.txt".to_string());
    let result = day_5_2::process_lines(lines);
    println!("Result [{:?}]", result);
}
