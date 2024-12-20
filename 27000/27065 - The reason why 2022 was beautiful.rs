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

    let mut is_exceed_num = [false; 5001];
    let mut factors = vec![Vec::new(); 5001];

    for i in 1..=5000 {
        let mut nums = Vec::new();

        for j in 1..=(i as f64).sqrt() as usize {
            if i % j == 0 {
                nums.push(j);

                if j != 1 && j != i / j {
                    nums.push(i / j);
                }
            }
        }

        let sum = nums.iter().sum::<usize>();

        is_exceed_num[i] = sum > i;
        factors[i] = nums;
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();

        writeln!(
            out,
            "{}",
            if is_exceed_num[n] && factors[n].iter().all(|&x| !is_exceed_num[x]) {
                "Good Bye"
            } else {
                "BOJ 2022"
            }
        )
        .unwrap();
    }
}
