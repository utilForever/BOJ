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

fn valid_next(base: usize, val_min: usize, w: usize) -> Option<usize> {
    let start = base.max(val_min);
    let start_k = if start >= w { (start - w + 7) / 8 } else { 0 };

    let v = w + 8 * start_k;

    if v <= 9 * w && v >= start {
        Some(v)
    } else {
        None
    }
}

fn increase_eight(curr: usize, val_max: usize, want: isize) -> (usize, isize) {
    if want <= 0 {
        return (curr, 0);
    }

    let k_max = ((val_max as isize) - (curr as isize)) / 8;

    if k_max <= 0 {
        return (curr, 0);
    }

    let k_can = k_max as isize;
    let k_want = want / 8;
    let k_real = k_can.min(k_want);

    if k_real <= 0 {
        return (curr, 0);
    }

    let incr = 8 * k_real;
    let s_new = (curr as isize + incr) as usize;

    (s_new, incr)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let h = scan.token::<usize>();
    let mut w = vec![0; h];

    for i in 0..h {
        w[i] = scan.token::<usize>();
    }

    let (_, d) = (scan.token::<i64>(), scan.token::<i64>());
    let w_sum = w.iter().sum::<usize>() as i64;
    let total = w_sum + 8 * d;

    let mut s = vec![0; h];
    let mut prev;

    if let Some(val_first) = valid_next(w[0], w[0], w[0]) {
        s[0] = val_first;
        prev = val_first;
    } else {
        writeln!(out, "-1").unwrap();
        return;
    }

    for i in 1..h {
        let min = prev + 1;

        if let Some(val) = valid_next(min, w[i], w[i]) {
            s[i] = val;
            prev = val;
        } else {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    let s_sum = s.iter().sum::<usize>() as i64;

    if s_sum > total {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut remain = total as isize - s_sum as isize;

    for i in (0..h).rev() {
        if remain <= 0 {
            break;
        }

        let upper_bound = 9 * w[i];

        let val_max = if i + 1 < h {
            let limit = if s[i + 1] == 0 { 0 } else { s[i + 1] - 1 };
            upper_bound.min(limit)
        } else {
            upper_bound
        };

        if s[i] >= val_max {
            continue;
        }

        let (s_new, used) = increase_eight(s[i], val_max, remain);

        remain -= used;
        s[i] = s_new;
    }

    if remain != 0 {
        writeln!(out, "-1").unwrap();
        return;
    }

    for i in 1..h {
        if s[i] <= s[i - 1] {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    let mut nine_used = 0;

    for i in 0..h {
        let diff = s[i] as isize - w[i] as isize;

        if diff < 0 || diff % 8 != 0 {
            writeln!(out, "-1").unwrap();
            return;
        }

        let nine_cnt = (diff / 8) as usize;

        if nine_cnt > w[i] {
            writeln!(out, "-1").unwrap();
            return;
        }

        nine_used += nine_cnt;
    }

    if nine_used != d as usize {
        writeln!(out, "-1").unwrap();
        return;
    }

    for i in 0..h {
        let diff = s[i] as isize - w[i] as isize;
        let cnt_nine = (diff / 8) as usize;
        let cnt_one = w[i] - cnt_nine;

        for _ in 0..cnt_nine {
            write!(out, "9 ").unwrap();
        }

        for _ in 0..cnt_one {
            write!(out, "1 ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
