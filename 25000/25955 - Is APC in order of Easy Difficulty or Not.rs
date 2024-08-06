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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut problems = vec![String::new(); n];

    for i in 0..n {
        problems[i] = scan.token::<String>();
    }

    let mut sorted_problems = problems.clone();

    sorted_problems.sort_by(|a, b| {
        let tier_a = a.chars().nth(0).unwrap();
        let tier_b = b.chars().nth(0).unwrap();
        
        let tier_a = match tier_a {
            'B' => 0,
            'S' => 1,
            'G' => 2,
            'P' => 3,
            'D' => 4,
            _ => panic!("Invalid problem type"),
        };

        let tier_b = match tier_b {
            'B' => 0,
            'S' => 1,
            'G' => 2,
            'P' => 3,
            'D' => 4,
            _ => panic!("Invalid problem type"),
        };

        if tier_a == tier_b {
            let num_a = a[1..].parse::<i64>().unwrap();
            let num_b = b[1..].parse::<i64>().unwrap();

            num_b.cmp(&num_a)
        } else {
            tier_a.cmp(&tier_b)
        }
    });

    let mut is_same = true;
    let (mut idx1, mut idx2) = (None, None);

    for i in 0..n {
        if problems[i] != sorted_problems[i] {
            is_same = false;

            if idx1 == None {
                idx1 = Some(i);
            } else {
                idx2 = Some(i);
            }
        }
    }

    writeln!(out, "{}", if is_same { "OK" } else { "KO" }).unwrap();

    if !is_same {
        writeln!(
            out,
            "{} {}",
            problems[idx2.unwrap()],
            problems[idx1.unwrap()]
        )
        .unwrap();
    }
}
