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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();

    for _ in 0..n {
        let (location, species) = (scan.token::<String>(), scan.token::<String>());
        let mut cnt = [0; 4];

        for c in species.chars() {
            match c {
                'B' => cnt[0] += 2,
                'C' => cnt[1] += 1,
                'M' => cnt[2] += 4,
                'W' => cnt[3] += 3,
                _ => unreachable!(),
            }
        }

        let max_val = *cnt.iter().max().unwrap();
        let mut max_cnt = 0;

        for i in 0..4 {
            if cnt[i] == max_val {
                max_cnt += 1;
            }
        }

        if max_cnt == 1 {
            writeln!(
                out,
                "{location}: The {} is the dominant species",
                match cnt.iter().position(|&x| x == max_val).unwrap() {
                    0 => "Bobcat",
                    1 => "Coyote",
                    2 => "Mountain Lion",
                    3 => "Wolf",
                    _ => unreachable!(),
                }
            )
            .unwrap();
        } else {
            writeln!(out, "{location}: There is no dominant species").unwrap();
        }
    }
}
