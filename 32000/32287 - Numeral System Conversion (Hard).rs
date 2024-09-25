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

// Generates all possible number count combinations.
// For each combination, compare with count_digits and accumulate the number of cases in the order of sorting.
// If the number of counts is the same, use the compute_permutation_rank function to calculate the permutation order.
fn generate_counts(
    digits_rev: &Vec<usize>,
    count_digits: &Vec<i64>,
    factorial: &Vec<i64>,
    counts: &mut Vec<i64>,
    ret: &mut i64,
    pos: usize,
    count_sum: usize,
    n: usize,
    m: usize,
) {
    if pos == m {
        if count_sum != n {
            return;
        }

        match compare_counts(counts, count_digits) {
            Ordering::Less => {
                let permutation = combination(counts, factorial, n);
                *ret += permutation;
            }
            Ordering::Equal => {
                let mut counts_copy = counts.clone();
                let rank = compute_permutation_rank(&digits_rev, factorial, &mut counts_copy);

                *ret += rank - 1;
            }
            Ordering::Greater => {}
        }

        return;
    }

    let count_max = n - count_sum;

    for c in 0..=count_max {
        counts[pos] = c as i64;

        generate_counts(
            digits_rev,
            count_digits,
            factorial,
            counts,
            ret,
            pos + 1,
            count_sum + c,
            n,
            m,
        );

        counts[pos] = 0;
    }
}

// Compare the number of numbers according to the sorting criteria (M-1 to 1 in reverse order).
fn compare_counts(a: &Vec<i64>, b: &Vec<i64>) -> Ordering {
    for i in (1..a.len()).rev() {
        if a[i] != b[i] {
            return a[i].cmp(&b[i]);
        }
    }

    Ordering::Equal
}

// Calculate the number of permutations that can be made from the given number of combinations.
fn combination(counts: &Vec<i64>, factorial: &Vec<i64>, n: usize) -> i64 {
    let mut denominator = 1;

    for &c in counts.iter() {
        denominator *= factorial[c as usize];
    }

    factorial[n] / denominator
}

// Calculate the order of the given permutation (reversed S).
// For each digit, accumulate the number of all permutations that can be made from the remaining digits using the smallest possible number.
fn compute_permutation_rank(
    permutation: &Vec<usize>,
    factorial: &Vec<i64>,
    counts: &mut Vec<i64>,
) -> i64 {
    let mut rank = 1;

    for i in 0..permutation.len() {
        let digit = permutation[i];

        for d in 0..digit {
            if counts[d] == 0 {
                continue;
            }

            counts[d] -= 1;

            let permutation_new = combination(counts, factorial, permutation.len() - i - 1);

            rank += permutation_new;
            counts[d] += 1;
        }

        if counts[digit] == 0 {
            break;
        }

        counts[digit] -= 1;
    }

    rank
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let s = scan.token::<String>();
    let digits = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let digits_rev = digits.iter().rev().cloned().collect::<Vec<_>>();

    let mut count_digits = vec![0; m];

    for &d in digits.iter() {
        count_digits[d] += 1;
    }

    let mut factorial = vec![1; n + 1];

    for i in 1..=n {
        factorial[i] = factorial[i - 1] * i as i64;
    }

    let mut counts = vec![0; m];
    let mut ret = 0;

    generate_counts(
        &digits_rev,
        &count_digits,
        &factorial,
        &mut counts,
        &mut ret,
        0,
        0,
        n,
        m,
    );

    writeln!(out, "{ret}").unwrap();
}
