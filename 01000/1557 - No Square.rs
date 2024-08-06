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

fn get_no_square_num_count(arr: &Vec<i64>, k: i64) -> i64 {
    let mut count = 0;
    let mut i = 1;

    loop {
        if i * i > k {
            break;
        }

        count += arr[i as usize] * k / (i * i);

        i += 1;
    }

    count
}

static SQRT: usize = 42000;

// Reference: https://math.stackexchange.com/questions/20529/fast-method-for-nth-squarefree-number-using-mathematica
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let k = scan.token::<i64>();
    let mut arr = vec![0; SQRT + 1];

    arr[1] = 1;

    for i in 1..=SQRT {
        for j in (i * 2..=SQRT).step_by(i) {
            arr[j] -= arr[i];
        }
    }

    let mut left = 0;
    let mut right = k * 2;

    while left < right - 1 {
        let mid = (left + right) / 2;

        if get_no_square_num_count(&arr, mid) < k {
            left = mid;
        } else {
            right = mid;
        }
    }

    writeln!(out, "{}", right).unwrap();
}
