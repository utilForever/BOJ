use io::Write;
use std::{collections::VecDeque, io, str};

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

const AA: i64 = 1 << 0;
const AO: i64 = 1 << 1;
const BB: i64 = 1 << 2;
const BO: i64 = 1 << 3;
const OO: i64 = 1 << 4;
const AB: i64 = 1 << 5;

const A: i64 = AA + AO + AB;
const B: i64 = BB + BO + AB;
const O: i64 = AO + BO + OO;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut blood_types = vec![0; n + 1];

    // A = 1 or 2 (AA or AO)
    // B = 4 or 8 (BB or BO)
    // O = 16 (OO)
    // AB = 32 (AB)
    for i in 1..=n {
        let blood_type = scan.token::<String>();

        match blood_type.as_str() {
            "A" => blood_types[i] = AA + AO,
            "B" => blood_types[i] = BB + BO,
            "O" => blood_types[i] = OO,
            "AB" => blood_types[i] = AB,
            _ => unreachable!("Invalid blood type"),
        }
    }

    let mut relationships = vec![Vec::new(); n + 1];
    let mut parent1 = vec![0; n + 1];
    let mut parent2 = vec![0; n + 1];
    let mut degrees = vec![0; n + 1];

    // a: child, b: parent1, c: parent2
    for _ in 0..m {
        let (a, b, c) = (
            scan.token::<usize>() + 1,
            scan.token::<usize>() + 1,
            scan.token::<usize>() + 1,
        );

        relationships[b].push(a);
        relationships[c].push(a);

        parent1[a] = b;
        parent2[a] = c;
        degrees[a] = 2;
    }

    let mut queue = VecDeque::new();

    // Push the root nodes to the queue
    for i in 1..=n {
        if degrees[i] == 0 {
            queue.push_back(i);
        }
    }

    // Check if the child's blood type is compatible with the parent's blood type
    // If not compatible, print "Regretful..."
    // Otherwise, print "Good Family Tree"
    while let Some(parent) = queue.pop_front() {
        if parent1[parent] != 0 && parent2[parent] != 0 {
            let blood_type_parent1 = blood_types[parent1[parent]];
            let blood_type_parent2 = blood_types[parent2[parent]];
            let mut blood_type_child_candidate = 0;

            if (blood_type_parent1 & A) != 0 && (blood_type_parent2 & A) != 0 {
                blood_type_child_candidate |= AA;
            }

            if (blood_type_parent1 & A) != 0 && (blood_type_parent2 & O) != 0 {
                blood_type_child_candidate |= AO;
            }

            if (blood_type_parent1 & O) != 0 && (blood_type_parent2 & A) != 0 {
                blood_type_child_candidate |= AO;
            }

            if (blood_type_parent1 & B) != 0 && (blood_type_parent2 & B) != 0 {
                blood_type_child_candidate |= BB;
            }

            if (blood_type_parent1 & B) != 0 && (blood_type_parent2 & O) != 0 {
                blood_type_child_candidate |= BO;
            }

            if (blood_type_parent1 & O) != 0 && (blood_type_parent2 & B) != 0 {
                blood_type_child_candidate |= BO;
            }

            if (blood_type_parent1 & O) != 0 && (blood_type_parent2 & O) != 0 {
                blood_type_child_candidate |= OO;
            }

            if (blood_type_parent1 & A) != 0 && (blood_type_parent2 & B) != 0 {
                blood_type_child_candidate |= AB;
            }

            if (blood_type_parent1 & B) != 0 && (blood_type_parent2 & A) != 0 {
                blood_type_child_candidate |= AB;
            }

            if (blood_type_child_candidate & blood_types[parent]) == 0 {
                writeln!(out, "Regretful...").unwrap();
                return;
            }

            blood_types[parent] &= blood_type_child_candidate;
        }

        for &child in relationships[parent].iter() {
            degrees[child] -= 1;

            if degrees[child] == 0 {
                queue.push_back(child);
            }
        }
    }

    writeln!(out, "Good Family Tree").unwrap();
}
