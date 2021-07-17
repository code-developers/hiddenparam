use std::{collections::VecDeque, io};

pub fn diff(
    text1: &str,
    text2: &str,
) -> io::Result<Vec<String>> {
    let mut processor = Processor::new();
    {
        let mut replace = diffs::Replace::new(&mut processor);
        diffs::myers::diff(&mut replace, &text1.lines().collect::<Vec<&str>>(), &text2.lines().collect::<Vec<&str>>())?;
    }
    Ok(processor.result())
}

struct Processor {
    inserted: usize,
    removed: usize,

    context: Context,
    result: Vec<String>,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            inserted: 0,
            removed: 0,

            context: Context::new(),
            result: Vec::new(),
        }
    }

    pub fn result(self) -> Vec<String> {
        self.result
    }
}