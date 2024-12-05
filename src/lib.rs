use std::io;
use std::io::{Lines, StdinLock};

pub struct MyIn {
    lines: Lines<StdinLock<'static>>,
    cur_line: Option<String>,
}

impl MyIn {
    pub fn new() -> MyIn {
        MyIn {
            lines: (io::stdin().lines()),
            cur_line: Option::None,
        }
    }
    pub fn read_line(&mut self) -> String {
        let next = self.lines.next();
        next.unwrap_or(Ok(String::new())).unwrap()
    }
    pub fn read_word(&mut self) -> String {
        let line = self
            .cur_line
            .take()
            .unwrap_or_else(|| self.read_line())
            .trim()
            .to_string();
        match line.split_once(' ') {
            None => line,
            Some((word, rest)) => {
                self.cur_line = Option::Some(rest.to_string());
                println!("read {} {:?}\n", word, &self.cur_line);
                word.to_string()
            }
        }
    }
    pub fn read_num(&mut self) -> f64 {
        self.read_word().parse().unwrap_or(-1.0)
    }
}
