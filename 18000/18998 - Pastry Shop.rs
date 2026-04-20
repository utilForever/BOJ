use io::Write;
use std::{collections::VecDeque, io, str};

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

fn add_range(diff: &mut Vec<(i64, i64)>, t: &Vec<i64>, l: usize, r: usize, k: usize, cnt: i64) {
    if cnt == 0 || l > r {
        return;
    }

    let a = cnt * t[k];
    let b = -cnt * (k as i64);

    diff[l].0 += a;
    diff[r + 1].0 -= a;
    diff[l].1 += b;
    diff[r + 1].1 -= b;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut times = vec![0; n + 1];
    let mut durations = vec![0; m];

    for i in 1..=n {
        times[i] = scan.token::<i64>();
    }

    for i in 0..m {
        durations[i] = scan.token::<usize>();
    }

    let times_sum = times.iter().sum::<i64>();
    let duration_max = *durations.iter().max().unwrap();
    let mut hull = VecDeque::new();
    let mut diff = vec![(0, 0); duration_max + 2];

    hull.push_back((1, duration_max, 0, 1));

    for left in 1..=n {
        let mut right = 0;

        while let Some(&(l, r, k, start)) = hull.front() {
            let x = ((times[left] - times[k]) / ((left - k) as i64)).min(duration_max as i64);

            if x < (l as i64) {
                break;
            }

            let cut = r.min(x as usize);

            add_range(&mut diff, &times, l, cut, k, (left - start) as i64);

            right = cut;

            if cut < r {
                hull.front_mut().unwrap().0 = cut + 1;
                break;
            } else {
                hull.pop_front();
            }
        }

        if right > 0 {
            hull.push_front((1, right, left, left));
        }
    }

    for (l, r, k, start) in hull {
        add_range(&mut diff, &times, l, r, k, (n - start + 1) as i64);
    }

    let triangle = (n as i64) * ((n + 1) as i64) / 2;
    let mut a = 0;
    let mut b = 0;
    let mut ret = vec![0; duration_max + 1];

    for d in 1..=duration_max {
        a += diff[d].0;
        b += diff[d].1;

        let prefix_max_sum = a + b * (d as i64);
        ret[d] = (d as i64) * triangle + prefix_max_sum - times_sum;
    }

    for i in 0..m {
        writeln!(out, "{}", ret[durations[i]]).unwrap();
    }
}
