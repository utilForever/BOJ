use io::Write;
use std::{collections::VecDeque, fmt, io, str, usize};

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
enum Position {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl From<usize> for Position {
    fn from(value: usize) -> Self {
        match value {
            0 => Position::Up,
            1 => Position::Down,
            2 => Position::Left,
            3 => Position::Right,
            4 => Position::Front,
            5 => Position::Back,
            _ => panic!("Invalid position value"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
enum Direction {
    Normal = 1,
    Half = 2,
    Prime = 3,
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            1 => Direction::Normal,
            2 => Direction::Half,
            3 => Direction::Prime,
            _ => panic!("Invalid direction value"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Move(Position, Direction);

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self(Position::Front, Direction::Normal) => write!(f, "F"),
            Self(Position::Front, Direction::Half) => write!(f, "F2"),
            Self(Position::Front, Direction::Prime) => write!(f, "F'"),
            Self(Position::Back, Direction::Normal) => write!(f, "B"),
            Self(Position::Back, Direction::Half) => write!(f, "B2"),
            Self(Position::Back, Direction::Prime) => write!(f, "B'"),
            Self(Position::Left, Direction::Normal) => write!(f, "L"),
            Self(Position::Left, Direction::Half) => write!(f, "L2"),
            Self(Position::Left, Direction::Prime) => write!(f, "L'"),
            Self(Position::Right, Direction::Normal) => write!(f, "R"),
            Self(Position::Right, Direction::Half) => write!(f, "R2"),
            Self(Position::Right, Direction::Prime) => write!(f, "R'"),
            Self(Position::Up, Direction::Normal) => write!(f, "U"),
            Self(Position::Up, Direction::Half) => write!(f, "U2"),
            Self(Position::Up, Direction::Prime) => write!(f, "U'"),
            Self(Position::Down, Direction::Normal) => write!(f, "D"),
            Self(Position::Down, Direction::Half) => write!(f, "D2"),
            Self(Position::Down, Direction::Prime) => write!(f, "D'"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[allow(dead_code)]
#[rustfmt::skip]
enum Facelet {
    U1 = 0,  U2 = 1,  U3 = 2,
    U4 = 3,  U5 = 4,  U6 = 5,
    U7 = 6,  U8 = 7,  U9 = 8,

    R1 = 9,  R2 = 10, R3 = 11,
    R4 = 12, R5 = 13, R6 = 14,
    R7 = 15, R8 = 16, R9 = 17,

    F1 = 18, F2 = 19, F3 = 20,
    F4 = 21, F5 = 22, F6 = 23,
    F7 = 24, F8 = 25, F9 = 26,

    D1 = 27, D2 = 28, D3 = 29,
    D4 = 30, D5 = 31, D6 = 32,
    D7 = 33, D8 = 34, D9 = 35,

    L1 = 36, L2 = 37, L3 = 38,
    L4 = 39, L5 = 40, L6 = 41,
    L7 = 42, L8 = 43, L9 = 44,

    B1 = 45, B2 = 46, B3 = 47,
    B4 = 48, B5 = 49, B6 = 50,
    B7 = 51, B8 = 52, B9 = 53,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
enum Corner {
    URF = 0,
    UFL = 1,
    ULB = 2,
    UBR = 3,
    DFR = 4,
    DLF = 5,
    DBL = 6,
    DRB = 7,
}

impl From<usize> for Corner {
    fn from(value: usize) -> Self {
        match value {
            0 => Corner::URF,
            1 => Corner::UFL,
            2 => Corner::ULB,
            3 => Corner::UBR,
            4 => Corner::DFR,
            5 => Corner::DLF,
            6 => Corner::DBL,
            7 => Corner::DRB,
            _ => panic!("Invalid corner index"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
enum Edge {
    UR = 0,
    UF = 1,
    UL = 2,
    UB = 3,
    DR = 4,
    DF = 5,
    DL = 6,
    DB = 7,
    FR = 8,
    FL = 9,
    BL = 10,
    BR = 11,
}

impl From<usize> for Edge {
    fn from(value: usize) -> Self {
        match value {
            0 => Edge::UR,
            1 => Edge::UF,
            2 => Edge::UL,
            3 => Edge::UB,
            4 => Edge::DR,
            5 => Edge::DF,
            6 => Edge::DL,
            7 => Edge::DB,
            8 => Edge::FR,
            9 => Edge::FL,
            10 => Edge::BL,
            11 => Edge::BR,
            _ => panic!("Invalid edge index"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct CubieCube {
    corner_pos: [Corner; 8],
    corner_ori: [u8; 8],
    edge_pos: [Edge; 12],
    edge_ori: [u8; 12],
}

impl Default for CubieCube {
    fn default() -> Self {
        CubieCube {
            corner_pos: [
                Corner::URF,
                Corner::UFL,
                Corner::ULB,
                Corner::UBR,
                Corner::DFR,
                Corner::DLF,
                Corner::DBL,
                Corner::DRB,
            ],
            corner_ori: [0; 8],
            edge_pos: [
                Edge::UR,
                Edge::UF,
                Edge::UL,
                Edge::UB,
                Edge::DR,
                Edge::DF,
                Edge::DL,
                Edge::DB,
                Edge::FR,
                Edge::FL,
                Edge::BL,
                Edge::BR,
            ],
            edge_ori: [0; 12],
        }
    }
}

trait Phase {
    fn preprocess(&mut self);
    fn solve(&self, cube: &mut CubieCube) -> Vec<Move>;
}

struct Phase1 {
    table: Vec<i8>,
}

impl Phase for Phase1 {
    fn preprocess(&mut self) {
        // ...
    }

    fn solve(&self, cube: &mut CubieCube) -> Vec<Move> {
        let mut moves = Vec::new();
        
        // ...

        moves
    }
}

impl Phase1 {
    fn new() -> Self {
        Self {
            table: vec![-1; 2048],
        }
    }
}

struct Phase2 {
    table: Vec<i8>,
    idx_e_slice: Vec<i16>,
}

impl Phase for Phase2 {
    fn preprocess(&mut self) {
        // ...
    }

    fn solve(&self, cube: &mut CubieCube) -> Vec<Move> {
        let mut moves = Vec::new();
        
        // ...

        moves
    }
}

impl Phase2 {
    fn new() -> Self {
        Self {
            table: vec![-1; 2187 * 495],
            idx_e_slice: Vec::new(),
        }
    }
}

struct Phase3 {
    table: Vec<i8>,
    idx_corner_permutation: Vec<i32>,
    idx_edge_class: Vec<i16>,
}

impl Phase for Phase3 {
    fn preprocess(&mut self) {
        // ...
    }

    fn solve(&self, cube: &mut CubieCube) -> Vec<Move> {
        let mut moves = Vec::new();
        
        // ...

        moves
    }
}

impl Phase3 {
    fn new() -> Self {
        Self {
            table: vec![-1; 40320 * 70],
            idx_corner_permutation: Vec::new(),
            idx_edge_class: Vec::new(),
        }
    }
}

struct Phase4 {
    table: Vec<i8>,
    idx_permutation: Vec<i8>,
}

impl Phase for Phase4 {
    fn preprocess(&mut self) {
        // ...
    }

    fn solve(&self, cube: &mut CubieCube) -> Vec<Move> {
        let mut moves = Vec::new();
        
        // ...

        moves
    }
}

impl Phase4 {
    fn new() -> Self {
        Self {
            table: vec![-1; 576 * 13824],
            idx_permutation: Vec::new(),
        }
    }
}

struct CubeSolver {
    phases: [Box<dyn Phase>; 4],
}

impl CubeSolver {
    fn new() -> Self {
        let mut phase1 = Phase1::new();
        phase1.preprocess();

        let mut phase2 = Phase2::new();
        phase2.preprocess();

        let mut phase3 = Phase3::new();
        phase3.preprocess();

        let mut phase4 = Phase4::new();
        phase4.preprocess();

        Self {
            phases: [
                Box::new(phase1),
                Box::new(phase2),
                Box::new(phase3),
                Box::new(phase4),
            ],
        }
    }

    fn solve(&self, cube: &mut CubieCube) -> Vec<Move> {
        let mut moves = Vec::new();

        for phase in &self.phases {
            let mut phase_moves = phase.solve(cube);
            moves.append(&mut phase_moves);
        }

        moves
    }
}

// Thanks for @jinhan814 to provide the important idea of the solution
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let solver = CubeSolver::new();

    for _ in 0..t {
        let mut color: [usize; 54] = [0; 54];

        for i in 0..54 {
            let idx = scan.token::<usize>();
            color[i] = idx;
        }

        let mut cube = CubieCube::from_color(&color);
        let ret = solver.solve(&mut cube);

        for val in ret {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
