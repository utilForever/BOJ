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
        let n = scan.token::<usize>();
        let mut rps_robots = vec![String::new(); n];

        for i in 0..n {
            rps_robots[i] = scan.token::<String>();
        }

        let rps_robots = rps_robots
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let k = rps_robots[0].len();
        let mut ret = vec![true; n];

        for i in 0..k {
            let mut rps = Vec::new();

            for j in 0..n {
                if !ret[j] {
                    continue;
                }

                rps.push((j, rps_robots[j][i]));
            }

            let is_all_same = rps.iter().all(|(_, c)| *c == rps[0].1);
            let is_exist_rps = rps.iter().any(|(_, c)| *c == 'R')
                && rps.iter().any(|(_, c)| *c == 'P')
                && rps.iter().any(|(_, c)| *c == 'S');

            if is_all_same || is_exist_rps {
                continue;
            }

            // Case R and P
            if rps.iter().any(|(_, c)| *c == 'R') && rps.iter().any(|(_, c)| *c == 'P') {
                for (j, c) in rps.iter() {
                    if *c == 'R' {
                        ret[*j] = false;
                    }
                }
            }

            // Case R and S
            if rps.iter().any(|(_, c)| *c == 'R') && rps.iter().any(|(_, c)| *c == 'S') {
                for (j, c) in rps.iter() {
                    if *c == 'S' {
                        ret[*j] = false;
                    }
                }
            }

            // Case P and S
            if rps.iter().any(|(_, c)| *c == 'P') && rps.iter().any(|(_, c)| *c == 'S') {
                for (j, c) in rps.iter() {
                    if *c == 'P' {
                        ret[*j] = false;
                    }
                }
            }

            if ret.iter().filter(|&&is_survive| is_survive).count() == 1 {
                break;
            }
        }

        writeln!(
            out,
            "{}",
            if ret.iter().filter(|&&is_survive| is_survive).count() > 1 {
                0
            } else {
                ret.iter().position(|&is_survive| is_survive).unwrap() + 1
            }
        )
        .unwrap();
    }
}
