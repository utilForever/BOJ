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

    let name_yeondu = scan.token::<String>();
    let name_yeondu = name_yeondu.chars().collect::<Vec<_>>();

    let n = scan.token::<usize>();
    let mut teams = vec![(String::new(), 0); n];

    for i in 0..n {
        let name = scan.token::<String>();
        let name = name.chars().collect::<Vec<_>>();

        let l = name_yeondu.iter().filter(|&&c| c == 'L').count()
            + name.iter().filter(|&&c| c == 'L').count();
        let o = name_yeondu.iter().filter(|&&c| c == 'O').count()
            + name.iter().filter(|&&c| c == 'O').count();
        let v = name_yeondu.iter().filter(|&&c| c == 'V').count()
            + name.iter().filter(|&&c| c == 'V').count();
        let e = name_yeondu.iter().filter(|&&c| c == 'E').count()
            + name.iter().filter(|&&c| c == 'E').count();
        let val = ((l + o) * (l + v) * (l + e) * (o + v) * (o + e) * (v + e)) % 100;

        teams[i] = (name.into_iter().collect::<String>(), val);
    }

    teams.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    writeln!(out, "{}", teams[0].0).unwrap();
}
