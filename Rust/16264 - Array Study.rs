use io::Write;
use std::{collections::LinkedList, io, str};

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

static SQUARE: usize = 400;
static SIZE: usize = 200_001 / SQUARE + 10;

#[derive(Default, Clone)]
struct Query {
    start: usize,
    end: usize,
    val: usize,
}

impl Query {
    fn new(start: usize, end: usize, val: usize) -> Self {
        Self { start, end, val }
    }
}

fn plus(
    arr: &Vec<i32>,
    pos: &mut Vec<LinkedList<usize>>,
    count: &mut Vec<usize>,
    bucket: &mut Vec<usize>,
    x: usize,
    dir: usize,
) {
    let deque = &mut pos[arr[x] as usize];
    let mut now;

    if !deque.is_empty() {
        now = deque.back().unwrap() - deque.front().unwrap();
        count[now] -= 1;
        bucket[now / SQUARE] -= 1;
    }

    if dir == 0 {
        deque.push_front(x);
    } else {
        deque.push_back(x);
    }

    now = deque.back().unwrap() - deque.front().unwrap();
    count[now] += 1;
    bucket[now / SQUARE] += 1;
}

fn minus(
    arr: &Vec<i32>,
    pos: &mut Vec<LinkedList<usize>>,
    count: &mut Vec<usize>,
    bucket: &mut Vec<usize>,
    x: usize,
    dir: usize,
) {
    let deque = &mut pos[arr[x] as usize];
    let mut now = deque.back().unwrap() - deque.front().unwrap();

    count[now] -= 1;
    bucket[now / SQUARE] -= 1;

    if dir == 0 {
        deque.pop_front();
    } else {
        deque.pop_back();
    }

    if !deque.is_empty() {
        now = deque.back().unwrap() - deque.front().unwrap();
        count[now] += 1;
        bucket[now / SQUARE] += 1;
    }
}

fn process_query(count: &Vec<usize>, bucket: &Vec<usize>) -> usize {
    for i in (0..=(SIZE - 1)).rev() {
        if bucket[i] == 0 {
            continue;
        }

        for j in (0..=(SQUARE - 1)).rev() {
            if count[i * SQUARE + j] > 0 {
                return i * SQUARE + j;
            }
        }
    }

    0
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<usize>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut arr = vec![0; 200_001];

        for i in 1..=n {
            arr[i] = scan.token::<i32>();
        }

        for i in 1..=n {
            arr[i] += arr[i - 1];
        }

        for i in 0..=n {
            arr[i] += 100_000;
        }

        let m = scan.token::<usize>();
        let mut query = vec![Query::default(); m + 1];

        for i in 0..m {
            let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
            query[i] = Query::new(a - 1, b, i);
        }

        query.sort_by(|a, b| {
            if a.start / SQUARE != b.start / SQUARE {
                a.start.cmp(&b.start).reverse()
            } else {
                a.end.cmp(&b.end).reverse()
            }
        });

        let (mut start, mut end, mut val) = (query[0].start, query[0].end, query[0].val);
        let mut pos = vec![LinkedList::new(); 200_001];
        let mut count = vec![0; 200_001];
        let mut bucket = vec![0; SIZE];

        for i in start..=end {
            plus(&arr, &mut pos, &mut count, &mut bucket, i, 1);
        }

        let mut ans = vec![0; 200_001];

        ans[val] = process_query(&count, &bucket);

        for i in 1..m {
            val = query[i].val;

            while query[i].start < start {
                start -= 1;
                plus(&arr, &mut pos, &mut count, &mut bucket, start, 0);
            }

            while query[i].end > end {
                end += 1;
                plus(&arr, &mut pos, &mut count, &mut bucket, end, 1);
            }

            while query[i].start > start {
                minus(&arr, &mut pos, &mut count, &mut bucket, start, 0);
                start += 1;
            }

            while query[i].end < end {
                minus(&arr, &mut pos, &mut count, &mut bucket, end, 1);
                end -= 1;
            }

            ans[val] = process_query(&count, &bucket);
        }

        writeln!(out, "{}", ans.iter().sum::<usize>()).unwrap();
    }
}
