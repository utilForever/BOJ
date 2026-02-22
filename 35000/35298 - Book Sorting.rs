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
    let mut heights = vec![0; n];

    for i in 0..n {
        heights[i] = scan.token::<usize>();
    }

    let mut parity = 0;

    for i in 0..n {
        for j in i + 1..n {
            if heights[i] > heights[j] {
                parity ^= 1;
            }
        }
    }

    if parity == 1 {
        writeln!(out, "NO").unwrap();
    } else {
        writeln!(out, "YES").unwrap();

        let operate = |heights: &mut Vec<usize>, ops: &mut Vec<usize>, pos: usize| {
            heights[..pos + 1].rotate_right(2);
            ops.push(pos);
        };
        let mut operations = Vec::new();

        for i in (3..=n).rev() {
            let pos = heights.iter().position(|&x| x == i).unwrap() + 1;

            if pos == i {
                continue;
            }

            if i % 2 == 1 {
                let inv2 = (i + 1) / 2;
                let num_ops = ((2 * i - pos) * inv2) % i;

                for _ in 0..num_ops {
                    operate(&mut heights, &mut operations, i - 1);
                }
            } else {
                let inv2 = i / 2;
                let num_ops = ((2 * i - 3 - pos) * inv2) % (i - 1);

                for _ in 0..num_ops {
                    operate(&mut heights, &mut operations, i - 2);
                }

                operate(&mut heights, &mut operations, i - 1);
            }
        }

        writeln!(out, "{}", operations.len()).unwrap();

        for op in operations {
            write!(out, "{op} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
