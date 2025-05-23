use io::Write;
use std::{collections::HashSet, io, str};

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let mut statements = Vec::new();

    loop {
        let line = scan.line().trim().to_string();

        if line.is_empty() {
            break;
        }

        statements.push(line);
    }

    let mut variables = HashSet::new();
    let mut check = vec![false; statements.len() - 1];
    let statement_last = statements
        .last()
        .unwrap()
        .clone()
        .chars()
        .collect::<Vec<_>>();

    for i in (0..statements.len() - 1).rev() {
        let statement = statements[i].clone().chars().collect::<Vec<_>>();

        if statement_last.iter().any(|&c| c == statement[9]) && !variables.contains(&statement[9]) {
            variables.insert(statement[9]);
            check[i] = true;
        }
    }

    let mut offset = 0;

    for i in 0..statements.len() - 1 {
        if !check[i] {
            continue;
        }

        for _ in 0..offset {
            write!(out, " ").unwrap();
        }

        writeln!(out, "{}", statements[i]).unwrap();

        offset += 1;
    }

    for _ in 0..offset {
        write!(out, " ").unwrap();
    }

    writeln!(out, "{}", statements.last().unwrap()).unwrap();
}
