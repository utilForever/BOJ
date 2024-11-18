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

fn calculate_max_dollars(
    producers: &Vec<(i64, i64)>,
    consumers: &Vec<(i64, i64)>,
    ret: &mut i64,
    left: usize,
    right: usize,
    opt_left: usize,
    opt_right: usize,
) {
    if left > right {
        return;
    }

    let mid = (left + right) / 2;
    let mut idx_opt = opt_left;
    let mut dollars_max = i64::MIN;

    for i in opt_left..=opt_right {
        let diff_x = consumers[i].0 - producers[mid].0;
        let diff_y = consumers[i].1 - producers[mid].1;

        let dollars = if diff_x < 0 && diff_y < 0 {
            0
        } else {
            diff_x * diff_y
        };

        if dollars > dollars_max {
            dollars_max = dollars;
            idx_opt = i;
        }
    }

    if *ret < dollars_max {
        *ret = dollars_max;
    }

    if left != mid {
        calculate_max_dollars(producers, consumers, ret, left, mid - 1, opt_left, idx_opt);
    }

    if mid != right {
        calculate_max_dollars(
            producers,
            consumers,
            ret,
            mid + 1,
            right,
            idx_opt,
            opt_right,
        );
    }
}

// Reference: https://ps.mjstudio.net/dnc-opt
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (m, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut producers = vec![(0, 0); m];
    let mut consumers = vec![(0, 0); n];

    for i in 0..m {
        producers[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    for i in 0..n {
        consumers[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut producers_new: Vec<(i64, i64)> = Vec::new();
    let mut consumers_new: Vec<(i64, i64)> = Vec::new();

    producers.sort();

    for producer in producers {
        if producers_new.is_empty() || producer.1 < producers_new.last().unwrap().1 {
            producers_new.push(producer);
        }
    }

    consumers.sort();
    consumers.reverse();

    for consumer in consumers {
        if consumers_new.is_empty() || consumer.1 > consumers_new.last().unwrap().1 {
            consumers_new.push(consumer);
        }
    }

    consumers_new.reverse();

    let mut ret = 0;

    calculate_max_dollars(
        &producers_new,
        &consumers_new,
        &mut ret,
        0,
        producers_new.len() - 1,
        0,
        consumers_new.len() - 1,
    );

    writeln!(out, "{ret}").unwrap();
}
