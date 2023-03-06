use io::Write;
use std::{cmp, io, str};

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

fn compare<T: PartialOrd>(a: &T, b: &T) -> i32 {
    match a.partial_cmp(b).unwrap() {
        cmp::Ordering::Greater => 1,
        cmp::Ordering::Less => -1,
        _ => 0,
    }
}

pub type NodeCell<K, V> = Option<Box<Node<K, V>>>;

pub struct Node<K, V> {
    left: NodeCell<K, V>,
    right: NodeCell<K, V>,
    key: K,
    val: V,
}

impl<K: PartialOrd, V> Node<K, V> {
    fn new(key: K, val: V) -> Node<K, V> {
        Node {
            left: None,
            right: None,
            key: key,
            val: val,
        }
    }

    fn height(x: Option<&Box<Node<K, V>>>) -> usize {
        if let Some(node) = x {
            let lh = Node::height(node.left.as_ref());
            let rh = Node::height(node.left.as_ref());
            if lh <= rh {
                rh + 1
            } else {
                lh + 1
            }
        } else {
            0
        }
    }

    fn size(x: Option<&Box<Node<K, V>>>) -> usize {
        if let Some(node) = x {
            1 + Node::size(node.left.as_ref()) + Node::size(node.right.as_ref())
        } else {
            0
        }
    }

    fn splay(mut h: NodeCell<K, V>, key: &K) -> NodeCell<K, V> {
        if h.is_none() {
            return None;
        }
        let cmp1 = h.as_ref().map(|n| compare(key, &n.key)).unwrap();

        if cmp1 < 0 {
            // key not in tree, done
            if h.as_ref().unwrap().left.is_none() {
                return h;
            }
            let cmp2 = compare(key, &h.as_ref().unwrap().left.as_ref().unwrap().key);
            if cmp2 < 0 {
                h.as_mut().map(|n| {
                    n.left.as_mut().map(|n| {
                        n.left = Node::splay(n.left.take(), key);
                    })
                });
                h = Node::rotate_right(h);
            } else if cmp2 > 0 {
                if let Some(ref mut n) = h.as_mut() {
                    if n.left.as_mut().map_or(false, |n| {
                        n.right = Node::splay(n.right.take(), key);
                        n.right.is_some()
                    }) {
                        n.left = Node::rotate_left(n.left.take());
                    }
                }
            }
            // key not in tree
            if h.as_ref().unwrap().left.is_none() {
                return h;
            } else {
                return Node::rotate_right(h);
            }
        } else if cmp1 > 0 {
            // key not in tree, done
            if h.as_ref().unwrap().right.is_none() {
                return h;
            }
            let cmp2 = compare(key, &h.as_ref().unwrap().right.as_ref().unwrap().key);
            if cmp2 < 0 {
                h.as_mut().map(|n| {
                    if n.right.as_mut().map_or(false, |n| {
                        n.left = Node::splay(n.left.take(), key);
                        n.left.is_some()
                    }) {
                        n.right = Node::rotate_right(n.right.take());
                    }
                });
            } else if cmp2 > 0 {
                h.as_mut().map(|n| {
                    n.right.as_mut().map(|n| {
                        n.right = Node::splay(n.right.take(), key);
                    })
                });
                h = Node::rotate_left(h);
            }
            // key not in tree
            if h.as_ref().unwrap().right.is_none() {
                return h;
            } else {
                return Node::rotate_left(h);
            }
        }
        h
    }

    fn rotate_right(mut h: NodeCell<K, V>) -> NodeCell<K, V> {
        let mut x = h.as_mut().map_or(None, |n| n.left.take());
        h.as_mut()
            .map(|n| n.left = x.as_mut().map_or(None, |n| n.right.take()));
        x.as_mut().map(|n| n.right = h);
        x
    }

    fn rotate_left(mut h: NodeCell<K, V>) -> NodeCell<K, V> {
        let mut x = h.as_mut().map_or(None, |n| n.right.take());
        h.as_mut()
            .map(|n| n.right = x.as_mut().map_or(None, |n| n.left.take()));
        x.as_mut().map(|n| n.left = h);
        x
    }
}

/// Splay tree. Supports splay-insert, -search, and -delete.
pub struct SplayTree<K, V> {
    root: NodeCell<K, V>,
    // size: usize
}

impl<K: PartialOrd, V> SplayTree<K, V> {
    pub fn new() -> SplayTree<K, V> {
        SplayTree {
            root: None,
            // size: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn size(&self) -> usize {
        Node::size(self.root.as_ref())
    }

    pub fn height(&self) -> usize {
        Node::height(self.root.as_ref())
    }

    pub fn clear(&mut self) {
        self.root = None;
    }

    // get() needs to update tree structure
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.root = Node::splay(self.root.take(), key);
        self.root
            .as_ref()
            .map_or(None, |n| if n.key == *key { Some(&n.val) } else { None })
    }

    pub fn contains_key(&mut self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn get_mut<'t>(&'t mut self, key: &K) -> Option<&'t mut V> {
        self.root = Node::splay(self.root.take(), key);
        self.root.as_mut().map_or(None, |n| {
            if n.key == *key {
                Some(&mut n.val)
            } else {
                None
            }
        })
    }

    /// Splay tree insertion.
    pub fn insert(&mut self, key: K, val: V) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(key, val)));
            return;
        }

        let mut root = Node::splay(self.root.take(), &key);

        let cmp = compare(&key, &root.as_ref().unwrap().key);
        if cmp < 0 {
            let mut n = Node::new(key, val);
            n.left = root.as_mut().unwrap().left.take();
            n.right = root;
            self.root = Some(Box::new(n));
        } else if cmp > 0 {
            let mut n = Node::new(key, val);
            n.right = root.as_mut().unwrap().right.take();
            n.left = root;
            self.root = Some(Box::new(n));
        } else if cmp == 0 {
            root.as_mut().map(|n| n.val = val);
            self.root = root;
        } else {
            unreachable!();
        }
    }

    /// Splay tree deletion.
    // use Algs4 approach
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.root.is_none() {
            return None;
        }

        let mut root = Node::splay(self.root.take(), key);

        if *key == root.as_ref().unwrap().key {
            if root.as_ref().unwrap().left.is_none() {
                self.root = root.as_mut().unwrap().right.take();
            } else {
                let x = root.as_mut().unwrap().right.take();
                self.root = Node::splay(root.as_mut().unwrap().left.take(), key);
                self.root.as_mut().map(|n| n.right = x);
            }
            root.map(|n| n.val)
        } else {
            None
        }
    }
}

// Reference: https://snippets.kiwiyou.dev/splay-tree
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let mut splay_tree = SplayTree::new();

    for _ in 0..n {
        let val = scan.token::<i64>();
        splay_tree.insert(val, val);
    }

    let m = scan.token::<i64>();

    for _ in 0..m {
        let command = scan.token::<i64>();

        if command == 1 {
        } else {
        }
    }
}
