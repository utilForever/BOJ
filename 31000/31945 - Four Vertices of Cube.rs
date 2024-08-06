use io::Write;
use std::{cmp::Ordering, io, str};

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

pub fn next_permutation(nums: &mut Vec<usize>) -> bool {
    let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            nums.reverse();
            return false;
        }
    };

    let swap_with = nums[last_ascending + 1..]
        .binary_search_by(|n| usize::cmp(&nums[last_ascending], n).then(Ordering::Less))
        .unwrap_err();
    nums.swap(last_ascending, last_ascending + swap_with);
    nums[last_ascending + 1..].reverse();

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let vertices = [
        [0, 0, 0],
        [0, 0, 1],
        [0, 1, 0],
        [0, 1, 1],
        [1, 0, 0],
        [1, 0, 1],
        [1, 1, 0],
        [1, 1, 1],
    ];

    for _ in 0..t {
        let (a, b, c, d) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let (v1, v2, v3, v4) = (vertices[a], vertices[b], vertices[c], vertices[d]);

        writeln!(
            out,
            "{}",
            if v1[0] == v2[0] && v2[0] == v3[0] && v3[0] == v4[0]
                || v1[1] == v2[1] && v2[1] == v3[1] && v3[1] == v4[1]
                || v1[2] == v2[2] && v2[2] == v3[2] && v3[2] == v4[2]
            {
                "YES"
            } else {
                "NO"
            }
        )
        .unwrap();
    }
}
