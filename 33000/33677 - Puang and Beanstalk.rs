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

    let n = scan.token::<usize>();

    if n == 0 {
        writeln!(out, "0 0").unwrap();
        return;
    }

    let mut dp = vec![(-1, -1); n + 1];
    let mut queue = VecDeque::new();

    dp[0] = (0, 0);
    queue.push_back((0, 0, 0));

    while let Some((height, day, water)) = queue.pop_front() {
        let day_new = day + 1;

        // Operation 1
        if height + 1 <= n {
            let (height_new, water_new) = (height + 1, water + 1);
            let need_update = match dp[height_new] {
                (-1, -1) => true,
                (day_old, water_old) => {
                    day_new < day_old || (day_new == day_old && water_new < water_old)
                }
            };

            if need_update {
                dp[height_new] = (day_new, water_new);
                queue.push_back((height_new, day_new, water_new));
            }
        }

        // Operation 2
        if height > 0 && height * 3 <= n {
            let (height_new, water_new) = (height * 3, water + 3);
            let need_update = match dp[height_new] {
                (-1, -1) => true,
                (day_old, water_old) => {
                    day_new < day_old || (day_new == day_old && water_new < water_old)
                }
            };

            if need_update {
                dp[height_new] = (day_new, water_new);
                queue.push_back((height_new, day_new, water_new));
            }
        }

        // Operation 3
        if height > 0 && height * height <= n {
            let (height_new, water_new) = (height * height, water + 5);
            let need_update = match dp[height_new] {
                (-1, -1) => true,
                (day_old, water_old) => {
                    day_new < day_old || (day_new == day_old && water_new < water_old)
                }
            };

            if need_update {
                dp[height_new] = (day_new, water_new);
                queue.push_back((height_new, day_new, water_new));
            }
        }
    }

    writeln!(out, "{} {}", dp[n].0, dp[n].1).unwrap();
}
