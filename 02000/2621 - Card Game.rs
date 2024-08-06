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

    let mut cards = [(' ', 0); 5];

    for i in 0..5 {
        cards[i] = (scan.token::<char>(), scan.token::<u8>());
    }

    cards.sort_by(|a, b| b.1.cmp(&a.1));

    // Rule 1
    let all_same_color = cards.iter().all(|&x| x.0 == cards[0].0);
    let continuous_numbers = cards.windows(2).all(|x| x[0].1 == x[1].1 + 1);

    if all_same_color && continuous_numbers {
        writeln!(out, "{}", 900 + cards[0].1 as i64).unwrap();
        return;
    }

    // Rule 2
    let mut nums = [0; 10];

    for i in 0..5 {
        nums[cards[i].1 as usize] += 1;
    }

    if nums.contains(&4) {
        writeln!(out, "{}", 800 + cards[2].1 as i64).unwrap();
        return;
    }

    // Rule 3
    if nums.contains(&3) && nums.contains(&2) {
        let pos1 = nums.iter().position(|&x| x == 3).unwrap();
        let pos2 = nums.iter().position(|&x| x == 2).unwrap();

        writeln!(out, "{}", 700 + pos1 as i64 * 10 + pos2 as i64).unwrap();
        return;
    }

    // Rule 4
    if all_same_color {
        writeln!(out, "{}", 600 + cards[0].1 as i64).unwrap();
        return;
    }

    // Rule 5
    if continuous_numbers {
        writeln!(out, "{}", 500 + cards[0].1 as i64).unwrap();
        return;
    }

    // Rule 6
    if nums.contains(&3) {
        let pos = nums.iter().position(|&x| x == 3).unwrap();

        writeln!(out, "{}", 400 + pos as i64).unwrap();
        return;
    }

    // Rule 7
    if nums.iter().filter(|&x| *x == 2).count() == 2 {
        let pos1 = nums.iter().position(|&x| x == 2).unwrap();
        let pos2 = nums.iter().rposition(|&x| x == 2).unwrap();

        writeln!(out, "{}", 300 + pos1 as i64 + pos2 as i64 * 10).unwrap();
        return;
    }

    // Rule 8
    if nums.contains(&2) {
        let pos = nums.iter().position(|&x| x == 2).unwrap();

        writeln!(out, "{}", 200 + pos as i64).unwrap();
        return;
    }

    // Rule 9
    writeln!(out, "{}", 100 + cards[0].1 as i64).unwrap();
}
