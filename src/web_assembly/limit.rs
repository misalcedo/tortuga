#[derive(Copy, Clone)]
pub enum Limit {
    Min(usize),
    MinMax { min: usize, max: usize },
}
