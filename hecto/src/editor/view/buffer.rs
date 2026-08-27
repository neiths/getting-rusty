#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    fn Default() -> Self {
        Self {
            lines: vec!["Hello, World!".to_string()]
        }
    }
}

