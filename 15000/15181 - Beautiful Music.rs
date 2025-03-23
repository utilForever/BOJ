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

    let mapping = |x: char| -> i64 {
        match x {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            _ => unreachable!(),
        }
    };

    loop {
        let s = scan.token::<String>();

        if s == "#" {
            break;
        }

        let s = s.chars().collect::<Vec<_>>();
        let mut prev = s[0];
        let mut ret = true;

        for i in 1..s.len() {
            let diff = (mapping(s[i]) - mapping(prev) + 7) % 7;

            if diff != 2 && diff != 4 && diff != 6 {
                ret = false;
                break;
            }

            prev = s[i];
        }

        writeln!(
            out,
            "{}",
            if ret {
                "That music is beautiful."
            } else {
                "Ouch! That hurts my ears."
            }
        )
        .unwrap();
    }
}
