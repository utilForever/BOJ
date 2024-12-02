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

    let m = scan.token::<usize>();
    let mut bonuses = vec![0; m];
    let mut goals = vec![0; m];

    for i in 0..m {
        bonuses[i] = scan.token::<i64>();
    }

    for i in 0..m {
        goals[i] = scan.token::<i64>();
    }

    for (&bonus, &goal) in bonuses.iter().zip(goals.iter()) {
        // Edge case for bonus = 1
        // If bonus = 1, you can get 1 item for each purchase
        // So if goal > 0, you only need to purchase 1 item to get infinite items
        // If goal = 0, you don't need to purchase any items
        let ret = if bonus == 1 {
            if goal == 0 {
                0
            } else {
                1
            }
        } else {
            let mut left = 0;
            let mut right = 1_000_000_000;

            while left <= right {
                let mid = (left + right) / 2;
                // The total number of items you can get after purchasing mid items is
                // mid + ((mid - 1) / (bonus - 1)).floor()
                let item_total = mid + ((mid - 1) / (bonus - 1));

                if item_total >= goal {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }

            left
        };

        write!(out, "{ret} ").unwrap();
    }

    writeln!(out).unwrap();
}
