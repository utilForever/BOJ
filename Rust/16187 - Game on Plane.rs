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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut grundy = vec![0; 5001];

    for i in 0..=5000 {
        let mut s = HashSet::new();

        for j in 0..=(i as i32 - 2) {
            s.insert(grundy[j as usize] ^ grundy[i - 2 - j as usize]);
        }

        let mut j = 0;

        loop {
            if s.get(&j) == None {
                grundy[i] = j;
                break;
            }

            j += 1;
        }
    }

    let t = scan.token::<usize>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        writeln!(out, "{}", if grundy[n] > 0 { "First" } else { "Second" }).unwrap();
    }
}
