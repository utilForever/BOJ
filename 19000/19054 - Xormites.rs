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

fn process_game(nums: &Vec<i64>, mut left: usize, mut right: usize) -> bool {
    let mut num_ones = 0;

    for i in left..=right {
        if nums[i] == 1 {
            num_ones += 1;
        }
    }

    // If the number of ones remains "2", the first player can't win.
    // Because XOR operation filps 0 and 1, original -> fliped -> original.
    if (num_ones & 0b10) != 0 {
        return false;
    }

    // The only sequence that the first player can defend is S + aabbccdd... + inv(S)
    // Calculate left/right in case of S and inv(S)
    while left < right && nums[left] == nums[right] {
        left += 1;
        right -= 1;
    }

    // Check aabbccdd...
    let mut i = left;

    while i <= right {
        if (nums[i] ^ nums[i + 1]) != 0 {
            return false;
        }

        i += 2;
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut nums = vec![0; n];

        for i in 0..n {
            nums[i] = scan.token::<i64>();
        }

        let mut sum = 0;

        for j in 0..n {
            sum ^= nums[j];
        }

        // If sum is 0, the game should be "Draw".
        if sum == 0 {
            writeln!(out, "Draw").unwrap();
            continue;
        }

        // The number of integers is 1, the game should be "First".
        if n == 1 {
            writeln!(out, "First").unwrap();
            continue;
        }

        // The number of integers is even, the game should be "First".
        if n % 2 == 0 {
            writeln!(out, "First").unwrap();
            continue;
        }

        let mut msb = 0;

        for j in (0..31).rev() {
            if ((sum >> j) & 1) != 0 {
                msb = j;
                break;
            }
        }

        // Preprocess that each number leaves most significant "1".
        // NOTE: 1 <= X <= 1'000'000'000 ~= 2^30
        for j in 0..n {
            nums[j] = (nums[j] >> msb) & 1;
        }

        // Case 1: The first player selects the beginning of the sequence.
        // NOTE: The first player should select "1" at first turn.
        if nums[0] == 1 && process_game(&nums, 1, n - 1) {
            writeln!(out, "First").unwrap();
            continue;
        }

        // Case 2: The first player selects the end of the sequence.
        // NOTE: The first player should select "1" at first turn.
        if nums[n - 1] == 1 && process_game(&nums, 0, n - 2) {
            writeln!(out, "First").unwrap();
            continue;
        }

        writeln!(out, "Second").unwrap();
    }
}
