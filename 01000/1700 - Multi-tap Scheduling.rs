use io::Write;
use std::{collections::BinaryHeap, io, str};

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut goods = vec![0; k];

    for i in 0..k {
        goods[i] = scan.token::<usize>();
    }

    let mut last = vec![usize::MAX; k + 1];
    let mut pos_next = vec![usize::MAX; k];

    for i in (0..k).rev() {
        pos_next[i] = last[goods[i]];
        last[goods[i]] = i;
    }

    let mut priority_queue = BinaryHeap::new();
    let mut used = vec![false; k + 1];
    let mut pos_use_next = vec![usize::MAX; k + 1];
    let mut cnt_plugged = 0;
    let mut ret = 0;

    for i in 0..k {
        if used[goods[i]] {
            pos_use_next[goods[i]] = pos_next[i];
            priority_queue.push((pos_next[i], goods[i]));
            continue;
        }

        if cnt_plugged < n {
            priority_queue.push((pos_next[i], goods[i]));
            used[goods[i]] = true;
            pos_use_next[goods[i]] = pos_next[i];
            cnt_plugged += 1;
            continue;
        }

        loop {
            let (pos, good) = priority_queue.pop().unwrap();

            if used[good] && pos_use_next[good] == pos {
                used[good] = false;
                cnt_plugged -= 1;
                break;
            }
        }

        priority_queue.push((pos_next[i], goods[i]));
        used[goods[i]] = true;
        pos_use_next[goods[i]] = pos_next[i];
        cnt_plugged += 1;
        ret += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
