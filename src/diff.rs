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

struct Context {
    pub start: Option<usize>,
    pub data: VecDeque<String>,
    pub changed: bool,
    
    pub counter: usize,
    pub equaled: usize,
    pub removed: usize,
    pub inserted: usize
}

impl Context {
    pub fn new() -> Self {
        Self {
            start: None,
            data: VecDeque::new(),
            changed: false,

            counter: 0
        }
    }
}