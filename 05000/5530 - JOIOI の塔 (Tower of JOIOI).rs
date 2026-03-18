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

fn check(s: &Vec<char>, cnt_i: usize, x: usize) -> bool {
    let mut remain = cnt_i;
    let mut cnt_first = 0;
    let mut cnt_last = 0;
    let mut completed = 0;

    for &c in s {
        match c {
            'J' => {
                cnt_first += 1;
            }
            'O' => {
                if cnt_first > 0 {
                    cnt_first -= 1;
                    cnt_last += 1;
                }
            }
            'I' => {
                let is_last = remain <= x;

                remain -= 1;

                if is_last {
                    if cnt_last > 0 {
                        cnt_last -= 1;
                        completed += 1;

                        if completed == x {
                            return true;
                        }
                    }
                } else {
                    cnt_first += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let s = scan.token::<String>().chars().collect::<Vec<_>>();

    let cnt_i = s.iter().filter(|&&c| c == 'I').count();
    let cnt_o = s.iter().filter(|&&c| c == 'O').count();

    let mut left = 0;
    let mut right = cnt_i.min(cnt_o).min(n / 3) + 1;

    while left + 1 < right {
        let mid = (left + right) / 2;

        if check(&s, cnt_i, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    writeln!(out, "{left}").unwrap();
}
