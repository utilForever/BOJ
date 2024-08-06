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

    let n = scan.token::<i64>();
    let mut polls = Vec::new();
    let mut ret = 0;

    for i in 1..=n {
        polls.push((scan.token::<i64>(), i));
    }

    if n == 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    loop {
        polls.sort_by(|a, b| b.0.cmp(&a.0));

        if polls[0].1 == 1 && polls[0].0 > polls[1].0 {
            writeln!(out, "{ret}").unwrap();
            break;
        }

        ret += 1;

        let pos1 = polls.iter().position(|x| x.1 == 1).unwrap();
        let pos2 = polls
            .iter()
            .position(|x| x.0 == polls[0].0 && x.1 != 1)
            .unwrap();

        polls[pos1].0 += 1;
        polls[pos2].0 -= 1;
    }
}
