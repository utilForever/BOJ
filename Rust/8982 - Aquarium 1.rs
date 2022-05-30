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
    let mut coords = vec![(0, 0); n / 2];

    for i in 0..n / 2 {
        (coords[i].0, coords[i].1) = (scan.token::<usize>(), scan.token::<usize>());
        (_, _) = (scan.token::<usize>(), scan.token::<usize>());
    }

    let mut ret = 0;

    for i in 1..n / 2 {
        ret += coords[i].1 * (coords[i].0 - coords[i - 1].0);
    }

    let k = scan.token::<usize>();
    let mut holes = vec![(0, 0); k];
    let mut heights = vec![0; n / 2];

    for i in 0..k {
        (holes[i].0, holes[i].1) = (scan.token::<usize>(), scan.token::<usize>());
        (_, _) = (scan.token::<usize>(), scan.token::<usize>());
    }

    holes.sort();

    for i in 0..k {
        let mut idx = 1;

        while !(coords[idx].1 == holes[i].1
            && coords[idx - 1].0 <= holes[i].0
            && holes[i].0 <= coords[idx].0)
        {
            idx += 1;

            if idx == n / 2 {
                break;
            }
        }

        if heights[idx] == coords[idx].1 {
            continue;
        }

        ret -= (holes[i].1 - heights[idx]) * (coords[idx].0 - coords[idx - 1].0);
        heights[idx] = holes[i].1;

        // Left-side
        let mut height_left = holes[i].1;
        let mut idx_left = idx - 1;

        while idx_left >= 1 {
            height_left = cmp::min(height_left, coords[idx_left].1);

            if height_left == 0 || heights[idx_left] == height_left {
                break;
            }

            ret -=
                (height_left - heights[idx_left]) * (coords[idx_left].0 - coords[idx_left - 1].0);
            heights[idx_left] = height_left;
            idx_left -= 1;
        }

        // Right-side
        let mut height_right = holes[i].1;
        let mut idx_right = idx + 1;

        while idx_right < n / 2 {
            height_right = cmp::min(height_right, coords[idx_right].1);

            if height_right == 0 || heights[idx_right] == height_right {
                break;
            }

            ret -= (height_right - heights[idx_right])
                * (coords[idx_right].0 - coords[idx_right - 1].0);
            heights[idx_right] = height_right;
            idx_right += 1;
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
