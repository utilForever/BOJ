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

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut levels_character = vec![0; n];
    let mut sum_levels = vec![0; 201];
    let mut monsters = vec![0; m + 1];

    for i in 0..n {
        let level = scan.token::<usize>();
        levels_character[i] = level;

        for j in 0..k {
            if level + j > 200 {
                break;
            }

            sum_levels[level + j] += 1;
        }
    }

    for i in 1..=m {
        monsters[i] = scan.token::<usize>();
    }

    let mut towers = vec![0; m + 1];

    for i in 1..200 {
        if sum_levels[i] == 0 {
            continue;
        }

        let mut idx_max = 0;
        let mut level_max = 0;

        for j in 1..=m {
            if monsters[j] > level_max && monsters[j] <= i {
                idx_max = j;
                level_max = monsters[j];
            }
        }

        towers[idx_max] += sum_levels[i];
    }

    let mut origin = 0;

    for i in 1..=m {
        origin += (i - 1) * towers[i];
    }

    let mut ret = origin;
    let mut tower_pos1 = 1;
    let mut tower_pos2 = 2;

    for i in 1..=m {
        for j in i + 1..=m {
            let mut val = 0;

            for k in 1..=m {
                let cnt_move = (k - 1).min((i - 1) + (k as i64 - j as i64).abs() as usize);
                val += cnt_move * towers[k];
            }

            if ret > val {
                ret = val;
                tower_pos1 = i;
                tower_pos2 = j;
            }
        }
    }

    writeln!(out, "{tower_pos1} {tower_pos2}").unwrap();
    writeln!(out, "{}", origin - ret).unwrap();
}
