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

    fn check_shared_element_horizontal(&mut self) {
        let len_front = self.queue_horizontal_front.len();
        let len_back = self.queue_horizontal_back.len();

        // The shared element is middle of the queue (front + back)

        // In case of push, if the difference between the front and back is 2,
        // all elements should be moved from back to front.
        if len_front + 2 == len_back {
            self.queue_horizontal_front.push_back(self.shared_element);
            self.shared_element = self.queue_horizontal_back.pop_front().unwrap();
            return;
        }

        // In case of pop, if the difference between the front and back is 1,
        // all elements should be moved from back to front.
        if len_front == len_back + 1 {
            self.queue_horizontal_back.push_front(self.shared_element);
            self.shared_element = self.queue_horizontal_front.pop_back().unwrap();
            return;
        }
    }

    fn check_shared_element_vertical(&mut self) {
        let len_front = self.queue_vertical_front.len();
        let len_back = self.queue_vertical_back.len();

        // The shared element is middle of the queue (front + back)

        // In case of push, if the difference between the front and back is 2,
        // all elements should be moved from back to front.
        if len_front + 2 == len_back {
            self.queue_vertical_front.push_back(self.shared_element);
            self.shared_element = self.queue_vertical_back.pop_front().unwrap();
            return;
        }

        // In case of pop, if the difference between the front and back is 1,
        // all elements should be moved from back to front.
        if len_front == len_back + 1 {
            self.queue_vertical_back.push_front(self.shared_element);
            self.shared_element = self.queue_vertical_front.pop_back().unwrap();
            return;
        }
    }

    fn hpush(&mut self, x: i64) {
        if self.size == 0 {
            self.shared_element = x;
            self.size = 1;
            return;
        }

        self.queue_horizontal_back.push_back(x);
        self.check_shared_element_horizontal();

        self.size += 1;
    }

    fn hpop(&mut self) -> i64 {
        if self.size == 0 {
            -1
        } else if self.queue_horizontal_front.is_empty() && self.queue_horizontal_back.is_empty() {
            let val = self.shared_element;

            if !self.queue_vertical_front.is_empty() {
                self.shared_element = self.queue_vertical_front.pop_back().unwrap();
                self.check_shared_element_vertical();
            } else if !self.queue_vertical_back.is_empty() {
                self.shared_element = self.queue_vertical_back.pop_front().unwrap();
                self.check_shared_element_vertical();
            } else {
                self.shared_element = 0;
            }

            self.size -= 1;

            val
        } else if self.queue_horizontal_front.is_empty() {
            let val = self.shared_element;

            self.shared_element = self.queue_horizontal_back.pop_front().unwrap();
            self.check_shared_element_horizontal();
            self.size -= 1;

            val
        } else {
            let val = self.queue_horizontal_front.pop_front().unwrap();

            self.check_shared_element_horizontal();
            self.size -= 1;

            val
        }
    }

    fn hfront(&self) -> i64 {
        if self.size == 0 {
            -1
        } else if self.queue_horizontal_front.is_empty() && self.queue_horizontal_back.is_empty() {
            self.shared_element
        } else if self.queue_horizontal_front.is_empty() {
            self.shared_element
        } else {
            *self.queue_horizontal_front.front().unwrap()
        }
    }

    fn hback(&self) -> i64 {
        if self.size == 0 {
            -1
        } else if self.queue_horizontal_front.is_empty() && self.queue_horizontal_back.is_empty() {
            self.shared_element
        } else if self.queue_horizontal_back.is_empty() {
            self.shared_element
        } else {
            *self.queue_horizontal_back.back().unwrap()
        }
    }

    fn hsize(&self) -> usize {
        self.queue_horizontal_front.len()
            + self.queue_horizontal_back.len()
            + if self.size == 0 { 0 } else { 1 }
    }

    fn vpush(&mut self, x: i64) {
        if self.size == 0 {
            self.shared_element = x;
            self.size = 1;
            return;
        }

        self.queue_vertical_back.push_back(x);
        self.check_shared_element_vertical();

        self.size += 1;
    }

    fn vpop(&mut self) -> i64 {
        if self.size == 0 {
            -1
        } else if self.queue_vertical_front.is_empty() && self.queue_vertical_back.is_empty() {
            let val = self.shared_element;

            if !self.queue_horizontal_front.is_empty() {
                self.shared_element = self.queue_horizontal_front.pop_back().unwrap();
                self.check_shared_element_horizontal();
            } else if !self.queue_horizontal_back.is_empty() {
                self.shared_element = self.queue_horizontal_back.pop_front().unwrap();
                self.check_shared_element_horizontal();
            } else {
                self.shared_element = 0;
            }

            self.size -= 1;

            val
        } else if self.queue_vertical_front.is_empty() {
            let val = self.shared_element;

            self.shared_element = self.queue_vertical_back.pop_front().unwrap();
            self.check_shared_element_vertical();
            self.size -= 1;

            val
        } else {
            let val = self.queue_vertical_front.pop_front().unwrap();

            self.check_shared_element_vertical();
            self.size -= 1;

            val
        }
    }

    fn vfront(&self) -> i64 {
        if self.size == 0 {
            -1
        } else if self.queue_vertical_front.is_empty() && self.queue_vertical_back.is_empty() {
            self.shared_element
        } else if self.queue_vertical_front.is_empty() {
            self.shared_element
        } else {
            *self.queue_vertical_front.front().unwrap()
        }
    }

    fn vback(&self) -> i64 {
        if self.size == 0 {
            -1
        } else if self.queue_vertical_front.is_empty() && self.queue_vertical_back.is_empty() {
            self.shared_element
        } else if self.queue_vertical_back.is_empty() {
            self.shared_element
        } else {
            *self.queue_vertical_back.back().unwrap()
        }
    }

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
