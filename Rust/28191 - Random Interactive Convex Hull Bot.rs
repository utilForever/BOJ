use std::{
    io::{self, StdinLock},
    str,
};

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
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    // NOTE: 1 = Counter-clockwise, -1 = Clockwise
    // IMPORTANT: The result should be counter-clockwise
    let query = |scan: &mut UnsafeScanner<StdinLock<'_>>, i: usize, j: usize, k: usize| -> i64 {
        println!("? {i} {j} {k}");

        let ret = scan.token::<i64>();
        ret
    };

    let n = scan.token::<usize>();
    let mut ret = if query(&mut scan, 1, 2, 3) == 1 {
        vec![1, 2, 3]
    } else {
        vec![1, 3, 2]
    };

    for i in 4..=n {
        let mut left = 0;
        let mut right = ret.len() - 1;

        // Query to check whether the new point lies in the same halfplane using binary search
        while left < right {
            let mid = (left + right + 1) / 2;

            if query(&mut scan, ret[0], ret[mid], i) == 1 {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        // If it is inside, done
        if left > 0 && left < ret.len() - 1 && query(&mut scan, ret[left], ret[left + 1], i) == 1 {
            continue;
        }

        // If it is outside, calculate where to insert the new point
        if left + 1 < ret.len() {
            ret.rotate_left(left + 1);
        }

        // Remove the points near it while it is not convex
        while ret.len() > 2 && query(&mut scan, i, ret[1], ret[0]) == 1 {
            ret.remove(0);
        }

        while ret.len() > 2 && query(&mut scan, i, ret[ret.len() - 1], ret[ret.len() - 2]) == 1 {
            ret.remove(ret.len() - 1);
        }

        // Insert new point
        ret.insert(0, i);
    }

    print!("! {}", ret.len());

    for idx in ret {
        print!(" {idx}");
    }
}
