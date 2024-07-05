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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, w, l) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut trucks = vec![0; n];

    for i in 0..n {
        trucks[i] = scan.token::<i64>();
    }

    let mut queue: VecDeque<(usize, i64)> = VecDeque::new();
    let mut time = 0;
    let mut sum = 0;

    for i in 0..n {
        // Remove trucks that have passed the bridge
        while !queue.is_empty() && queue.front().unwrap().0 + w <= time {
            sum -= queue.pop_front().unwrap().1;
        }

        // Remove trucks that exceed the weight limit
        while sum + trucks[i] > l {
            time = queue.front().unwrap().0 + w;
            sum -= queue.pop_front().unwrap().1;
        }

        // Add truck to the bridge
        // If the bridge is not full, add the truck to the bridge
        // If the bridge is full, remove the truck that has passed the bridge and add the truck to the bridge
        if queue.len() < w {
            queue.push_back((time, trucks[i]));
            sum += trucks[i];
        } else {
            time = queue.front().unwrap().0 + w;
            sum -= queue.pop_front().unwrap().1;
            queue.push_back((time, trucks[i]));
            sum += trucks[i];
        }

        time += 1;
    }

    time += w;

    writeln!(out, "{time}").unwrap();
}
