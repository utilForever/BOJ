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

const B: usize = 9;
const Q: usize = 6;
const V: [[u8; Q]; B] = [
    [1, 1, 1, 1, 1, 1],
    [0, 1, 1, 1, 1, 1],
    [1, 0, 1, 1, 1, 1],
    [1, 1, 0, 1, 1, 1],
    [0, 1, 1, 0, 1, 1],
    [0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 1, 0],
    [0, 0, 0, 1, 1, 1],
    [1, 0, 1, 0, 0, 1],
];

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let n = scan.token::<usize>();
    let mut query = "? ".to_string();

    for _ in 0..n {
        query.push('0');
    }

    println!("{query}");

    let cnt_strikes_init = scan.token::<usize>();
    let blocks = (n + B - 1) / B;
    let mut data = vec!['0'; n];
    let mut ret = vec!['0'; n];

    for b in 0..blocks {
        let start = b * B;
        let len = B.min(n - start);
        let mut ones_in_subset = vec![0; Q];

        for j in 0..Q {
            let mut positions = Vec::with_capacity(B);

            for i in 0..len {
                if V[i][j] == 1 {
                    positions.push(start + i);
                }
            }

            for &pos in positions.iter() {
                data[pos] = '1';
            }

            let mut query = "? ".to_string();

            for i in 0..n {
                query.push(data[i]);
            }

            println!("{query}");

            let cnt_ones = positions.len();
            let cnt_strikes = scan.token::<usize>();

            ones_in_subset[j] = (cnt_strikes - cnt_strikes_init + cnt_ones) / 2;

            for &pos in positions.iter() {
                data[pos] = '0';
            }
        }

        for mask in 0u16..(1 << len) {
            let mut sum = vec![0; Q];

            for i in 0..len {
                if ((mask >> i) & 1) == 1 {
                    for j in 0..Q {
                        sum[j] += V[i][j] as usize;
                    }
                }
            }

            if sum == ones_in_subset {
                for i in 0..len {
                    ret[start + i] = if ((mask >> i) & 1) == 1 { '1' } else { '0' };
                }

                break;
            }
        }
    }

    let mut query = "! ".to_string();

    for i in 0..n {
        query.push(ret[i]);
    }

    println!("{query}");
}
