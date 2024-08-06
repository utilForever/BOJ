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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let k = scan.token::<i64>();
        let answers_me = scan.token::<String>();
        let answers_friend = scan.token::<String>();
        let answers_me = answers_me.chars().collect::<Vec<_>>();
        let answers_friend = answers_friend.chars().collect::<Vec<_>>();
        let mut cnt_same = 0;

        answers_me
            .iter()
            .zip(answers_friend.iter())
            .for_each(|(a, b)| {
                if a == b {
                    cnt_same += 1;
                }
            });

        writeln!(out, "{}", answers_me.len() as i64 - (cnt_same - k).abs()).unwrap();
    }
}
