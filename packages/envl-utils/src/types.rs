#[derive(Debug, Clone, PartialEq, Default)]
pub struct FilePosition {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Position {
    pub file_path: String,
    pub start: FilePosition,
    pub end: FilePosition,
}
