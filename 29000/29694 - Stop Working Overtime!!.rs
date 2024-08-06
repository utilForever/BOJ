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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, a, b, m) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut works = vec![(0, 0); n];

    // Case 4: No constraints
    for i in 0..n {
        works[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    works.sort();

    if a == 0 && b == 0 && m != 0 {
        writeln!(out, "-1").unwrap();
        return;
    }

    'find_x: for mut x in (0..=100).rev() {
        let y = if b == 0 {
            i64::MAX
        } else if a * x <= m && (m - a * x) % b == 0 {
            (m - a * x) / b
        } else {
            continue;
        };

        if a == 0 {
            x = i64::MAX;
        }

        let mut days = 0;
        let mut num_weekdays_day = 0;
        let mut num_weekdays_night = 0;
        let mut num_weekends = 0;
        let mut used_weekdays = 0;
        let mut used_weekends = 0;
    
        for &(d, mut t) in works.iter() {
            while days < d {
                if days % 7 == 5 || days % 7 == 6 {
                    num_weekends += 1;
                } else {
                    num_weekdays_day += 1;
                    num_weekdays_night += 1;
                }
    
                days += 1;
            }

            if b > 0 {
                let days_worked = t.min(num_weekends);
                t -= days_worked;
                num_weekends -= days_worked;
                used_weekends += days_worked;

                let recover = used_weekends - y.min(used_weekends);
                used_weekends -= recover;
                num_weekends += recover;
                t += recover;
            }
    
            let days_worked = t.min(num_weekdays_night);
            t -= days_worked;
            num_weekdays_night -= days_worked;
            used_weekdays += days_worked;

            let recover = used_weekdays - x.min(used_weekdays);
            used_weekdays -= recover;
            num_weekdays_night += recover;
            t += recover;

            let days_worked = t.min(num_weekdays_day);
            t -= days_worked;
            num_weekdays_day -= days_worked;

            if b == 0 {
                let days_worked = t.min(num_weekends);
                t -= days_worked;
                num_weekends -= days_worked;
                used_weekends += days_worked;
            }
    
            if t > 0 {
                continue 'find_x;
            }
        }
    
        if (a == 0 || x == used_weekdays) && (b == 0 || y == used_weekends) {
            writeln!(out, "{used_weekends}").unwrap();
            return;
        }
    }

    writeln!(out, "-1").unwrap();
}
