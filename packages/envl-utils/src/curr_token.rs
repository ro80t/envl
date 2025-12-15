use crate::types::FilePosition;

pub trait Push<T> {
    fn push(&mut self, c: T, pos: FilePosition);
}

#[derive(Clone, Debug, Default)]
pub struct CurrentToken {
    pub token: String,
    pub start: FilePosition,
}

impl CurrentToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_pos(&mut self, pos: FilePosition) {
        self.start = pos;
    }

    pub fn clear(&mut self) {
        self.token.clear();
        self.start = FilePosition::default();
    }

    pub fn is_empty(&mut self) -> bool {
        self.token.is_empty()
    }
}

impl Push<char> for CurrentToken {
    fn push(&mut self, c: char, pos: FilePosition) {
        if self.token.is_empty() {
            self.start = pos;
        }
        self.token.push(c);
    }
}

impl Push<String> for CurrentToken {
    fn push(&mut self, c: String, pos: FilePosition) {
        if self.token.is_empty() {
            self.start = pos;
        }
        self.token.push_str(&c);
    }
}
