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

    let mut keyboard = HashMap::new();

    keyboard.insert('a', (2, 1));
    keyboard.insert('b', (2, 2));
    keyboard.insert('c', (2, 3));
    keyboard.insert('d', (3, 1));
    keyboard.insert('e', (3, 2));
    keyboard.insert('f', (3, 3));
    keyboard.insert('g', (4, 1));
    keyboard.insert('h', (4, 2));
    keyboard.insert('i', (4, 3));
    keyboard.insert('j', (5, 1));
    keyboard.insert('k', (5, 2));
    keyboard.insert('l', (5, 3));
    keyboard.insert('m', (6, 1));
    keyboard.insert('n', (6, 2));
    keyboard.insert('o', (6, 3));
    keyboard.insert('p', (7, 1));
    keyboard.insert('q', (7, 2));
    keyboard.insert('r', (7, 3));
    keyboard.insert('s', (7, 4));
    keyboard.insert('t', (8, 1));
    keyboard.insert('u', (8, 2));
    keyboard.insert('v', (8, 3));
    keyboard.insert('w', (9, 1));
    keyboard.insert('x', (9, 2));
    keyboard.insert('y', (9, 3));
    keyboard.insert('z', (9, 4));

    let mut mapping = vec![0; 10];

    for i in 1..=9 {
        mapping[i] = scan.token::<usize>();
    }

    let s = scan.token::<String>();
    let mut prev = 0;

    for c in s.chars() {
        let (num, cnt) = *keyboard.get(&c).unwrap();

        if num == prev {
            write!(out, "#").unwrap();
        }

        for _ in 0..cnt {
            write!(out, "{}", mapping.iter().position(|&x| x == num).unwrap()).unwrap();
        }

        prev = num;
    }

    writeln!(out).unwrap();
}
