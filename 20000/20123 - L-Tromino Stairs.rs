use io::Write;
use std::{io, str};

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

const BASE2: [&str; 2] = ["|", "b-"];
const BASE6: [&str; 6] = ["|", "b-", "p-|", "||b-", "|b-||", "b--db-"];
const BASE9: [&str; 9] = [
    "|",
    "b-",
    "p-|",
    "||b-",
    "|b-p-",
    "b-||p-",
    "p-b-|p-",
    "||p-||||",
    "-d|-d-db-",
];
const BASE11: [&str; 11] = [
    "|",
    "b-",
    "p-|",
    "||b-",
    "|b-p-",
    "b-||p-",
    "p-b-|p-",
    "||p-||||",
    "-d|-d-db-",
    "p-|p-|p-||",
    "|-d|-d|-db-",
];

const RECT2: [&str; 6] = ["p-", "||", "-d", "p-", "||", "-d"];
const RECT3: [&str; 6] = ["p-|", "|-d", "p-|", "|-d", "p-|", "|-d"];

fn build_rectangle(width: usize) -> Vec<String> {
    let mut ret = vec![String::new(); 6];
    let mut idx = width;

    if idx % 2 == 1 {
        for i in 0..6 {
            ret[i].push_str(RECT3[i]);
        }

        idx -= 3;
    }

    for _ in 0..idx / 2 {
        for i in 0..6 {
            ret[i].push_str(RECT2[i]);
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    if n % 3 == 1 || n == 3 || n == 5 {
        writeln!(out, "impossible").unwrap();
        return;
    }

    let mut ret: Vec<String> = match n % 6 {
        0 => BASE6.iter().map(|&s| s.to_string()).collect(),
        2 => BASE2.iter().map(|&s| s.to_string()).collect(),
        3 => BASE9.iter().map(|&s| s.to_string()).collect(),
        5 => BASE11.iter().map(|&s| s.to_string()).collect(),
        _ => unreachable!(),
    };

    let mut idx = ret.len();

    while idx < n {
        let rectangle = build_rectangle(idx);

        for i in 0..6 {
            let mut row = String::new();
            row.push_str(&rectangle[i]);
            row.push_str(BASE6[i]);

            ret.push(row);
        }

        idx += 6;
    }

    for i in 0..ret.len() {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}
