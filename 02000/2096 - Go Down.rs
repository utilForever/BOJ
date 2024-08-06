use io::Write;
use std::{cmp, io, str};

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

    let n = scan.token::<usize>();

    let mut arr = [0, 0, 0];
    arr[0] = scan.token::<usize>();
    arr[1] = scan.token::<usize>();
    arr[2] = scan.token::<usize>();

    let mut min_values = [arr[0], arr[1], arr[2]];
    let mut max_values = [arr[0], arr[1], arr[2]];

    for _ in 1..n {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let mut temp = [0, 0, 0];

        temp[0] = cmp::min(min_values[0] + a, min_values[1] + a);
        temp[1] = cmp::min(
            min_values[0] + b,
            cmp::min(min_values[1] + b, min_values[2] + b),
        );
        temp[2] = cmp::min(min_values[1] + c, min_values[2] + c);

        min_values = temp;

        temp[0] = cmp::max(max_values[0] + a, max_values[1] + a);
        temp[1] = cmp::max(
            max_values[0] + b,
            cmp::max(max_values[1] + b, max_values[2] + b),
        );
        temp[2] = cmp::max(max_values[1] + c, max_values[2] + c);

        max_values = temp;
    }

    writeln!(
        out,
        "{} {}",
        cmp::max(max_values[0], cmp::max(max_values[1], max_values[2])),
        cmp::min(min_values[0], cmp::min(min_values[1], min_values[2]))
    )
    .unwrap();
}
