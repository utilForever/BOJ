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
pub struct Node<T> {
    element: T,
    rank: usize,
    left: *mut Node<T>,
    right: *mut Node<T>,
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.left.is_null() {
                drop(Box::from_raw(self.left));
            }

            if !self.right.is_null() {
                drop(Box::from_raw(self.right));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LeftistHeap<T> {
    root: *mut Node<T>,
}

impl<T> Drop for LeftistHeap<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.root.is_null() {
                drop(Box::from_raw(self.root));
            }

            self.root = std::ptr::null_mut();
        }
    }
}

impl<T: Clone + Ord> LeftistHeap<T> {
    pub fn new() -> Self {
        Self {
            root: std::ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_null()
    }

    pub fn insert(&mut self, x: T) {
        let new_node = Box::new(Node {
            element: x,
            rank: 0,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
        });

        self.root = LeftistHeap::merge_nodes(Box::into_raw(new_node), self.root);
    }

    pub fn find_min(&self) -> &T {
        unsafe { &(*self.root).element }
    }

    pub fn delete_min(&mut self) {
        if self.is_empty() {
            return;
        }

        unsafe {
            let mut old_root = Box::from_raw(self.root);
            self.root = LeftistHeap::merge_nodes((*self.root).left, (*self.root).right);

            old_root.left = std::ptr::null_mut();
            old_root.right = std::ptr::null_mut();
            drop(old_root);
        }
    }

    pub fn merge(&mut self, other: *mut LeftistHeap<T>) {
        unsafe {
            if self.root == (*other).root {
                return;
            }

            self.root = LeftistHeap::merge_nodes(self.root, (*other).root);
            (*other).root = std::ptr::null_mut();
        }
    }

    fn merge_nodes(a: *mut Node<T>, b: *mut Node<T>) -> *mut Node<T> {
        if a.is_null() {
            return b;
        }

        if b.is_null() {
            return a;
        }

        unsafe {
            if (*a).element < (*b).element {
                LeftistHeap::merge_internal(a, b)
            } else {
                LeftistHeap::merge_internal(b, a)
            }
        }
    }

    fn merge_internal(a: *mut Node<T>, b: *mut Node<T>) -> *mut Node<T> {
        unsafe {
            if (*a).left.is_null() {
                (*a).left = b;
            } else {
                (*a).right = LeftistHeap::merge_nodes((*a).right, b);

                if (*(*a).left).rank < (*(*a).right).rank {
                    let tmp = (*a).left;
                    (*a).left = (*a).right;
                    (*a).right = tmp;
                }

                (*a).rank = (*(*a).right).rank + 1;
            }

            a
        }
    }
}

fn process_hu_tucker(papers: &mut Vec<i64>) -> i64 {
    let len = papers.len();
    let mut heaps = vec![LeftistHeap::<i64>::new(); len - 1];
    let mut priority_queue = BinaryHeap::new();
    let mut left = vec![0; len];
    let mut right = vec![0; len];
    let mut costs = vec![0; len - 1];
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
            let min = *heaps[index].find_min();
            heaps[index].delete_min();

            if papers[index] + min == cost {
                merge_left = true;
            } else if papers[right[index] as usize] + min == cost {
                merge_right = true;
            } else {
                heaps[index].delete_min();
            }
        }

        ret += cost;
        heaps[index].insert(cost);

        if merge_left {
            papers[index] = i64::MAX / 2;
        }

        if merge_right {
            papers[right[index] as usize] = i64::MAX / 2;
        }

        if merge_left && index > 0 {
            let idx_left = left[index] as usize;
            let rhs = std::ptr::addr_of_mut!(heaps[index]);

            heaps[idx_left].merge(rhs);
            right[idx_left] = right[index];
            right[index] = -1;
            left[right[idx_left] as usize] = idx_left as i64;
            index = idx_left;
        }

        if merge_right && right[index] < len as i64 - 1 {
            let idx_right = right[index] as usize;
            let rhs = std::ptr::addr_of_mut!(heaps[idx_right]);

            heaps[index].merge(rhs);
            right[index] = right[idx_right];
            right[idx_right] = -1;
            left[right[index] as usize] = index as i64;
        }

        costs[index] = papers[index] + papers[right[index] as usize];

        if !heaps[index].is_empty() {
            let min = *heaps[index].find_min();
            heaps[index].delete_min();

            costs[index] = costs[index].min(papers[index].min(papers[right[index] as usize]) + min);

            if !heaps[index].is_empty() {
                costs[index] = costs[index].min(min + heaps[index].find_min());
            }

            heaps[index].insert(min);
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
