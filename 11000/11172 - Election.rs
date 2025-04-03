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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut players = vec![(String::new(), 0, 0, 0); n];

        for i in 0..n {
            players[i] = (
                scan.token::<String>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );
        }

        let mut allies = vec![0; n];

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let dx = players[i].1 - players[j].1;
                let dy = players[i].2 - players[j].2;

                if dx * dx + dy * dy <= (players[i].3 + players[j].3).pow(2) {
                    allies[i] += 1;
                }
            }
        }

        let allies_max = *allies.iter().max().unwrap();
        let cnt_max = allies.iter().filter(|&&x| x == allies_max).count();

        if cnt_max == 1 {
            writeln!(
                out,
                "{}",
                players[allies.iter().position(|&x| x == allies_max).unwrap()].0
            )
            .unwrap();
        } else {
            writeln!(out, "TIE").unwrap();
        }
    }
}
