#[derive(Debug)]
pub enum DayPart {
    One,
    Two,
}

impl DayPart {
    pub fn is_one(&self) -> bool {
        match(self) {
            Self::One => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub enum DayProblem {
    Day13(DayPart),
    Day14(DayPart),
}
