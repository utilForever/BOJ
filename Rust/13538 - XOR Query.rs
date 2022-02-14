use io::Write;
use std::{io, str};
use std::ops::{Deref, DerefMut};

struct StaticCell<T: 'static + Sized> {
    inner: &'static mut T,
}

impl<T> StaticCell<T> {
    pub fn new(t: T) -> Self {
        Self {
            inner: Box::leak(Box::new(t)),
        }
    }
}

impl<T> Clone for StaticCell<T> {
    fn clone(&self) -> Self {
        unsafe {
            Self {
                inner: &mut *((self.inner as *const T) as *mut T),
            }
        }
    }
}

impl<T> Deref for StaticCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<T> DerefMut for StaticCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

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
    left: Option<StaticCell<Node>>,
    right: Option<StaticCell<Node>>,
}

impl Node {
    fn new(value: i32, left: Option<StaticCell<Node>>, right: Option<StaticCell<Node>>) -> Self {
        Self { value, left, right }
    }

    fn update(&self, x: i32, y: i32, pos: i32) -> StaticCell<Node> {
        if x <= pos && y >= pos {
            if x == y {
                return StaticCell::new(Node::new(self.value + 1, None, None));
            }

            let mid = (x + y) / 2;
            let left = self.left.as_ref().unwrap().update(x, mid, pos);
            let right = self
                .right
                .as_ref()
                .unwrap()
                .update(mid + 1, y, pos);

            return StaticCell::new(Node::new(
                self.value + 1,
                Some(left),
                Some(right),
            ));
        }

        StaticCell::new(self.clone())
    }
}

fn query_xor(left: &StaticCell<Node>, right: &StaticCell<Node>, x: i32, y: i32, k: i32) -> i32 {
    if x == y {
        return x;
    }

    let mid = (x + y) / 2;

    if (((y - x + 1) / 2) & k) != 0
        && left.left.as_ref().unwrap().value
            == right.left.as_ref().unwrap().value
    {
        return query_xor(
            left.right.as_ref().unwrap(),
            right.right.as_ref().unwrap(),
            mid + 1,
            y,
            k,
        );
    }

    if (((y - x + 1) / 2) & k) == 0
        && left.right.as_ref().unwrap().value
            != right.right.as_ref().unwrap().value
    {
        return query_xor(
            left.right.as_ref().unwrap(),
            right.right.as_ref().unwrap(),
            mid + 1,
            y,
            k,
        );
    }

    query_xor(
        left.left.as_ref().unwrap(),
        right.left.as_ref().unwrap(),
        x,
        mid,
        k,
    )
}

fn query_sum(p: &StaticCell<Node>, x: i32, y: i32, min: i32, max: i32) -> i32 {
    if y < min || x > max {
        return 0;
    }

    if x >= min && y <= max {
        return p.value;
    }

    let mid = (x + y) / 2;

    return query_sum(p.left.as_ref().unwrap(), x, mid, min, max)
        + query_sum(p.right.as_ref().unwrap(), mid + 1, y, min, max);
}

fn query_kth(left: &StaticCell<Node>, right: &StaticCell<Node>, x: i32, y: i32, k: i32) -> i32 {
    if x == y {
        return x;
    }

    let mid = (x + y) / 2;

    if right.left.as_ref().unwrap().value
        - left.left.as_ref().unwrap().value
        >= k
    {
        return query_kth(
            left.left.as_ref().unwrap(),
            right.left.as_ref().unwrap(),
            x,
            mid,
            k,
        );
    }

    query_kth(
        left.right.as_ref().unwrap(),
        right.right.as_ref().unwrap(),
        mid + 1,
        y,
        k - (right.left.as_ref().unwrap().value
            - left.left.as_ref().unwrap().value),
    )
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let max_x = 524_288;
    let mut segments = vec![None; 500_001];
    let mut num_elements = 0;

    segments[0] = Some(StaticCell::new(Node::new(0, None, None)));
    segments[0].as_mut().unwrap().left = segments[0].clone();
    segments[0].as_mut().unwrap().right = segments[0].clone();

    let m = scan.token::<usize>();

    for _ in 0..m {
        let cmd = scan.token::<usize>();

        if cmd == 1 {
            let x = scan.token::<i32>();

            num_elements += 1;
            let ret = segments[num_elements - 1]
                .as_mut()
                .unwrap()
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
