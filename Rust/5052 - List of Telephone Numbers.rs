use io::Write;
use std::{collections::HashMap, io, str};

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

pub struct TrieNode {
    value: Option<char>,
    is_final: bool,
    child_nodes: HashMap<char, TrieNode>,
}

impl TrieNode {
    // Create new node
    pub fn new(c: char, is_final: bool) -> TrieNode {
        TrieNode {
            value: Option::Some(c),
            is_final: is_final,
            child_nodes: HashMap::new(),
        }
    }

    pub fn new_root() -> TrieNode {
        TrieNode {
            value: Option::None,
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
    pub fn insert(&mut self, string_val: String) -> bool {
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
            current_node.is_final = true;
            !current_node.child_nodes.is_empty()
        } else {
            let ret = current_node.is_final;

            for new_counter in last_match..char_list.len() {
                current_node.insert_value(char_list[new_counter], false);
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[new_counter])
                    .unwrap();
            }

            current_node.is_final = true;
            ret
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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut trie = TrieStruct::create();

        let n = scan.token::<i64>();
        let mut ret = true;

        for _ in 0..n {
            let phone_number = scan.token::<String>();

            if !ret {
                continue;
            }

            if trie.insert(phone_number.clone()) {
                ret = false;
            }
        }

        writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
    }
}
