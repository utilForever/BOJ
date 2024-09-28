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

    let n = scan.token::<usize>();
    let mut jewelry_case = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            jewelry_case[i][j] = scan.token::<i64>();
        }
    }

    let process_check = |jewelry_case: &Vec<Vec<i64>>, n: usize| -> bool {
        let mut ret = true;

        // Horizontal
        for i in 0..n {
            let mut check = true;

            for j in 0..n - 1 {
                if jewelry_case[i][j] > jewelry_case[i][j + 1] {
                    check = false;
                    break;
                }
            }

            if !check {
                ret = false;
                break;
            }
        }

        // Vertical
        for i in 0..n {
            let mut check = true;

            for j in 0..n - 1 {
                if jewelry_case[j][i] > jewelry_case[j + 1][i] {
                    check = false;
                    break;
                }
            }

            if !check {
                ret = false;
                break;
            }
        }

        ret
    };

    let mut ret = 0;

    loop {
        if process_check(&jewelry_case, n) {
            break;
        }

        ret += 1;

        // Rotate counter clockwise
        let mut new_jewelry_case = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                new_jewelry_case[n - j - 1][i] = jewelry_case[i][j];
            }
        }

        jewelry_case = new_jewelry_case;
    }

    writeln!(out, "{ret}").unwrap();
}
