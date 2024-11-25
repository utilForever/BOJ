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

    let n = scan.token::<i64>();
    let mut romans: HashMap<char, i64> = HashMap::new();
    romans.insert('I', 1);
    romans.insert('V', 5);
    romans.insert('X', 10);
    romans.insert('L', 50);
    romans.insert('C', 100);
    romans.insert('D', 500);
    romans.insert('M', 1000);

    for _ in 0..n {
        let s = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut val = 0;
        let mut ret = 0;

        for i in (0..s.len()).rev() {
            let num = *romans.get(&s[i]).unwrap();

            if num >= val {
                ret += num;
                val = num;
            } else {
                ret -= num;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
