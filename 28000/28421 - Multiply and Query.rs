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

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n];
    let mut cnt = vec![0; 10001];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
        cnt[nums[i] as usize] += 1;
    }

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let x = scan.token::<i64>();

            if n == 1 {
                writeln!(out, "0").unwrap();
                continue;
            }

            if x == 0 {
                writeln!(out, "{}", if cnt[0] == 0 { 0 } else { 1 }).unwrap();
            } else {
                let mut idx = 1;
                let mut ret = false;

                while idx * idx <= x {
                    if x % idx != 0 || x / idx > 10000 {
                        idx += 1;
                        continue;
                    }

                    if idx * idx == x {
                        if cnt[idx as usize] >= 2 {
                            ret = true;
                            break;
                        }
                    } else {
                        if cnt[idx as usize] >= 1 && cnt[(x / idx) as usize] >= 1 {
                            ret = true;
                            break;
                        }
                    }

                    idx += 1;
                }

                writeln!(out, "{}", if ret { 1 } else { 0 }).unwrap();
            }
        } else {
            let i = scan.token::<usize>() - 1;

            cnt[nums[i] as usize] -= 1;
            nums[i] = 0;
            cnt[0] += 1;
        }
    }
}
