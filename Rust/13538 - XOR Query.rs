use io::Write;
use std::{cell::RefCell, io, rc::Rc, str};

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

#[derive(Clone)]
struct Node {
    value: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32, left: Option<Rc<RefCell<Node>>>, right: Option<Rc<RefCell<Node>>>) -> Self {
        Self { value, left, right }
    }

    fn update(&self, x: i32, y: i32, pos: i32) -> Rc<RefCell<Node>> {
        if x <= pos && y >= pos {
            if x == y {
                return Rc::new(RefCell::new(Node::new(self.value + 1, None, None)));
            }

            let mid = (x + y) / 2;
            let left = self.left.as_ref().unwrap().borrow().update(x, mid, pos);
            let right = self
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .update(mid + 1, y, pos);

            return Rc::new(RefCell::new(Node::new(
                self.value + 1,
                Some(left),
                Some(right),
            )));
        }

        Rc::new(RefCell::new(self.clone()))
    }
}

fn query_xor(left: &Rc<RefCell<Node>>, right: &Rc<RefCell<Node>>, x: i32, y: i32, k: i32) -> i32 {
    if x == y {
        return x;
    }

    let mid = (x + y) / 2;

    if (((y - x + 1) / 2) & k) != 0
        && left.borrow().left.as_ref().unwrap().borrow().value
            == right.borrow().left.as_ref().unwrap().borrow().value
    {
        return query_xor(
            left.borrow().right.as_ref().unwrap(),
            right.borrow().right.as_ref().unwrap(),
            mid + 1,
            y,
            k,
        );
    }

    if (((y - x + 1) / 2) & k) == 0
        && left.borrow().right.as_ref().unwrap().borrow().value
            != right.borrow().right.as_ref().unwrap().borrow().value
    {
        return query_xor(
            left.borrow().right.as_ref().unwrap(),
            right.borrow().right.as_ref().unwrap(),
            mid + 1,
            y,
            k,
        );
    }

    query_xor(
        left.borrow().left.as_ref().unwrap(),
        right.borrow().left.as_ref().unwrap(),
        x,
        mid,
        k,
    )
}

fn query_sum(p: &Rc<RefCell<Node>>, x: i32, y: i32, min: i32, max: i32) -> i32 {
    if y < min || x > max {
        return 0;
    }

    if x >= min && y <= max {
        return p.borrow().value;
    }

    let mid = (x + y) / 2;

    return query_sum(p.borrow().left.as_ref().unwrap(), x, mid, min, max)
        + query_sum(p.borrow().right.as_ref().unwrap(), mid + 1, y, min, max);
}

fn query_kth(left: &Rc<RefCell<Node>>, right: &Rc<RefCell<Node>>, x: i32, y: i32, k: i32) -> i32 {
    if x == y {
        return x;
    }

    let mid = (x + y) / 2;

    if right.borrow().left.as_ref().unwrap().borrow().value
        - left.borrow().left.as_ref().unwrap().borrow().value
        >= k
    {
        return query_kth(
            left.borrow().left.as_ref().unwrap(),
            right.borrow().left.as_ref().unwrap(),
            x,
            mid,
            k,
        );
    }

    query_kth(
        left.borrow().right.as_ref().unwrap(),
        right.borrow().right.as_ref().unwrap(),
        mid + 1,
        y,
        k - (right.borrow().left.as_ref().unwrap().borrow().value
            - left.borrow().left.as_ref().unwrap().borrow().value),
    )
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let max_x = 524_288;
    let mut segments = vec![None; 500_001];
    let mut num_elements = 0;

    segments[0] = Some(Rc::new(RefCell::new(Node::new(0, None, None))));
    segments[0].as_mut().unwrap().borrow_mut().left = segments[0].clone();
    segments[0].as_mut().unwrap().borrow_mut().right = segments[0].clone();

    let m = scan.token::<usize>();

    for _ in 0..m {
        let cmd = scan.token::<usize>();

        if cmd == 1 {
            let x = scan.token::<i32>();

            num_elements += 1;
            let ret = segments[num_elements - 1]
                .as_mut()
                .unwrap()
                .borrow()
                .update(0, max_x - 1, x);
            segments[num_elements] = Some(ret);
        } else if cmd == 2 {
            let (x, y, z) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i32>(),
            );

            writeln!(
                out,
                "{}",
                query_xor(
                    &segments[x - 1].clone().unwrap(),
                    &segments[y].clone().unwrap(),
                    0,
                    max_x - 1,
                    z
                )
            )
            .unwrap();
        } else if cmd == 3 {
            let x = scan.token::<usize>();

            num_elements -= x;
        } else if cmd == 4 {
            let (x, y, z) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i32>(),
            );

            writeln!(
                out,
                "{}",
                query_sum(&segments[y].clone().unwrap(), 0, max_x - 1, 1, z)
                    - query_sum(&segments[x - 1].clone().unwrap(), 0, max_x - 1, 1, z)
            )
            .unwrap();
        } else {
            let (x, y, z) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i32>(),
            );

            writeln!(
                out,
                "{}",
                query_kth(
                    &segments[x - 1].clone().unwrap(),
                    &segments[y].clone().unwrap(),
                    0,
                    max_x - 1,
                    z
                )
            )
            .unwrap();
        }
    }
}
