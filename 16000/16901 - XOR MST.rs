use io::Write;
use std::{io, str};

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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Node {
    children: [Option<usize>; 2],
    count: u64,
}
impl Node {
    #[inline]
    fn new() -> Self {
        Self {
            children: [None; 2],
            count: 0,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BinaryTrie {
    nodes: Vec<Node>,
}

impl BinaryTrie {
    #[inline]
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new()],
        }
    }

    #[inline]
    pub fn insert(&mut self, x: u32) -> u64 {
        self.insert_n(x, 1)
    }

    #[inline]
    pub fn insert_n(&mut self, x: u32, n: u64) -> u64 {
        if n == 0 {
            return 0;
        }

        let mut idx_node = 0;

        for i in (0..32).rev() {
            self.nodes[idx_node].count += n;

            idx_node = match self.nodes[idx_node].children[(x >> i & 1) as usize] {
                Some(i) => i,
                None => {
                    self.nodes.push(Node::new());
                    self.nodes[idx_node].children[(x >> i & 1) as usize] =
                        Some(self.nodes.len() - 1);
                    self.nodes.len() - 1
                }
            };
        }

        self.nodes[idx_node].count += n;
        self.nodes[idx_node].count
    }

    #[inline]
    pub fn count(&self, x: u32) -> u64 {
        let mut idx_node = Some(0);

        for i in (0..32).rev() {
            if idx_node.is_none() {
                return 0;
            }

            idx_node = self.nodes[idx_node.unwrap()].children[(x >> i & 1) as usize];
        }

        if idx_node.is_none() {
            return 0;
        }

        self.nodes[idx_node.unwrap()].count
    }

    #[inline]
    pub fn count_less(&self, x: u32) -> u64 {
        self.inner_count_than(x, 1)
    }

    #[inline]
    pub fn count_more(&self, x: u32) -> u64 {
        self.inner_count_than(x, 0)
    }

    #[inline]
    fn inner_count_than(&self, x: u32, bit: u32) -> u64 {
        let mut idx_node = Some(0);
        let mut count = 0;

        for i in (0..32).rev() {
            if idx_node.is_none() {
                break;
            }

            if (x >> i & 1) == bit {
                count += match self.nodes[idx_node.unwrap()].children[(bit ^ 1) as usize] {
                    Some(i) => self.nodes[i].count,
                    None => 0,
                }
            }

            idx_node = self.nodes[idx_node.unwrap()].children[(x >> i & 1) as usize];
        }

        count
    }

    #[inline]
    pub fn erase(&mut self, x: u32) -> Option<()> {
        if self.count(x) < 1 {
            return None;
        }

        self.inner_erase(x, 1)
    }

    #[inline]
    pub fn erase_all(&mut self, x: u32) -> Option<()> {
        let cnt_erase = self.count(x);

        if cnt_erase == 0 {
            return None;
        }

        self.inner_erase(x, cnt_erase)
    }

    #[inline]
    fn inner_erase(&mut self, x: u32, cnt_erase: u64) -> Option<()> {
        let mut idx_node = Some(0);

        for i in (0..32).rev() {
            self.nodes[idx_node?].count -= cnt_erase;
            idx_node = self.nodes[idx_node?].children[(x >> i & 1) as usize];
        }

        self.nodes[idx_node?].count -= cnt_erase;

        Some(())
    }

    #[inline]
    pub fn xor_min(&self, x: u32) -> Option<u32> {
        let mut idx_node = Some(0);
        let mut ret = 0;

        for i in (0..32).rev() {
            let bit = {
                let mut buff = (x >> i & 1) as usize;

                if self.nodes[idx_node.unwrap()].children[buff]
                    .filter(|&index| self.nodes[index].count > 0)
                    .is_none()
                {
                    buff ^= 1;
                }

                buff
            };

            ret ^= (bit as u32) << i;
            idx_node = self.nodes[idx_node.unwrap()].children[bit];
        }

        Some(ret ^ x)
    }

    #[inline]
    pub fn min(&self) -> Option<u32> {
        self.nth_element(1)
    }

    #[inline]
    pub fn max(&self) -> Option<u32> {
        let max = self.size();
        self.nth_element(max)
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.nodes[0].count
    }

    #[inline]
    pub fn nth_element(&self, n: u64) -> Option<u32> {
        if n > self.size() || n == 0 {
            return None;
        }

        let mut idx_node = Some(0);
        let mut idx = n;
        let mut ret = 0;

        for i in (0..32).rev() {
            let count = if let Some(i) = self.nodes[idx_node.unwrap()].children[0] {
                self.nodes[i].count
            } else {
                0
            };

            let bit = if count >= idx {
                0
            } else {
                idx -= count;
                1
            };

            ret ^= (bit as u32) << i;
            idx_node = self.nodes[idx_node.unwrap()].children[bit];
        }

        Some(ret)
    }

    #[inline]
    pub fn lower_bound(&self, x: u32) -> Option<u32> {
        self.nth_element(self.count_less(x + 1))
    }

    #[inline]
    pub fn upper_bound(&self, x: u32) -> Option<u32> {
        self.nth_element(self.count_less(x + 1) + 1)
    }
}

fn process_mst(nums: &Vec<u32>, start: usize, end: usize, depth: i64) -> i64 {
    if start >= end || depth < 0 {
        return 0;
    }

    // Check the range does not need to be split at the current bit depth
    // (All numbers in the range either have the current bit set or unset)
    if (nums[start] & (1 << depth)) != 0 || (nums[end] & (1 << depth)) == 0 {
        return process_mst(nums, start, end, depth - 1);
    }

    // Find the splitting point where numbers transition from 0 to 1 for the current bit
    let mut mid = end;

    // Move left until we find a number with the bit unset
    while mid >= start && (nums[mid] & (1 << depth)) != 0 {
        mid -= 1;
    }

    mid += 1;

    let mut trie = BinaryTrie::new();

    // Insert all numbers in the right partition (where the current bit is set)
    for i in mid..=end {
        trie.insert(nums[i]);
    }

    // Find the minimum XOR for numbers in the left partition
    let mut val = u32::MAX;

    // Compute the minimum XOR value between the current number and the Binary Trie
    for i in start..mid {
        val = val.min(trie.xor_min(nums[i]).unwrap());
    }

    // If no valid XOR value was found, default to 0
    if val == u32::MAX {
        val = 0;
    }

    // Recursively process the left and right partitions
    let left = process_mst(nums, start, mid - 1, depth - 1);
    let right = process_mst(nums, mid, end, depth - 1);

    // Return the total weight: left + minimum XOR value + right
    left + val as i64 + right
}

// Reference: https://github.com/uesugi6111/competitive-library/blob/master/src/structure/binary_trie.rs
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<u32>();
    }

    nums.sort();

    writeln!(out, "{}", process_mst(&nums, 0, n - 1, 29)).unwrap();
}
