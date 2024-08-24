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

    let p = scan.token::<i64>();

    for _ in 0..p {
        let t = scan.token::<i64>();
        let mut nums = vec![0; 12];

        for j in 0..12 {
            nums[j] = scan.token::<i64>();
        }

        let mut ret = 0;

        for j in 1..11 {
            let prev = nums[j - 1];

            for k in j..11 {
                let next = nums[k + 1];
                let mut check = true;

                for l in j..=k {
                    if nums[l] <= prev || nums[l] <= next {
                        check = false;
                        break;
                    }
                }

                if check {
                    ret += 1;
                }
            }
        }

        writeln!(out, "{t} {ret}").unwrap();
    }
}
