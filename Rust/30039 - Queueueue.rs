use io::Write;
use std::{collections::VecDeque, io, str};

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

struct Queueueue {
    shared_element: i64,
    size: usize,
    queue_horizontal_front: VecDeque<i64>,
    queue_horizontal_back: VecDeque<i64>,
    queue_vertical_front: VecDeque<i64>,
    queue_vertical_back: VecDeque<i64>,
}

impl Queueueue {
    fn new() -> Self {
        Self {
            shared_element: 0,
            size: 0,
            queue_horizontal_front: VecDeque::new(),
            queue_horizontal_back: VecDeque::new(),
            queue_vertical_front: VecDeque::new(),
            queue_vertical_back: VecDeque::new(),
        }
    }

    fn hpush(&mut self, x: i64) {}

    fn hpop(&mut self) -> i64 {}

    fn hfront(&self) -> i64 {}

    fn hback(&self) -> i64 {}

    fn hsize(&self) -> usize {
        self.queue_horizontal_front.len()
            + self.queue_horizontal_back.len()
            + if self.size == 0 { 0 } else { 1 }
    }

    fn vpush(&mut self, x: i64) {}

    fn vpop(&mut self) -> i64 {}

    fn vfront(&self) -> i64 {}

    fn vback(&self) -> i64 {}

    fn vsize(&self) -> usize {
        self.queue_vertical_front.len()
            + self.queue_vertical_back.len()
            + if self.size == 0 { 0 } else { 1 }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn empty(&self) -> i64 {
        if self.size == 0 {
            1
        } else {
            0
        }
    }

    fn middle(&self) -> i64 {
        if self.size == 0 {
            -1
        } else {
            self.shared_element
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let mut queueueue = Queueueue::new();

    for _ in 0..n {
        let command = scan.token::<String>();

        match command.as_str() {
            "hpush" => {
                let x = scan.token::<i64>();
                queueueue.hpush(x);
            }
            "hpop" => {
                writeln!(out, "{}", queueueue.hpop()).unwrap();
            }
            "hfront" => {
                writeln!(out, "{}", queueueue.hfront()).unwrap();
            }
            "hback" => {
                writeln!(out, "{}", queueueue.hback()).unwrap();
            }
            "hsize" => {
                writeln!(out, "{}", queueueue.hsize()).unwrap();
            }
            "vpush" => {
                let x = scan.token::<i64>();
                queueueue.vpush(x);
            }
            "vpop" => {
                writeln!(out, "{}", queueueue.vpop()).unwrap();
            }
            "vfront" => {
                writeln!(out, "{}", queueueue.vfront()).unwrap();
            }
            "vback" => {
                writeln!(out, "{}", queueueue.vback()).unwrap();
            }
            "vsize" => {
                writeln!(out, "{}", queueueue.vsize()).unwrap();
            }
            "size" => {
                writeln!(out, "{}", queueueue.size()).unwrap();
            }
            "empty" => {
                writeln!(out, "{}", queueueue.empty()).unwrap();
            }
            "middle" => {
                writeln!(out, "{}", queueueue.middle()).unwrap();
            }
            _ => unreachable!(),
        }
    }
}
