#[derive(Debug)]
pub enum List {
    Empty,
    Num(u8),
    Array(Vec<Box<List>>),
}
