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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut friends = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            friends[i][j] = scan.token::<i64>();
        }
    }

    let mut diff = vec![0; n];

    for i in 0..n {
        let mut cnt = 0;

        for j in 0..m {
            if friends[i][j] != friends[0][j] {
                cnt += 1;
            }
        }

        diff[i] = cnt;
    }

    'outer: for j in 0..m {
        let mut valid = true;
        let mut blocked = vec![false; 2001];
        let mut required = None;

        for i in 0..n {
            if friends[i][j] == friends[0][j] {
                if diff[i] != 0 {
                    valid = false;
                    break;
                }
            } else {
                match diff[i] {
                    1 => {
                        blocked[friends[i][j] as usize] = true;
                    }
                    2 => {
                        if let Some(req) = required {
                            if req != friends[i][j] {
                                valid = false;
                                break;
                            }
                        } else {
                            required = Some(friends[i][j]);
                        }
                    }
                    _ => {
                        valid = false;
                        break;
                    }
                }
            }
        }

        if !valid {
            continue;
        }

        let candidate_v = if let Some(x) = required {
            if blocked[x as usize] {
                continue 'outer;
            }

            x
        } else {
            let mut chosen = None;

            for v in 0..=2000 {
                if v == friends[0][j] {
                    continue;
                }

                if blocked[v as usize] {
                    continue;
                }

                chosen = Some(v);
                break;
            }

            if let Some(v) = chosen {
                v
            } else {
                continue;
            }
        };

        let mut candidate_a = friends[0].clone();
        candidate_a[j] = candidate_v;

        let mut check = true;

        for i in 0..n {
            let mut cnt = 0;

            for k in 0..m {
                if candidate_a[k] != friends[i][k] {
                    cnt += 1;
                }
            }

            if cnt != 1 {
                check = false;
                break;
            }
        }

        if check {
            for val in candidate_a.iter() {
                write!(out, "{val} ").unwrap();
            }

            writeln!(out).unwrap();
            return;
        }
    }
}
