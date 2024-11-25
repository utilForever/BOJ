use io::Write;
use std::{array, io, str};

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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

#[derive(Debug)]
struct TrieNode {
    idx: i64,
    is_end_of_word: bool,
    children: [Option<Box<TrieNode>>; 26],
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
            idx: -1,
            is_end_of_word: false,
            children: array::from_fn(|_| None),
        }
    }
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str, idx: usize) {
        let mut node = &mut self.root;

        for c in word.chars() {
            let index = (c as u8 - b'A') as usize;

            if node.children[index].is_none() {
                node.children[index] = Some(Box::new(TrieNode::default()));
            }

            node = node.children[index].as_mut().unwrap();
        }

        node.is_end_of_word = true;
        node.idx = idx as i64;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = &self.root;

        for c in word.chars() {
            let index = (c as u8 - b'A') as usize;

            if node.children[index].is_none() {
                return false;
            }

            node = node.children[index].as_ref().unwrap();
        }

        node.is_end_of_word
    }
}

const DY: [i64; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
const DX: [i64; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

static mut BOARD: [[char; 1000]; 1000] = [[' '; 1000]; 1000];
static mut RET: [Option<(usize, usize, usize)>; 1000] = [None; 1000];
static mut L: usize = 0;
static mut C: usize = 0;
static mut Y_START: usize = 0;
static mut X_START: usize = 0;

unsafe fn process_dfs(node: &TrieNode, y: i64, x: i64, dir: usize) {
    if y < 0 || y >= L as i64 || x < 0 || x >= C as i64 {
        return;
    }

    let y = y as usize;
    let x = x as usize;
    let ch = BOARD[y][x];
    let index = (ch as u8 - b'A') as usize;

    if node.children[index].is_none() {
        return;
    }

    let node = node.children[index].as_ref().unwrap();

    if node.is_end_of_word && RET[node.idx as usize].is_none() {
        RET[node.idx as usize] = Some((Y_START, X_START, dir));
    }

    let y_next = y as i64 + DY[dir];
    let x_next = x as i64 + DX[dir];

    process_dfs(node, y_next, x_next, dir);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (l, c, w) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    for i in 0..l {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            unsafe {
                BOARD[i][j] = c;
            }
        }
    }

    let mut trie = Trie::new();

    for i in 0..w {
        let word = scan.token::<String>();
        trie.insert(&word, i);
    }

    unsafe {
        L = l;
        C = c;
    }

    for i in 0..l {
        for j in 0..c {
            for k in 0..8 {
                unsafe {
                    Y_START = i;
                    X_START = j;
                }

                unsafe {
                    process_dfs(&trie.root, i as i64, j as i64, k);
                }
            }
        }
    }

    unsafe {
        for i in 0..w {
            let (y, x, dir) = RET[i].unwrap();
            
            writeln!(out, "{y} {x} {}", (b'A' + dir as u8) as char).unwrap();
        }
    }
}
