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
}

#[derive(Debug, Clone)]
pub enum Heap<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    element: T,
    rank: usize,
    left: Heap<T>,
    right: Heap<T>,
}

impl<T: Clone + Ord> Heap<T> {
    pub fn is_empty(&self) -> bool {
        match self {
            Heap::Empty => true,
            _ => false,
        }
    }

    pub fn insert(self, x: T) -> Heap<T> {
        let new_node = Box::new(Node {
            rank: 1,
            element: x,
            left: Heap::Empty,
            right: Heap::Empty,
        });

        Heap::merge_internal(Heap::NonEmpty(new_node), self)
    }

    pub fn find_min(&self) -> Option<&T> {
        match self {
            Heap::NonEmpty(ref node) => Some(&node.element),
            Heap::Empty => None,
        }
    }

    pub fn delete_min(self) -> Heap<T> {
        match self {
            Heap::NonEmpty(node) => Heap::merge_internal(node.left, node.right),
            Heap::Empty => Heap::Empty,
        }
    }

    pub fn merge(&mut self, other: &mut Heap<T>) -> Heap<T> {
        Heap::merge_internal(self.clone(), other.clone())
    }

    fn merge_internal(a: Heap<T>, b: Heap<T>) -> Heap<T> {
        match (a, b) {
            (h, Heap::Empty) => h,
            (Heap::Empty, h) => h,
            (Heap::NonEmpty(h1), Heap::NonEmpty(h2)) => {
                if h1.element <= h2.element {
                    Heap::make(
                        h1.element,
                        h1.left,
                        Heap::merge_internal(h1.right, Heap::NonEmpty(h2)),
                    )
                } else {
                    Heap::make(
                        h2.element,
                        h2.left,
                        Heap::merge_internal(Heap::NonEmpty(h1), h2.right),
                    )
                }
            }
        }
    }
    
    fn make(element: T, left: Heap<T>, right: Heap<T>) -> Heap<T> {
        if left.rank() >= right.rank() {
            Heap::NonEmpty(Box::new(Node {
                element,
                rank: right.rank() + 1,
                left,
                right,
            }))
        } else {
            Heap::NonEmpty(Box::new(Node {
                element,
                rank: left.rank() + 1,
                left: right,
                right: left,
            }))
        }
    }

    fn rank(&self) -> usize {
        match self {
            Heap::Empty => 0,
            Heap::NonEmpty(ref node) => node.rank,
        }
    }
}

fn process_hu_tucker(papers: &mut Vec<i64>) -> i64 {
    let len = papers.len();
    let mut heaps = vec![Heap::Empty; len];
    let mut priority_queue = BinaryHeap::new();
    let mut left = vec![0; len];
    let mut right = vec![0; len];
    let mut costs = vec![0; len];
    let mut ret = 0;

    for i in 0..len - 1 {
        left[i] = i as i64 - 1;
        right[i] = i as i64 + 1;
        costs[i] = papers[i] + papers[i + 1];
        priority_queue.push(Reverse((costs[i], i)));
    }

    for _ in 0..len - 1 {
        let mut cost;
        let mut index;

        loop {
            (cost, index) = priority_queue.pop().unwrap().0;

            if right[index] != -1 && costs[index] == cost {
                break;
            }
        }

        let mut merge_left = false;
        let mut merge_right = false;

        if papers[index] + papers[right[index] as usize] == cost {
            merge_left = true;
            merge_right = true;
        } else {
            let min = *heaps[index].find_min().unwrap();
            heaps[index] = heaps[index].clone().delete_min();

            if papers[index] + min == cost {
                merge_left = true;
            } else if papers[right[index] as usize] + min == cost {
                merge_right = true;
            } else {
                heaps[index] = heaps[index].clone().delete_min();
            }
        }

        ret += cost;
        heaps[index] = heaps[index].clone().insert(cost);

        if merge_left {
            papers[index] = i64::MAX / 2;
        }

        if merge_right {
            papers[right[index] as usize] = i64::MAX / 2;
        }

        if merge_left && index > 0 {
            let idx_left = left[index] as usize;

            heaps[idx_left] = heaps[idx_left].clone().merge(&mut heaps[index]);
            right[idx_left] = right[index];
            right[index] = -1;
            left[right[idx_left] as usize] = idx_left as i64;
            index = idx_left;
        }

        if merge_right && right[index] < len as i64 - 1 {
            let idx_right = right[index] as usize;

            heaps[index] = heaps[index].clone().merge(&mut heaps[idx_right]);
            right[index] = right[idx_right];
            right[idx_right] = -1;
            left[right[index] as usize] = index as i64;
        }

        costs[index] = papers[index] + papers[right[index] as usize];

        if !heaps[index].is_empty() {
            let min = *heaps[index].find_min().unwrap();
            heaps[index] = heaps[index].clone().delete_min();

            costs[index] = costs[index].min(papers[index].min(papers[right[index] as usize]) + min);

            if !heaps[index].is_empty() {
                costs[index] = costs[index].min(min + heaps[index].find_min().unwrap());
            }

            heaps[index] = heaps[index].clone().insert(min);
        }

        priority_queue.push(Reverse((costs[index], index)));
    }

    ret
}

// Reference: https://sotanishy.github.io/cp-library-cpp/dp/hu_tucker.cpp
// Reference: https://tistory.joonhyung.xyz/15
// Reference: https://blog.myungwoo.kr/141
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let k = scan.token::<usize>();
        let mut papers = vec![0; k];

        for i in 0..k {
            papers[i] = scan.token::<i64>();
        }

        writeln!(out, "{}", process_hu_tucker(&mut papers)).unwrap();
    }
}
