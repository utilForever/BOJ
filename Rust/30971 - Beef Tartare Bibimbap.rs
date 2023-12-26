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

#[derive(Default, Clone)]
struct BeefTartare {
    sweet: i64,
    saltness: i64,
    wits: i64,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut beef_tartares = vec![BeefTartare::default(); n];

    for i in 0..n {
        beef_tartares[i].sweet = scan.token::<i64>();
    }

    for i in 0..n {
        beef_tartares[i].saltness = scan.token::<i64>();
    }

    for i in 0..n {
        beef_tartares[i].wits = scan.token::<i64>();
    }

    let mut permutations = (0..n).collect::<Vec<usize>>();
    let mut ret = -1;

    loop {
        let mut check = true;
        
        for i in 1..n {
            if beef_tartares[permutations[i - 1]].wits * beef_tartares[permutations[i]].wits > k {
                check = false;
                break;
            }
        }

        if check {
            let mut tasty = 0;

            for i in 1..n {
                tasty += beef_tartares[permutations[i - 1]].sweet
                    * beef_tartares[permutations[i]].saltness;
            }

            ret = ret.max(tasty);
        }

        if !next_permutation(&mut permutations) {
            break;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
