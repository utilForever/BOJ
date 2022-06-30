use io::Write;
use std::{cmp::Ordering, collections::HashMap, io, str};
use Ordering::{Greater, Less};

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

pub trait Ext {
    type Item;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;

    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord;

    fn upper_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T> Ext for [T] {
    type Item = T;

    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }
    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Less { mid } else { base };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp == Less) as usize
    }

    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Greater { base } else { mid };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp != Greater) as usize
    }
}

pub struct TrieNode {
    value: Option<char>,
    indexes: Vec<usize>,
    is_final: bool,
    child_nodes: HashMap<char, TrieNode>,
}

impl TrieNode {
    // Create new node
    pub fn new(c: char, is_final: bool) -> TrieNode {
        TrieNode {
            value: Option::Some(c),
            indexes: Vec::new(),
            is_final: is_final,
            child_nodes: HashMap::new(),
        }
    }

    pub fn new_root() -> TrieNode {
        TrieNode {
            value: Option::None,
            indexes: Vec::new(),
            is_final: false,
            child_nodes: HashMap::new(),
        }
    }

    // Check if a node has that value
    pub fn check_value(self, c: char) -> bool {
        self.value == Some(c)
    }

    pub fn insert_value(&mut self, c: char, is_final: bool) {
        self.child_nodes.insert(c, TrieNode::new(c, is_final));
    }
}

struct TrieStruct {
    root_node: TrieNode,
}

impl TrieStruct {
    // Create a TrieStruct
    pub fn create() -> TrieStruct {
        TrieStruct {
            root_node: TrieNode::new_root(),
        }
    }

    // Insert a string
    pub fn insert(&mut self, string_val: String, idx: usize) {
        let mut current_node = &mut self.root_node;
        let char_list: Vec<char> = string_val.chars().collect();
        let mut last_match = 0;

        for letter_counter in 0..char_list.len() {
            current_node.indexes.push(idx);

            if current_node
                .child_nodes
                .contains_key(&char_list[letter_counter])
            {
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[letter_counter])
                    .unwrap();
            } else {
                last_match = letter_counter;
                break;
            }

            last_match = letter_counter + 1;
        }

        if last_match == char_list.len() {
            current_node.indexes.push(idx);
            current_node.is_final = true;
        } else {
            for new_counter in last_match..char_list.len() {
                current_node.insert_value(char_list[new_counter], false);
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[new_counter])
                    .unwrap();
                current_node.indexes.push(idx);
            }

            current_node.is_final = true;
        }
    }

    // Find a string
    pub fn find(&mut self, string_val: String) -> bool {
        let mut current_node = &mut self.root_node;
        let char_list: Vec<char> = string_val.chars().collect();

        for counter in 0..char_list.len() {
            if !current_node.child_nodes.contains_key(&char_list[counter]) {
                return false;
            } else {
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[counter])
                    .unwrap();
            }
        }
        return true;
    }

    pub fn query(&mut self, string_val: String, left: usize, right: usize) -> usize {
        let mut current_node = &mut self.root_node;
        let char_list: Vec<char> = string_val.chars().collect();
        let mut last_match = 0;

        for letter_counter in 0..char_list.len() {
            if current_node
                .child_nodes
                .contains_key(&char_list[letter_counter])
            {
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[letter_counter])
                    .unwrap();
            } else {
                last_match = letter_counter;
                break;
            }

            last_match = letter_counter + 1;
        }

        if last_match == char_list.len() {
            let idx_left = current_node.indexes.lower_bound(&left);
            let idx_right = current_node.indexes.lower_bound(&right);

            idx_right - idx_left
        } else {
            0
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i32>());
    let mut trie = TrieStruct::create();
    let mut rna_chains = Vec::new();
    let mut reversed_rna_chains = Vec::new();

    for _ in 0..n {
        rna_chains.push(scan.token::<String>());
    }

    rna_chains.sort();

    for i in 0..n {
        reversed_rna_chains.push(rna_chains[i].chars().rev().collect::<String>());
    }

    for i in 0..n {
        trie.insert(reversed_rna_chains[i].clone(), i + 1);
    }

    rna_chains.insert(0, "".to_string());

    for _ in 0..m {
        let (mut p, q) = (scan.token::<String>(), scan.token::<String>());
        let q = q.chars().rev().collect::<String>();

        let left = rna_chains.lower_bound(&p);
        let next_char = p.as_bytes()[p.len() - 1] + 1;
        unsafe {
            let bytes = p.as_bytes_mut();
            *bytes.last_mut().unwrap() = next_char;
        }
        let right = rna_chains.upper_bound(&p);

        writeln!(out, "{}", trie.query(q, left, right)).unwrap();
    }
}
