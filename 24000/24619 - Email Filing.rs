use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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

fn check(folders: &Vec<usize>, m: usize, n: usize, k: usize) -> bool {
    let mut remain = vec![0; m];

    for &folder in folders.iter() {
        remain[folder] += 1;
    }

    let mut email_first = 0;
    let mut email_last = k - 1;
    let mut folder_first = 0;
    let mut processed = vec![false; n];
    let mut priority_queue = BinaryHeap::new();

    for i in 0..k {
        priority_queue.push(Reverse((folders[i], i)));
    }

    while email_last < n - 1 && folder_first < m {
        if remain[folder_first] == 0 {
            folder_first += 1;
            continue;
        }

        let can_file = match priority_queue.peek() {
            Some(Reverse((folder, _))) => *folder < folder_first + k,
            None => false,
        };

        if can_file {
            let Reverse((folder, idx)) = priority_queue.pop().unwrap();

            if idx >= email_first {
                remain[folder] -= 1;
                processed[idx] = true;

                email_last += 1;
                priority_queue.push(Reverse((folders[email_last], email_last)));
            }
        } else {
            while email_first < n && processed[email_first] {
                email_first += 1;
            }

            if email_first < n {
                email_first += 1;
            }

            email_last += 1;
            priority_queue.push(Reverse((folders[email_last], email_last)));
        }
    }

    while folder_first < m {
        if remain[folder_first] == 0 {
            folder_first += 1;
            continue;
        }

        let Some(Reverse((folder, idx))) = priority_queue.pop() else {
            return false;
        };

        if idx >= email_first && !processed[idx] {
            if folder >= folder_first + k {
                return false;
            }

            processed[idx] = true;
            remain[folder] -= 1;

            while email_first > 0 {
                email_first -= 1;

                if !processed[email_first] {
                    priority_queue.push(Reverse((folders[email_first], email_first)));
                    break;
                }
            }
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (m, n, k) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let mut folders = vec![0; n];

        for i in 0..n {
            folders[i] = scan.token::<usize>() - 1;
        }

        writeln!(
            out,
            "{}",
            if check(&folders, m, n, k) {
                "YES"
            } else {
                "NO"
            }
        )
        .unwrap();
    }
}
