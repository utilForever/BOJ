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

    let n = scan.token::<usize>();
    let mut temperatures = vec![0; n - 1];

    for i in 0..n - 1 {
        temperatures[i] = scan.token::<usize>();
    }

    let mut idx_waters = vec![0; n];

    for i in 0..n {
        idx_waters[i] = scan.token::<usize>();
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![vec![vec![false; 101]; 3]; 101]; 3];
    let mut tracking = vec![vec![vec![vec![(0, 0, 0, 0); 101]; 3]; 101]; 3];
    let mut can_move = false;
    let mut ret = Vec::new();

    queue.push_back((0, 100, 1, 100));
    visited[0][100][1][100] = true;
    tracking[0][100][1][100] = (-1, -1, -1, -1);

    while !queue.is_empty() {
        let (idx1, temperature1, idx2, temperature2) = queue.pop_front().unwrap();

        if idx_waters[idx1] == 1
            && temperatures[0] == temperature1
            && idx_waters[idx2] == 2
            && temperatures[1] == temperature2
        {
            can_move = true;

            let mut curr_idx1 = idx1;
            let mut curr_temperature1 = temperature1;
            let mut curr_idx2 = idx2;
            let mut curr_temperature2 = temperature2;

            loop {
                let (parent_idx1, parent_temperature1, parent_idx2, parent_temperature2) =
                    tracking[curr_idx1][curr_temperature1][curr_idx2][curr_temperature2];

                if parent_idx1 == -1
                    && parent_temperature1 == -1
                    && parent_idx2 == -1
                    && parent_temperature2 == -1
                {
                    break;
                }

                if parent_idx1 as usize != curr_idx1 {
                    ret.push((parent_idx1 as usize, curr_idx1));
                }

                if parent_idx2 as usize != curr_idx2 {
                    ret.push((parent_idx2 as usize, curr_idx2));
                }

                curr_idx1 = parent_idx1 as usize;
                curr_temperature1 = parent_temperature1 as usize;
                curr_idx2 = parent_idx2 as usize;
                curr_temperature2 = parent_temperature2 as usize;
            }

            ret.reverse();

            break;
        }

        let cup_rest = if (idx1 == 0 && idx2 == 1) || (idx1 == 1 && idx2 == 0) {
            2
        } else if (idx1 == 0 && idx2 == 2) || (idx1 == 2 && idx2 == 0) {
            1
        } else {
            0
        };

        if temperature1 > 0 && !visited[cup_rest][temperature1 - 5][idx2][temperature2] {
            visited[cup_rest][temperature1 - 5][idx2][temperature2] = true;
            tracking[cup_rest][temperature1 - 5][idx2][temperature2] = (
                idx1 as i64,
                temperature1 as i64,
                idx2 as i64,
                temperature2 as i64,
            );
            queue.push_back((cup_rest, temperature1 - 5, idx2, temperature2));
        }

        if temperature2 > 0 && !visited[idx1][temperature1][cup_rest][temperature2 - 5] {
            visited[idx1][temperature1][cup_rest][temperature2 - 5] = true;
            tracking[idx1][temperature1][cup_rest][temperature2 - 5] = (
                idx1 as i64,
                temperature1 as i64,
                idx2 as i64,
                temperature2 as i64,
            );
            queue.push_back((idx1, temperature1, cup_rest, temperature2 - 5));
        }
    }

    if can_move {
        writeln!(out, "{}", ret.len()).unwrap();

        for (idx1, idx2) in ret {
            writeln!(out, "{} {}", idx1 + 1, idx2 + 1).unwrap();
        }
    } else {
        writeln!(out, "-1").unwrap();
    }
}
