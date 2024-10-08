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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let g = scan.token::<usize>();
        let mut nums = vec![0; g];

        for i in 0..g {
            nums[i] = scan.token::<i32>();
        }

        if g == 1 {
            writeln!(out, "1").unwrap();
            continue;
        }

        let mut ret = 0;

        for m in 1..1_000_000 {
            let mut cnt = vec![0i16; m];

            for num in nums.iter() {
                cnt[*num as usize % m] += 1;
            }

            if cnt.iter().all(|&x| x == 0 || x == 1) {
                ret = m;
                break;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
