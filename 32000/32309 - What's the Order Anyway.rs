use io::Write;
use std::{cmp::Ordering, io, str};

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

pub fn next_permutation(nums: &mut Vec<usize>) -> bool {
    let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            nums.reverse();
            return false;
        }
    };

    let swap_with = nums[last_ascending + 1..]
        .binary_search_by(|n| usize::cmp(&nums[last_ascending], n).then(Ordering::Less))
        .unwrap_err();
    nums.swap(last_ascending, last_ascending + swap_with);
    nums[last_ascending + 1..].reverse();

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut actors = (0..n).collect::<Vec<usize>>();
    let mut clues = vec![(0, 0, 0); c];

    for i in 0..c {
        let (a, x, y) = (
            scan.token::<i64>(),
            scan.token::<char>(),
            scan.token::<char>(),
        );
        clues[i] = (a, (x as u8 - b'A') as usize, (y as u8 - b'A') as usize);
    }

    let mut ret = 0;

    loop {
        let mut check = true;

        for clue in clues.iter() {
            let (a, x, y) = *clue;
            let pos_x = actors.iter().position(|&actor| actor == x).unwrap();
            let pos_y = actors.iter().position(|&actor| actor == y).unwrap();

            if (a == 1 && pos_x < pos_y)
                || (a == 2 && pos_x > pos_y)
                || (a == 3 && (pos_x as i64 - pos_y as i64).abs() == 1)
            {
                check = false;
                break;
            }
        }

        if check {
            ret += 1;
        }

        if !next_permutation(&mut actors) {
            break;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
