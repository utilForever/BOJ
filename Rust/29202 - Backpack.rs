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
}

#[derive(Default, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Book {
    weight: i64,
    volume: i64,
    thickness: i64,
    idx: usize,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut books = vec![Book::default(); n];

    for i in 0..n {
        books[i].weight = scan.token::<i64>();
        books[i].volume = scan.token::<i64>();
        books[i].thickness = scan.token::<i64>();
        books[i].idx = i + 1;
    }

    let mut priority_queue = BinaryHeap::new();
    let mut priority_queue_thickness = BinaryHeap::new();
    let mut priority_queue_erase = BinaryHeap::new();
    let mut sum_weight = 0;
    let mut min_weight = i64::MAX;
    let mut min_weight_idx = 0;
    let mut min_weight_thickness = i64::MAX;
    let mut min_weight_thickness_idx = 0;
    let mut idx = 0;
    let mut idx_last = 0;
    let mut ret = i64::MAX;

    if k == 1 {
        let mut ret = i64::MAX;
        let mut idx_last = 0;

        for i in 0..n {
            let val = books[i].weight + books[i].volume + books[i].thickness;

            if ret > val {
                ret = val;
                idx_last = i + 1;
            }
        }

        writeln!(out, "{ret}").unwrap();
        writeln!(out, "{idx_last}").unwrap();
        return;
    }

    books.sort_by(|a, b| a.volume.cmp(&b.volume));

    for i in 0..n {
        priority_queue.push(books[i].clone());
        priority_queue_thickness.push(-books[i].thickness);
        sum_weight += books[i].weight;

        if priority_queue.len() == k {
            let book = priority_queue.pop().unwrap();
            sum_weight -= book.weight;

            if min_weight > book.weight {
                min_weight = book.weight;
                min_weight_idx = book.idx;
            }

            if min_weight_thickness > book.weight + book.thickness {
                min_weight_thickness = book.weight + book.thickness;
                min_weight_thickness_idx = book.idx;
            }

            priority_queue_erase.push(-book.thickness);

            while !priority_queue_erase.is_empty()
                && priority_queue_erase.peek().unwrap() == priority_queue_thickness.peek().unwrap()
            {
                priority_queue_thickness.pop();
                priority_queue_erase.pop();
            }

            let ret1 = sum_weight
                + books[i].volume
                + min_weight
                + (-priority_queue_thickness.peek().unwrap());

            if ret > ret1 {
                ret = ret1;
                idx = i;
                idx_last = min_weight_idx;
            }

            let ret2 = sum_weight + books[i].volume + min_weight_thickness;

            if ret > ret2 {
                ret = ret2;
                idx = i;
                idx_last = min_weight_thickness_idx;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();

    let mut ret_books = books[0..idx + 1].to_vec();
    ret_books.sort();

    for i in 0..k - 1 {
        write!(out, "{} ", ret_books[i].idx).unwrap();
    }

    writeln!(out, "{idx_last}").unwrap();
}
