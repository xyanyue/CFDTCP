use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use aho_corasick::AhoCorasick;
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
#[derive(Debug)]
pub struct Processing {
    stop_words: Vec<String>,
    word_segmentation: bool,
    ac_auto: Option<AhoCorasick>,
}

impl Processing {
    pub fn new() -> Self {
        Processing {
            stop_words: Vec::new(),
            word_segmentation: false,
            ac_auto: None,
        }
    }

    pub fn new_file(path: &Path) -> Self {
        let mut p = Processing::new();
        if let Ok(lines) = read_lines(path) {
            // 使用迭代器，返回一个（可选）字符串
            for line in lines {
                if let Ok(s) = line {
                    p.add_stop_word(s);
                }
            }
        }
        p
    }

    pub fn add_stop_word(&mut self, s: String) -> &mut Self {
        self.stop_words.push(s);
        self
    }

    pub fn set_ac(&mut self) {
        self.ac_auto = match AhoCorasick::new(&self.stop_words) {
            Ok(ac) => Some(ac),
            Err(_) => None,
        };
    }

    pub fn parse(&self, mut input: String) -> String {
        match &self.ac_auto {
            Some(ac) => {
                let mut matches = vec![];
                for mat in ac.find_iter(&input) {
                    matches.push((mat.start(), mat.end()));
                }
                for (s, e) in matches {
                    for i in s..e {
                        input.set_by_index(i, '_' as u8);
                    }
                }
                // input
            }
            None => {}
        };
        input
    }

    // pub fn
}

trait SetByIndex {
    fn set_by_index(&mut self, idx: usize, c: u8);
}
impl SetByIndex for String {
    fn set_by_index(&mut self, idx: usize, c: u8) {
        if idx >= self.len() {
            panic!("Index out of bounds: {}, expected: [0,{})", idx, self.len());
        }
        unsafe {
            let _buf: &mut [u8] = self.as_bytes_mut();
            _buf[idx] = c;
        }
    }
}
