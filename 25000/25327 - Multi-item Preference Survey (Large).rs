use io::Write;
use std::{collections::HashMap, io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut students = HashMap::new();
    
    for _ in 0..n {
        let (subject, fruit, color) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );

        students.entry((subject.clone(), fruit.clone(), color.clone())).and_modify(|e| *e += 1).or_insert(1);
        students.entry(("-".to_string(), fruit.clone(), color.clone())).and_modify(|e| *e += 1).or_insert(1);
        students.entry((subject.clone(), "-".to_string(), color.clone())).and_modify(|e| *e += 1).or_insert(1);
        students.entry((subject.clone(), fruit.clone(), "-".to_string())).and_modify(|e| *e += 1).or_insert(1);
        students.entry(("-".to_string(), "-".to_string(), color.clone())).and_modify(|e| *e += 1).or_insert(1);
        students.entry(("-".to_string(), fruit.clone(), "-".to_string())).and_modify(|e| *e += 1).or_insert(1);
        students.entry((subject.clone(), "-".to_string(), "-".to_string())).and_modify(|e| *e += 1).or_insert(1);
        students.entry(("-".to_string(), "-".to_string(), "-".to_string())).and_modify(|e| *e += 1).or_insert(1);
    }

    for _ in 0..m {
        let (subject, fruit, color) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );

        writeln!(out, "{}", students.get(&(subject.clone(), fruit.clone(), color.clone())).unwrap_or(&0)).unwrap()
    }
}
