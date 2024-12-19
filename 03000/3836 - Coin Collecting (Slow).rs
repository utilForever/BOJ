use io::Write;
use std::collections::{BTreeMap, VecDeque};
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

#[derive(Clone)]
struct UnionFind {
    edges: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            edges: vec![-1; n],
        }
    }

    fn init(&mut self, n: usize) {
        self.edges = vec![-1; n];
    }

    fn get(&mut self, x: usize) -> i32 {
        if self.edges[x] < 0 {
            x as i32
        } else {
            self.edges[x] = self.get(self.edges[x] as usize);
            self.edges[x]
        }
    }

    fn same_set(&mut self, a: usize, b: usize) -> bool {
        self.get(a) == self.get(b)
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut a = self.get(x) as usize;
        let mut b = self.get(y) as usize;

        if a == b {
            return false;
        }

        if self.edges[a] > self.edges[b] {
            std::mem::swap(&mut a, &mut b);
        }

        self.edges[a] += self.edges[b];
        self.edges[b] = a as i32;

        true
    }
}

// Ground set element data type
#[allow(dead_code)]
#[derive(Debug)]
enum ElementData {
    Vector(i64),        // Used for Linear Matroid
    Edge(usize, usize), // Used for Graphic Matroid
}

// A single element of the ground set
#[derive(Debug)]
struct GroundSetElement {
    color_id: usize,              // Color identifier for colorful matroids
    data: ElementData,            // Type-specific data: vector or edge
    is_in_independent_set: bool,  // Whether this element is currently in the independent set
    independent_set_index: usize, // The position of this element in the independent set if it is included
}

// MatroidOracle trait: defines the basic interface for a matroid oracle
trait MatroidOracle {
    type Element;

    // Prepare the oracle with the given elements and a given independent set
    fn prepare(&mut self, elements: &[Self::Element], independent_set: &[usize]);

    // Check if an element (by id) can be inserted into the independent set while maintaining independence
    fn can_insert(&mut self, elements: &[Self::Element], independent_set: &[usize], id: usize) -> bool;

    // Check if we can exchange one element in the independent set with another element not in it
    fn can_exchange(
        &mut self,
        elements: &[Self::Element],
        independent_set: &[usize],
        id_inserted: usize,
        id_removed: usize,
    ) -> bool;
}

// Colorful Matroid Oracle
// Constraint: At most one element of each color can be chosen
#[allow(dead_code)]
struct ColorfulOracle {
    color_used: Vec<bool>,
}

#[allow(dead_code)]
impl ColorfulOracle {
    fn new(color_count: usize) -> Self {
        ColorfulOracle {
            // Index 0 unused, colors from 1 to color_count
            color_used: vec![false; color_count + 1],
        }
    }
}

#[allow(dead_code)]
impl MatroidOracle for ColorfulOracle {
    type Element = GroundSetElement;

    fn prepare(&mut self, elements: &[Self::Element], independent_set: &[usize]) {
        self.color_used.fill(false);

        // Mark which colors are already used by the current independent set
        for &idx in independent_set {
            self.color_used[elements[idx].color_id] = true;
        }
    }

    fn can_insert(
        &mut self,
        elements: &[Self::Element],
        _independent_set: &[usize],
        id: usize,
    ) -> bool {
        let c = elements[id].color_id;
        !self.color_used[c]
    }

    fn can_exchange(
        &mut self,
        elements: &[Self::Element],
        _independent_set: &[usize],
        id_inserted: usize,
        id_removed: usize,
    ) -> bool {
        let color_inserted = elements[id_inserted].color_id;
        let color_removed = elements[id_removed].color_id;

        // If the inserted color is not used yet, exchange is possible
        if !self.color_used[color_inserted] {
            return true;
        }

        // If the colors match, swapping them maintains color constraints
        color_inserted == color_removed
    }
}

// Linear Matroid Oracle
// Based on linear independence of vectors
struct LinearOracle {
    basis: LinearBasis,
    basis_without: Vec<LinearBasis>, // Basis sets when removing each element from the independent set
}

#[allow(dead_code)]
#[derive(Clone)]
struct LinearBasis {
    vectors: Vec<i64>,
}

#[allow(dead_code)]
impl LinearBasis {
    fn new() -> Self {
        LinearBasis {
            vectors: Vec::new(),
        }
    }

    fn add_vector(&mut self, v: i64) {
        self.vectors.push(v);
    }

    fn reset(&mut self) {
        self.vectors.clear();
    }

    fn size(&self) -> usize {
        self.vectors.len()
    }

    // Simple Gaussian elimination-like process for bitwise vectors
    fn gaussian_elimination(&mut self) {
        let n = self.size();

        for i in 0..n {
            // Sort to maintain some order (not the most efficient, but keeps logic simple)
            for j in i..n {
                if self.vectors[i] < self.vectors[j] {
                    self.vectors.swap(i, j);
                }
            }

            // Reduce subsequent vectors
            for j in i + 1..n {
                let candidate = self.vectors[j] ^ self.vectors[i];
                self.vectors[j] = self.vectors[j].min(candidate);
            }
        }
    }

    // Check if a new vector can be independent of the current basis
    fn independent_with(&self, mut new_vector: i64) -> bool {
        for &v in self.vectors.iter() {
            let candidate = (new_vector ^ v).min(new_vector);
            new_vector = candidate;
        }

        // If after reduction, new_vector is still > 0, it adds new info
        new_vector > 0
    }
}

#[allow(dead_code)]
impl LinearOracle {
    fn new() -> Self {
        LinearOracle {
            basis: LinearBasis::new(),
            basis_without: vec![],
        }
    }
}

#[allow(dead_code)]
impl MatroidOracle for LinearOracle {
    type Element = GroundSetElement;

    fn prepare(&mut self, elements: &[Self::Element], independent_set: &[usize]) {
        // Reset the main basis
        self.basis.reset();
        // Prepare an array of basis sets for each scenario where one element is excluded
        self.basis_without = vec![LinearBasis::new(); independent_set.len()];

        // Construct the basis from the current independent set
        for (i, &idx_i) in independent_set.iter().enumerate() {
            if let ElementData::Vector(v) = elements[idx_i].data {
                self.basis.add_vector(v);
            }

            // Construct basis_without[i]: the basis excluding element at idx_i
            for (j, &idx_j) in independent_set.iter().enumerate() {
                if i == j {
                    continue;
                }
                
                if let ElementData::Vector(vj) = elements[idx_j].data {
                    self.basis_without[i].add_vector(vj);
                }
            }
        }

        // Perform Gaussian elimination on the main basis
        self.basis.gaussian_elimination();

        // Also eliminate for each basis_without
        for basis in &mut self.basis_without {
            basis.gaussian_elimination();
        }
    }

    fn can_insert(
        &mut self,
        elements: &[Self::Element],
        _independent_set: &[usize],
        id: usize,
    ) -> bool {
        if let ElementData::Vector(v) = elements[id].data {
            self.basis.independent_with(v)
        } else {
            false
        }
    }

    fn can_exchange(
        &mut self,
        elements: &[Self::Element],
        _independent_set: &[usize],
        id_inserted: usize,
        id_removed: usize,
    ) -> bool {
        let pos = elements[id_removed].independent_set_index;

        if pos < self.basis_without.len() {
            if let ElementData::Vector(v) = elements[id_inserted].data {
                self.basis_without[pos].independent_with(v)
            } else {
                false
            }
        } else {
            false
        }
    }
}

// Graphic Matroid Oracle
// Based on independence of edges in a graph (no cycles)
#[allow(dead_code)]
struct GraphicOracle {
    node_count: usize, // The number of nodes in the underlying graph
    basis: GraphBasis, // The "basis" representing a forest structure formed by the current independent set
    basis_without: Vec<GraphBasis>, // A collection of "basis_without" structures, each representing a basis
                                    // formed by the current independent set excluding a particular element
}

#[allow(dead_code)]
#[derive(Clone)]
struct GraphBasis {
    uf: UnionFind,
    n: usize,
}

#[allow(dead_code)]
impl GraphBasis {
    fn new(n: usize) -> Self {
        Self {
            uf: UnionFind::new(n),
            n,
        }
    }

    fn reset(&mut self) {
        self.uf.init(self.n);
    }

    fn add(&mut self, a: usize, b: usize) {
        self.uf.unite(a, b);
    }

    fn independent_with(&mut self, a: usize, b: usize) -> bool {
        !self.uf.same_set(a, b)
    }
}

#[allow(dead_code)]
impl GraphicOracle {
    fn new(n: usize) -> Self {
        GraphicOracle {
            node_count: n,
            basis: GraphBasis::new(n),
            basis_without: vec![GraphBasis::new(n); n],
        }
    }
}

#[allow(dead_code)]
impl MatroidOracle for GraphicOracle {
    type Element = GroundSetElement;

    fn prepare(&mut self, elements: &[Self::Element], independent_set: &[usize]) {
        // Rebuild the basis from scratch
        self.basis.reset();
        // Prepare basis_without for each element in the independent set
        self.basis_without = vec![GraphBasis::new(self.node_count); independent_set.len()];

        for (i, &idx_i) in independent_set.iter().enumerate() {
            if let ElementData::Edge(u, v) = elements[idx_i].data {
                self.basis.add(u, v);
            }

            // basis_without[i]: basis excluding element at idx_i
            for j in 0..independent_set.len() {
                if i == j {
                    continue;
                }

                if let ElementData::Edge(u, v) = elements[idx_i].data {
                    self.basis_without[j].add(u, v);
                }
            }
        }
    }

    fn can_insert(
        &mut self,
        elements: &[Self::Element],
        _independent_set: &[usize],
        id: usize,
    ) -> bool {
        if let ElementData::Edge(u, v) = elements[id].data {
            self.basis.independent_with(u, v)
        } else {
            false
        }
    }

    fn can_exchange(
        &mut self,
        elements: &[Self::Element],
        _independent_set: &[usize],
        id_inserted: usize,
        id_removed: usize,
    ) -> bool {
        let pos = elements[id_removed].independent_set_index;

        if pos < self.basis_without.len() {
            if let ElementData::Edge(u, v) = elements[id_inserted].data {
                self.basis_without[pos].independent_with(u, v)
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[allow(dead_code)]
struct MatroidIntersection {
    elements: Vec<GroundSetElement>,
    independent_set: Vec<usize>,
}

#[allow(dead_code)]
impl MatroidIntersection {
    fn new(elements: Vec<GroundSetElement>) -> Self {
        MatroidIntersection {
            elements,
            independent_set: Vec::new(),
        }
    }

    // The key augmentation routine used in matroid intersection
    //
    // Given two matroid oracles O1 and O2, we try to find an augmenting path:
    // - Treat elements not in the IS that can be inserted into O1 as "sources"
    // - BFS in a graph where edges represent possible exchanges or insertions
    // - If we find a chain that leads to an element that can be inserted into O2,
    //   we have found an augmenting path to increase the size of the independent set
    fn augment<
        O1: MatroidOracle<Element = GroundSetElement>,
        O2: MatroidOracle<Element = GroundSetElement>,
    >(
        &mut self,
        oracle1: &mut O1,
        oracle2: &mut O2,
    ) -> bool {
        const AUGMENT_SOURCE: i32 = -1;
        const UNVISITED: i32 = -2;
        const NO_AUGMENT_PATH: i32 = -3;

        // Prepare both oracles with the current set
        oracle1.prepare(&self.elements, &self.independent_set);
        oracle2.prepare(&self.elements, &self.independent_set);

        let sz = self.elements.len();
        let mut parent = vec![UNVISITED; sz];
        let mut queue = VecDeque::new();

        // Initialize BFS with elements that can be inserted into oracle1 (O1)
        // These act like "starting points" for finding an augmenting path
        for i in 0..sz {
            if oracle1.can_insert(&self.elements, &self.independent_set, i) {
                parent[i] = AUGMENT_SOURCE;
                queue.push_back(i);
            }
        }

        let mut endpoint = NO_AUGMENT_PATH;

        // BFS to find an augmenting path
        while let Some(cur) = queue.pop_front() {
            if self.elements[cur].is_in_independent_set {
                // If 'cur' is currently in the independent set,
                // we try to find elements that could replace 'cur' via O1 exchange
                for nxt in 0..sz {
                    if parent[nxt] != UNVISITED {
                        continue;
                    }

                    // Try to exchange 'cur' with 'nxt' using O1
                    if !oracle1.can_exchange(&self.elements, &self.independent_set, nxt, cur) {
                        continue;
                    }

                    parent[nxt] = cur as i32;
                    queue.push_back(nxt);
                }
            } else {
                // If 'cur' is not in the independent set,
                // first check if it can be inserted into O2. If yes, we found an augmenting path
                if oracle2.can_insert(&self.elements, &self.independent_set, cur) {
                    endpoint = cur as i32;
                    break;
                }

                // Otherwise, try O2 exchanges with elements in the independent set
                for &to in self.independent_set.iter() {
                    if parent[to] != UNVISITED {
                        continue;
                    }

                    // Try to exchange 'to' with 'cur' using O2
                    if !oracle2.can_exchange(&self.elements, &self.independent_set, cur, to) {
                        continue;
                    }

                    parent[to] = cur as i32;
                    queue.push_back(to);
                }
            }
        }

        // If no endpoint was found, no augmentation is possible
        if endpoint == NO_AUGMENT_PATH {
            return false;
        }

        // We found an augmenting path. Flip the state of elements along this path
        let mut e = endpoint;
        
        while e != AUGMENT_SOURCE {
            let idx = e as usize;

            self.elements[idx].is_in_independent_set = !self.elements[idx].is_in_independent_set;
            e = parent[idx];
        }

        // Rebuild the independent_set vector based on updated is_in_independent_set flags
        self.independent_set.clear();

        for (i, el) in self.elements.iter_mut().enumerate() {
            if el.is_in_independent_set {
                el.independent_set_index = self.independent_set.len();
                self.independent_set.push(i);
            }
        }

        true
    }

    // Matroid intersection using Linear + Colorful matroids
    fn process_linear_colorful(&mut self) {
        // Determine the maximum color ID
        let max_color = self.elements.iter().map(|x| x.color_id).max().unwrap();

        let mut linear_oracle = LinearOracle::new();
        let mut colorful_oracle = ColorfulOracle::new(max_color);

        // Keep augmenting until no more augmenting paths can be found
        while self.augment(&mut colorful_oracle, &mut linear_oracle) {}
    }

    // Matroid intersection using Graphic + Colorful matroids
    fn process_graphic_colorful(&mut self, node_count: usize) {
        let max_color = self.elements.iter().map(|x| x.color_id).max().unwrap();

        let mut graphic_oracle = GraphicOracle::new(node_count);
        let mut colorful_oracle = ColorfulOracle::new(max_color);

        // Keep augmenting until no more augmenting paths can be found
        while self.augment(&mut colorful_oracle, &mut graphic_oracle) {}
    }
}

// Reference: https://codeforces.com/blog/entry/69287
// Reference: https://github.com/infossm/infossm.github.io/blob/master/_posts/2019-05-08-introduction-to-matroid.md
// Reference: https://github.com/infossm/infossm.github.io/blob/master/_posts/2019-06-17-Matroid-Intersection.md
// Reference: https://github.com/ShahjalalShohag/code-library/blob/main/Miscellaneous/Matroid%20Intersection%20Color%20Graphic%20Matroid.cpp
// Reference: https://github.com/ShahjalalShohag/code-library/blob/main/Miscellaneous/Matroid%20Intersection%20Color%20Linear%20Matroid.cpp
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let r = scan.token::<usize>();

        if r == 0 {
            break;
        }

        let mut coins = Vec::with_capacity(2 * r);
        let mut map = BTreeMap::new();

        for i in 0..r {
            let (a, b, c, d) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
            );

            coins.push(GroundSetElement {
                color_id: i,
                data: ElementData::Edge(a, b),
                is_in_independent_set: false,
                independent_set_index: 0,
            });

            coins.push(GroundSetElement {
                color_id: i,
                data: ElementData::Edge(c, d),
                is_in_independent_set: false,
                independent_set_index: 0,
            });

            map.insert(a, 0);
            map.insert(b, 0);
            map.insert(c, 0);
            map.insert(d, 0);
        }

        let mut idx = 0;

        for value in map.iter_mut() {
            *value.1 = idx;
            idx += 1;
        }

        for coin in coins.iter_mut() {
            if let ElementData::Edge(a, b) = coin.data {
                coin.data = ElementData::Edge(*map.get(&a).unwrap(), *map.get(&b).unwrap());
            }
        }

        let mut matroid = MatroidIntersection::new(coins);
        matroid.process_graphic_colorful(idx);

        writeln!(out, "{}", 2 * matroid.independent_set.len()).unwrap();
    }
}