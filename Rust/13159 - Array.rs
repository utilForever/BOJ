use io::Write;
use std::{cmp::Ordering, io, marker::PhantomData, mem, ptr::NonNull, str};

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

#[derive(Eq, PartialEq)]
enum SplayType {
    Zig,
    ZigZig,
    ZigZag,
}

pub struct Node<K: Ord, V: Clone> {
    pub left: Option<NonNull<Node<K, V>>>,
    pub right: Option<NonNull<Node<K, V>>>,
    pub parent: Option<NonNull<Node<K, V>>>,
    key: K,
    value: V,
    cnt: usize,
    sum: V,
    min: V,
    max: V,
    is_flip: bool,
}

impl<K: Ord, V: Clone> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            left: None,
            right: None,
            parent: None,
            key,
            value: value.clone(),
            cnt: 1,
            sum: value.clone(),
            min: value.clone(),
            max: value,
            is_flip: false,
        }
    }

    fn splay_type(&self, g: Option<&Node<K, V>>) -> Option<SplayType> {
        if (self.parent().is_none() && g.is_none())
            || (self.parent.is_some()
                && g.is_some()
                && unsafe {
                    mem::transmute::<*mut Self, *const Self>(self.parent.unwrap().as_ptr())
                        == g.unwrap()
                })
        {
            Some(SplayType::Zig)
        } else if (self.is_left() && self.parent()?.is_left())
            || (self.is_right() && self.parent()?.is_right())
        {
            Some(SplayType::ZigZig)
        } else if !self.parent()?.is_root() {
            Some(SplayType::ZigZag)
        } else {
            None
        }
    }

    pub fn left(&self) -> Option<&Self> {
        self.left.as_ref().map(|ptr| unsafe { ptr.as_ref() })
    }

    pub fn left_mut(&mut self) -> Option<&mut Self> {
        self.left.as_mut().map(|ptr| unsafe { ptr.as_mut() })
    }

    pub fn right(&self) -> Option<&Self> {
        self.right.as_ref().map(|ptr| unsafe { ptr.as_ref() })
    }

    pub fn right_mut(&mut self) -> Option<&mut Self> {
        self.right.as_mut().map(|ptr| unsafe { ptr.as_mut() })
    }

    pub fn parent(&self) -> Option<&Self> {
        self.parent.as_ref().map(|ptr| unsafe { ptr.as_ref() })
    }

    pub fn parent_mut(&mut self) -> Option<&mut Self> {
        self.parent.as_mut().map(|ptr| unsafe { ptr.as_mut() })
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn is_left(&self) -> bool {
        if let Some(parent) = self.parent() {
            parent
                .left
                .map(|left| unsafe {
                    mem::transmute::<*mut Self, *const Self>(left.as_ptr()) == self
                })
                .unwrap_or(false)
        } else {
            false
        }
    }

    pub fn is_right(&self) -> bool {
        if let Some(parent) = self.parent() {
            parent
                .right
                .map(|right| unsafe {
                    mem::transmute::<*mut Self, *const Self>(right.as_ptr()) == self
                })
                .unwrap_or(false)
        } else {
            false
        }
    }

    pub unsafe fn ref_into_box(&mut self) -> Box<Self> {
        Box::from_raw(self)
    }

    fn rotate_left(&mut self) {
        let self_ptr = self.into();
        let parent_ptr = self.parent;
        let right_ptr = self.right;
        let right = match self.right_mut() {
            Some(ptr) => ptr,
            None => return,
        };
        let left_ptr = right.left;

        // if let Some(parent) = self.parent_mut() {
        //     parent.push();
        // }
        // self.push();

        right.parent = parent_ptr;
        right.left = Some(self_ptr);

        let is_left = self.is_left();

        if let Some(parent) = self.parent_mut() {
            if is_left {
                parent.left = right_ptr;
            } else {
                parent.right = right_ptr;
            }
        }

        self.parent = right_ptr;
        self.right = left_ptr;
        // self.right_mut().map(|mut right| {
        //     right.parent = Some(self_ptr);
        //     right
        // });

        // if let Some(parent) = self.parent_mut() {
        //     parent.update();
        // }
        // self.update();
    }

    fn rotate_right(&mut self) {
        let self_ptr = self.into();
        let parent_ptr = self.parent;
        let left_ptr = self.left;
        let left = match self.left_mut() {
            Some(ptr) => ptr,
            None => return,
        };
        let right_ptr = left.right;

        left.parent = parent_ptr;
        left.right = Some(self_ptr);

        let is_left = self.is_left();

        if let Some(parent) = self.parent_mut() {
            if is_left {
                parent.left = left_ptr;
            } else {
                parent.right = left_ptr;
            }
        }

        self.parent = left_ptr;
        self.left = right_ptr;
        // self.left_mut().map(|mut left| {
        //     left.parent = Some(self_ptr);
        //     left
        // });

        // if let Some(parent) = self.parent_mut() {
        //     parent.update();
        // }
        // self.update();
    }

    pub fn splay(&mut self, g: Option<&Node<K, V>>) -> Option<NonNull<Node<K, V>>> {
        loop {
            let ret = self.splay_internal(g);

            if ret.is_none() && g.is_none() {
                return Some(self.into());
            }

            if let Some(new_root) = ret {
                return Some(new_root);
            }
        }
    }

    fn splay_internal(&mut self, g: Option<&Node<K, V>>) -> Option<NonNull<Node<K, V>>> {
        let self_ptr = self.into();
        let is_left = self.is_left();

        if let Some(splay_type) = self.splay_type(g) {
            match splay_type {
                SplayType::Zig => {
                    self.parent_mut().map(|p| {
                        if is_left {
                            p.rotate_right();
                        } else {
                            p.rotate_left();
                        }
                    });
                }
                SplayType::ZigZig => {
                    self.parent_mut().map(|p| {
                        if is_left {
                            p.parent_mut().map(|g| g.rotate_right());
                            p.rotate_right();
                        } else {
                            p.parent_mut().map(|g| g.rotate_left());
                            p.rotate_left();
                        }
                    });
                }
                SplayType::ZigZag => {
                    if is_left {
                        self.parent_mut().map(|p| p.rotate_right());
                        self.parent_mut().map(|g| g.rotate_left());
                    } else {
                        self.parent_mut().map(|p| p.rotate_left());
                        self.parent_mut().map(|g| g.rotate_right());
                    }
                }
            }
        }

        if self.is_root() {
            Some(self_ptr)
        } else {
            None
        }
    }

    pub fn insert_child<'a>(&'a mut self, key: K, value: V) -> Option<&'a mut Self> {
        match key.cmp(&self.key) {
            Ordering::Less if self.left.is_none() => {
                let node = Node {
                    left: None,
                    right: None,
                    parent: Some(self.into()),
                    key,
                    value: value.clone(),
                    cnt: 1,
                    sum: value.clone(),
                    min: value.clone(),
                    max: value,
                    is_flip: false,
                };

                let node_ptr = Box::leak(Box::new(node)).into();
                self.left = Some(node_ptr);
                self.left_mut()
            }
            Ordering::Equal => Some(self),
            Ordering::Greater if self.right.is_none() => {
                let node = Node {
                    left: None,
                    right: None,
                    parent: Some(self.into()),
                    key,
                    value: value.clone(),
                    cnt: 1,
                    sum: value.clone(),
                    min: value.clone(),
                    max: value,
                    is_flip: false,
                };

                let node_ptr = Box::leak(Box::new(node)).into();
                self.right = Some(node_ptr);
                self.right_mut()
            }
            _ => None,
        }
    }

    pub fn find_max(&mut self) -> &mut Self {
        let mut cur_node = self;

        loop {
            let ptr: *const Self = cur_node;

            cur_node = if let Some(next) = cur_node.right_mut() {
                next
            } else {
                break unsafe { &mut *mem::transmute::<*const Self, *mut Self>(ptr) };
            }
        }
    }

    pub fn merge(&mut self, right: &mut Self) -> Option<NonNull<Node<K, V>>> {
        let left_max = self.find_max();
        let ret = left_max.splay(None);

        right.parent = ret;
        left_max.right = Some(right.into());

        ret
    }

    pub fn push(&mut self) {}

    pub fn update(&mut self) {
        self.cnt = 1;
        self.cnt += self
            .left
            .map(|left| unsafe { left.as_ref().cnt })
            .unwrap_or(0);
        self.cnt += self
            .right
            .map(|right| unsafe { right.as_ref().cnt })
            .unwrap_or(0);
    }
}

pub enum Entry<'a, K: Ord, V: Clone> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K: Ord, V: Clone> Entry<'a, K, V> {
    pub fn or_insert(self, value: V) -> &'a mut Node<K, V> {
        match self {
            Entry::Occupied(entry) => entry.elem,
            Entry::Vacant(entry) => entry.insert(value),
        }
    }

    pub fn or_insert_with<F: FnOnce() -> V>(self, calc: F) -> &'a mut Node<K, V> {
        match self {
            Entry::Occupied(entry) => entry.elem,
            Entry::Vacant(entry) => entry.insert(calc()),
        }
    }

    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, calc: F) -> &'a mut Node<K, V> {
        match self {
            Entry::Occupied(entry) => entry.elem,
            Entry::Vacant(entry) => {
                let val = calc(&entry.key);
                entry.insert(val)
            }
        }
    }

    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => entry.elem.key(),
            Entry::Vacant(entry) => &entry.key,
        }
    }

    pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self {
        match self {
            Entry::Occupied(entry) => {
                f(entry.elem.value_mut());
                Entry::Occupied(entry)
            }
            _ => self,
        }
    }

    pub fn insert(self, value: V) -> &'a mut Node<K, V> {
        match self {
            Entry::Occupied(entry) => {
                *entry.elem.value_mut() = value;
                entry.elem
            }
            Entry::Vacant(entry) => entry.insert(value),
        }
    }
}

impl<'a, K: Ord, V: Clone + Default> Entry<'a, K, V> {
    pub fn or_default(self) -> &'a mut Node<K, V> {
        match self {
            Entry::Occupied(entry) => entry.elem,
            Entry::Vacant(entry) => entry.insert(V::default()),
        }
    }
}

pub struct OccupiedEntry<'a, K: Ord, V: Clone> {
    elem: &'a mut Node<K, V>,
}

impl<'a, K: Ord, V: Clone> OccupiedEntry<'a, K, V> {
    pub fn new(elem: &'a mut Node<K, V>) -> Self {
        OccupiedEntry { elem }
    }
}

pub struct VacantEntry<'a, K: Ord, V: Clone> {
    tree: &'a mut SplayTree<K, V>,
    parent: Option<&'a mut Node<K, V>>,
    key: K,
}

impl<'a, K: Ord, V: Clone> VacantEntry<'a, K, V> {
    pub fn new_root(tree: &'a mut SplayTree<K, V>, key: K) -> Self {
        VacantEntry {
            tree: tree,
            parent: None,
            key: key,
        }
    }

    pub fn new_elem(tree: &'a mut SplayTree<K, V>, parent: &'a mut Node<K, V>, key: K) -> Self {
        VacantEntry {
            tree: tree,
            parent: Some(parent),
            key: key,
        }
    }

    fn insert(self, value: V) -> &'a mut Node<K, V> {
        self.tree
            .insert_child(self.parent, self.key, value)
            .unwrap()
    }
}

enum FindResult<K: Ord, V: Clone> {
    Found(*mut Node<K, V>),
    GoDown(*mut Node<K, V>),
    NotFound,
}

pub struct SplayTree<K: Ord, V: Clone> {
    root: Option<NonNull<Node<K, V>>>,
    length: usize,
    marker: PhantomData<Box<Node<K, V>>>,
}

impl<K: Ord, V: Clone> SplayTree<K, V> {
    pub fn new() -> Self {
        SplayTree {
            root: None,
            length: 0,
            marker: PhantomData,
        }
    }

    pub fn root(&self) -> Option<&Node<K, V>> {
        self.root.map(|r| unsafe { r.as_ref() })
    }

    pub fn root_mut(&mut self) -> Option<&mut Node<K, V>> {
        self.root.map(|mut r| unsafe { r.as_mut() })
    }

    pub fn get(&mut self, key: &K) -> Option<&Node<K, V>> {
        match self.find_ptr(key) {
            FindResult::Found(node_ptr) => unsafe { Some(&*node_ptr) },
            _ => None,
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut Node<K, V>> {
        match self.find_ptr(key) {
            FindResult::Found(node_ptr) => unsafe { Some(&mut *node_ptr) },
            _ => None,
        }
    }

    pub fn entry<'a>(&'a mut self, key: K) -> Entry<'a, K, V> {
        match self.find_ptr(&key) {
            FindResult::NotFound => Entry::Vacant(VacantEntry::new_root(self, key)),
            FindResult::GoDown(parent_ptr) => Entry::Vacant(VacantEntry::new_elem(
                self,
                unsafe { &mut *parent_ptr },
                key,
            )),
            FindResult::Found(node_ptr) => {
                Entry::Occupied(OccupiedEntry::new(unsafe { &mut *node_ptr }))
            }
        }
    }

    fn find_ptr(&mut self, key: &K) -> FindResult<K, V> {
        let mut cur_node = if let Some(root) = self.root_mut() {
            root
        } else {
            return FindResult::NotFound;
        };
        let mut is_found = false;

        loop {
            let ptr: *mut Node<K, V> = cur_node;
            let next_node = match key.cmp(cur_node.key()) {
                Ordering::Less => cur_node.left_mut(),
                Ordering::Equal => {
                    is_found = true;
                    self.root = cur_node.splay(None);
                    None
                }
                Ordering::Greater => cur_node.right_mut(),
            };

            cur_node = if let Some(next) = next_node {
                next
            } else if is_found {
                return FindResult::Found(ptr);
            } else {
                return FindResult::GoDown(ptr);
            };
        }
    }

    pub fn insert<'a>(&'a mut self, key: K, value: V) -> &'a mut Node<K, V> {
        self.entry(key).insert(value)
    }

    pub fn insert_child<'a>(
        &'a mut self,
        maybe_parent: Option<&'a mut Node<K, V>>,
        key: K,
        value: V,
    ) -> Option<&'a mut Node<K, V>> {
        if let Some(parent) = maybe_parent {
            let node = parent.insert_child(key, value)?;
            self.root = node.splay(None);
            self.length += 1;
            Some(node)
        } else if self.root.is_none() {
            let node = Box::leak(Box::new(Node::new(key, value))).into();
            self.root = Some(node);
            self.length += 1;
            self.root_mut()
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<Box<Node<K, V>>> {
        let node = self.get_mut(key)?;
        let node_ptr: *const Node<K, V> = node;
        let left = node
            .left()
            .map(|l| unsafe { &mut *mem::transmute::<*const Node<K, V>, *mut Node<K, V>>(l) });
        let right = node.right_mut();

        self.root = match (left, right) {
            (Some(l), Some(r)) => {
                l.parent = None;
                r.parent = None;
                l.merge(r)
            }
            (Some(l), None) => {
                l.parent = None;
                Some(l.into())
            }
            (None, Some(r)) => {
                r.parent = None;
                Some(r.into())
            }
            _ => None,
        };

        self.length -= 1;

        unsafe {
            let node = &mut *mem::transmute::<*const Node<K, V>, *mut Node<K, V>>(node_ptr);
            node.left = None;
            node.right = None;
            Some(node.ref_into_box())
        }
    }

    pub fn find(&mut self, key: &K) -> bool {
        self.get(&key).is_some()
    }

    pub fn find_kth(&mut self, mut k: usize) {
        let mut cur_node = self.root_mut().unwrap();

        loop {
            while cur_node.left().is_some() && cur_node.left().unwrap().cnt > k {
                cur_node = cur_node.left_mut().unwrap();
            }

            if cur_node.left().is_some() {
                k -= cur_node.left().unwrap().cnt;
            }

            if k == 0 {
                break;
            }

            k -= 1;

            if cur_node.right().is_some() {
                cur_node = cur_node.right_mut().unwrap();
            } else {
                break;
            }
        }

        cur_node.splay(None);
    }

    pub fn flip(&mut self, start: usize, end: usize) {
        let ret = self.gather(start, end);
        ret.is_flip = !ret.is_flip;
    }

    pub fn gather(&mut self, start: usize, end: usize) -> &mut Node<K, V> {
        self.find_kth(end + 1);
        let tmp_ptr: *const Node<K, V> = self.root_mut().unwrap();
        self.find_kth(start - 1);

        unsafe {
            let tmp = &mut *mem::transmute::<*const Node<K, V>, *mut Node<K, V>>(tmp_ptr);
            tmp.splay(Some(self.root_mut().unwrap()));
        }

        self.root_mut()
            .unwrap()
            .right_mut()
            .unwrap()
            .left_mut()
            .unwrap()
    }

    pub fn shift(&mut self, start: usize, end: usize, offset: usize) {}
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<i64>(), scan.token::<i64>());
    let mut splay_tree = SplayTree::new();

    for i in 1..=n {
        splay_tree.insert(i, i);
    }

    for _ in 0..q {
        let num = scan.token::<i64>();

        if num == 1 {
            let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
            splay_tree.flip(l, r);
            let t = splay_tree.gather(l, r);
            writeln!(out, "{} {} {}", t.min, t.max, t.sum).unwrap();
        } else if num == 2 {
            let (l, r, x) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
            );
            splay_tree.shift(l, r, x);
        } else if num == 3 {
            let i = scan.token::<i64>();
        } else {
            let x = scan.token::<i64>();
        }
    }
}
