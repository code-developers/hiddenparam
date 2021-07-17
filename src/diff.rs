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

            counter: 0,
            equaled: 0,
            removed: 0,
            inserted: 0,
        }
    }

    pub fn to_vec(&self, removed: usize, inserted: usize) -> Vec<String> {
        let mut start = if let Some(start) = self.start {
            start
        } else {
            return Vec::new();
        };
        if start == 0 {
            start = 1;
        }
        let mut data = Vec::with_capacity(self.data.len() + 1);
        if self.changed {
            data.push(format!(
                "-{},{} +{},{}",
                start,
                self.equaled + self.removed,
                start + inserted - removed,
                self.equaled + self.inserted,
            ));
            for s in self.data.iter() {
                data.push(s.to_owned());
            }
        }
        data
    }
}