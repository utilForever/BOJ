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

#[derive(Default)]
struct Line {
    slope: i64,
    intercept: i64,
}

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let (n, mut k) = (scan.token::<i64>(), scan.token::<i64>());
    let mut vals = Vec::new();
    let (mut left, mut right) = (1, n);
    let mut recent = 1;
    let mut streak = 0;
    let mut sum = 0;

    loop {
        if right - left + 1 == k {
            print!("? ");

            for i in left..left + k {
                print!("{i} ");
            }

            for i in right + 1..=n {
                print!("{i} ");
            }

            println!();

            let val = scan.token::<i64>();
            sum += val;

            vals.push((streak + 1, sum));
            break;
        } else if right - left + 1 >= 2 * k {
            if recent == 2 {
                vals.push((-streak, sum));
                streak = 0;
                sum = 0;
            }

            print!("? ");

            for i in left..left + k {
                print!("{i} ");
            }

            for i in right + 1..=n {
                print!("{i} ");
            }

            println!();

            let val = scan.token::<i64>();
            sum += val;

            left += k;
            recent = 1;
            streak += 1;
        } else {
            if recent == 1 {
                vals.push((streak, sum));
                streak = 0;
                sum = 0;
            }

            print!("? ");

            for i in left..left + k {
                print!("{i} ");
            }

            for i in right + 1..=n {
                print!("{i} ");
            }

            println!();

            let val = scan.token::<i64>();
            sum += val;

            let total = right - left + 1 - k;
            right -= total;
            k -= total;
            recent = 2;
            streak += 1;
        }
    }

    let mut sum_left = Line::default();
    let mut sum_right = Line::default();

    for (mut streak, sum) in vals {
        if streak > 0 {
            sum_left.slope -= sum_right.slope * streak;
            sum_left.intercept += sum - sum_right.intercept * streak;
        } else {
            streak *= -1;

            let mut temp = Line::default();
            temp.slope = (1 - sum_left.slope) * streak;
            temp.intercept = -sum_left.intercept * streak - sum;

            sum_right.slope += temp.slope;
            sum_right.intercept += temp.intercept;
        }
    }

    println!(
        "! {}",
        (sum_left.intercept + sum_right.intercept) / (1 - sum_left.slope - sum_right.slope)
    );
}
