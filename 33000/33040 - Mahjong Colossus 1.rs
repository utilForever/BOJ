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

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let (_, d) = (scan.token::<usize>(), scan.token::<usize>());

    if h > w + 1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let s_min = (h - 1) * h / 2;

    if d < s_min {
        writeln!(out, "-1").unwrap();
        return;
    }

    let w_sum = w * (w + 1) / 2;
    let wh_sum_part = if w >= h { (w - h) * (w - h + 1) / 2 } else { 0 };

    let s_max = w_sum - wh_sum_part;

    if d > s_max {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut nines = vec![0; h + 1];

    for i in 0..h {
        nines[i] = i;
    }

    nines[h] = w + 1;

    let mut remain = d as isize - s_min as isize;

    for i in (0..h).rev() {
        if remain <= 0 {
            break;
        }

        let can_add = (nines[i + 1] - 1) as isize - nines[i] as isize;

        if can_add > 0 {
            let add = can_add.min(remain);

            nines[i] = (nines[i] as isize + add) as usize;
            remain -= add;
        }
    }

    if remain != 0 {
        writeln!(out, "-1").unwrap();
        return;
    }

    for i in 0..h {
        let cnt_nine = nines[i];
        let cnt_one = w - cnt_nine;

        for _ in 0..cnt_nine {
            write!(out, "9 ").unwrap();
        }

        for _ in 0..cnt_one {
            write!(out, "1 ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
