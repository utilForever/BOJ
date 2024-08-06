use io::Write;
use std::{
    collections::{BTreeMap, BTreeSet},
    io, str,
};

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

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let mut log_youtubers = BTreeSet::new();
    let mut log_broadcast: BTreeMap<String, Vec<i64>> = BTreeMap::new();

    for _ in 0..n {
        let (name, day, start, end) = (
            scan.token::<String>(),
            scan.token::<usize>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );
        let (start_hour, start_min) = (
            start[0..2].parse::<i64>().unwrap(),
            start[3..5].parse::<i64>().unwrap(),
        );
        let (end_hour, end_min) = (
            end[0..2].parse::<i64>().unwrap(),
            end[3..5].parse::<i64>().unwrap(),
        );
        let time_start = start_hour * 60 + start_min;
        let time_end = end_hour * 60 + end_min;

        log_youtubers.insert(name.clone());

        if !log_broadcast.contains_key(&name) {
            log_broadcast.insert(name.clone(), vec![0; m + 1]);
        }

        log_broadcast.get_mut(&name).unwrap()[day] = time_end - time_start;
    }

    for (youtuber, broadcast) in log_broadcast {
        for i in (1..=m).step_by(7) {
            let mut cnt_broadcast = 0;
            let mut time_total = 0;

            for j in i..i + 7 {
                cnt_broadcast += if broadcast[j] > 0 { 1 } else { 0 };
                time_total += broadcast[j];
            }

            if cnt_broadcast < 5 || time_total < 60 * 60 {
                log_youtubers.remove(&youtuber);
            }
        }
    }

    if log_youtubers.is_empty() {
        writeln!(out, "-1").unwrap();
    } else {
        for youtuber in log_youtubers {
            writeln!(out, "{youtuber}").unwrap();
        }
    }
}
