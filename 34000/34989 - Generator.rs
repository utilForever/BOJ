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

    let n = scan.token::<usize>();
    let mut sequence = vec![0; n];

    for i in 0..n {
        sequence[i] = scan.token::<i64>();
    }

    let s = scan.token::<String>();
    let mut ret = sequence.clone();

    if s == "yes" {
        let mut choose = -1;

        for k in 1..=n / 2 {
            let mut check = true;

            for i in 0..k {
                let left = sequence[i];
                let right = sequence[n - k + i];

                if left != 0 && right != 0 && left != right {
                    check = false;
                    break;
                }
            }

            if check {
                choose = k as i64;
                break;
            }
        }

        if choose != -1 {
            let k = choose as usize;

            for i in 0..k {
                let idx_left = i;
                let idx_right = n - k + i;

                if ret[idx_left] == 0 && ret[idx_right] == 0 {
                    ret[idx_left] = 1;
                    ret[idx_right] = 1;
                } else if ret[idx_left] == 0 {
                    ret[idx_left] = ret[idx_right];
                } else if ret[idx_right] == 0 {
                    ret[idx_right] = ret[idx_left];
                }
            }
        }

        for x in ret.iter_mut() {
            if *x == 0 {
                *x = 1;
            }
        }
    } else {
        let mut used = vec![false; 2001];

        for &val in ret.iter() {
            if val != 0 {
                used[val as usize] = true;
            }
        }

        let mut curr = 1;

        for i in 0..n {
            if ret[i] == 0 {
                while curr <= 2000 && used[curr] {
                    curr += 1;
                }

                ret[i] = curr as i64;
                used[curr] = true;
                curr += 1;
            }
        }
    }

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
