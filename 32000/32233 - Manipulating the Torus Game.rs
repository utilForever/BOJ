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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

pub mod bitset {
    use std::iter::FromIterator;

    const ONE_VALUE_LENGTH: usize = 63;
    const MAXIMUM: u64 = (1u64 << ONE_VALUE_LENGTH as u64) - 1;

    pub fn get_bit_position(index: usize) -> (usize, usize) {
        let data_index = index / ONE_VALUE_LENGTH;
        let bit_index = index % ONE_VALUE_LENGTH;

        (data_index, bit_index)
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct BitSet {
        data: Vec<u64>,
    }

    impl std::ops::BitOrAssign for BitSet {
        fn bitor_assign(&mut self, rhs: Self) {
            if self.data.len() < rhs.data.len() {
                self.data.resize(rhs.data.len(), 0);
            }

            let n = if self.data.len() > rhs.data.len() {
                rhs.data.len()
            } else {
                self.data.len()
            };

            for i in 0..n {
                assert!(self.data[i] <= MAXIMUM);
                assert!(rhs.data[i] <= MAXIMUM);

                self.data[i] |= rhs.data[i];
            }
        }
    }

    impl std::ops::BitXorAssign for BitSet {
        fn bitxor_assign(&mut self, rhs: Self) {
            if self.data.len() < rhs.data.len() {
                self.data.resize(rhs.data.len(), 0);
            }

            let n = if self.data.len() > rhs.data.len() {
                rhs.data.len()
            } else {
                self.data.len()
            };

            for i in 0..n {
                assert!(self.data[i] <= MAXIMUM);
                assert!(rhs.data[i] <= MAXIMUM);

                self.data[i] ^= rhs.data[i];
            }
        }
    }

    impl std::ops::Shl<usize> for BitSet {
        type Output = Self;

        fn shl(self, rhs: usize) -> Self {
            self.shift_left(rhs)
        }
    }

    impl BitSet {
        pub fn new(n: usize) -> Self {
            let size = (n + ONE_VALUE_LENGTH - 1) / ONE_VALUE_LENGTH;

            BitSet {
                data: vec![0; size],
            }
        }

        pub fn new_from(value: u64) -> Self {
            BitSet { data: vec![value] }
        }

        pub fn set(&mut self, index: usize, value: bool) {
            let (data_index, bit_index) = get_bit_position(index);
            assert!(self.data.len() > data_index);

            if value {
                self.data[data_index] |= 1u64 << bit_index;
            } else {
                let tmp = MAXIMUM ^ 1 << (bit_index as u64);
                self.data[data_index] &= tmp;
            }
        }

        pub fn get(&self, index: usize) -> bool {
            let (data_index, bit_index) = get_bit_position(index);
            assert!(self.data.len() > data_index);

            self.data[data_index] & (1u64 << bit_index as u64) != 0
        }

        pub fn shift_left(&self, shift: usize) -> Self {
            let mut next_data = Vec::new();
            let prefix_empty_count = shift / ONE_VALUE_LENGTH;
            let shift_count = (shift % ONE_VALUE_LENGTH) as u64;

            for _ in 0..prefix_empty_count {
                next_data.push(0);
            }

            let mut from_previous = 0;
            let room = ONE_VALUE_LENGTH as u64 - shift_count;

            for &data in self.data.iter() {
                let overflow = (data >> room) << room;
                let rest = data - overflow;
                let value = (rest << shift_count) + from_previous;
                assert!(value <= MAXIMUM);

                next_data.push(value);
                from_previous = overflow >> room;
            }

            if from_previous > 0 {
                next_data.push(from_previous);
            }

            BitSet { data: next_data }
        }

        pub fn iter(&self) -> BitSetIterator {
            BitSetIterator {
                bitsets: self,
                index: 0,
            }
        }
    }

    pub struct BitSetIterator<'a> {
        bitsets: &'a BitSet,
        index: usize,
    }

    impl<'a> Iterator for BitSetIterator<'a> {
        type Item = bool;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.bitsets.data.len() * ONE_VALUE_LENGTH {
                return None;
            }

            let result = Some(self.bitsets.get(self.index));
            self.index += 1;

            result
        }
    }

    impl<'a> FromIterator<bool> for BitSet {
        fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
            let mut data = Vec::new();
            let mut current_value: u64 = 0;
            let mut count = 0;

            for (index, value) in iter.into_iter().enumerate() {
                if value {
                    current_value |= 1u64 << (index % ONE_VALUE_LENGTH);
                }
                count += 1;

                if count % ONE_VALUE_LENGTH == 0 {
                    data.push(current_value);
                    current_value = 0;
                }
            }

            if count % ONE_VALUE_LENGTH != 0 {
                data.push(current_value);
            }

            BitSet { data }
        }
    }
}

fn torus_nimber(n: u64) -> u64 {
    if n <= 7 {
        n * (n - 1) / 2
    } else {
        3 * n
    }
}

fn sphere_nimber(n: u64) -> u64 {
    if n == 2 {
        1
    } else {
        3 * n - 6
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut tori = vec![0; 1000000];

    for i in 0..n {
        tori[i] = scan.token::<u64>();
    }

    let mut mat = vec![bitset::BitSet::new(1000000); 63];
    let mut mat_c = vec![false; 63];
    let mut grundy = vec![0; 1000000];
    let mut val = 0;

    for j in 0..n {
        let nimber_torus = torus_nimber(tori[j]);
        let nimber_sphere = sphere_nimber(tori[j]);

        val ^= nimber_torus;
        grundy[j] = nimber_torus ^ nimber_sphere;

        for i in 0..63 {
            mat[i].set(j, (grundy[j] & (1 << i)) != 0);
        }
    }

    for i in 0..63 {
        mat_c[i] = val & (1 << i) != 0;
    }

    let mut r = 0;

    for j in 0..n {
        if r >= 63 {
            break;
        }

        for i in r..63 {
            if !mat[i].get(j) {
                continue;
            }

            mat.swap(r, i);
            mat_c.swap(r, i);
            break;
        }

        if !mat[r].get(j) {
            continue;
        }

        for i in r + 1..63 {
            if !mat[i].get(j) {
                continue;
            }

            let mat_r = mat[r].clone();
            mat[i] ^= mat_r;
            mat_c[i] = mat_c[i] != mat_c[r];
        }

        r += 1;
    }

    for i in (1..63).rev() {
        if mat[i].iter().all(|x| !x) {
            continue;
        }

        let lz = mat[i].iter().position(|x| x).unwrap();

        for j in (0..i).rev() {
            if !mat[j].get(lz) {
                continue;
            }

            let mat_i = mat[i].clone();
            mat[j] ^= mat_i;
            mat_c[j] = mat_c[j] != mat_c[i];
        }
    }

    let mut indices = Vec::new();
    let mut idx = 0;

    for i in 0..63 {
        if mat_c[i] && mat[i].iter().all(|x| !x) {
            writeln!(out, "N").unwrap();
            return;
        }
    }

    writeln!(out, "Y").unwrap();

    for i in 0..63 {
        if !mat_c[i] {
            continue;
        }

        while idx < n && !mat[i].get(idx) {
            idx += 1;
        }

        indices.push(idx + 1);
    }

    writeln!(out, "{}", indices.len()).unwrap();

    for idx in indices {
        write!(out, "{idx} ").unwrap();
    }

    writeln!(out).unwrap();
}
