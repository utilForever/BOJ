use io::Write;
use std::cmp::Ordering;
use std::hash::Hash;
use std::iter::Sum;
use std::ops::Add;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io, str,
};

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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Facelet {
    Yellow,
    White,
    Green,
    Blue,
    Red,
    Orange,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Direction {
    R,
    L,
    D,
    U,
    F,
    B,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Amt {
    One,
    Two,
    Rev,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct FullMove {
    pub direction: Direction,
    pub amt: Amt,
}

pub const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::U,
    Direction::D,
    Direction::B,
    Direction::F,
    Direction::L,
    Direction::R,
];
pub const ALL_AMTS: [Amt; 3] = [Amt::One, Amt::Two, Amt::Rev];

pub trait ApplyMove: Sized {
    fn apply_many(self, moves: &[FullMove]) -> Self {
        let mut out = self;

        for m in moves {
            out = out.apply(*m);
        }

        out
    }

    fn apply(self, m: FullMove) -> Self;
}

impl<T> ApplyMove for T
where
    T: CanMove + Sized,
{
    fn apply(self, m: FullMove) -> Self {
        let FullMove { direction, amt } = m;

        match direction {
            Direction::R => match amt {
                Amt::One => self.r(),
                Amt::Two => self.r().r(),
                Amt::Rev => self.r().r().r(),
            },
            Direction::L => match amt {
                Amt::One => self.l(),
                Amt::Two => self.l().l(),
                Amt::Rev => self.l().l().l(),
            },
            Direction::D => match amt {
                Amt::One => self.d(),
                Amt::Two => self.d_two(),
                Amt::Rev => self.d().d().d(),
            },
            Direction::U => match amt {
                Amt::One => self.u(),
                Amt::Two => self.u_two(),
                Amt::Rev => self.u().u().u(),
            },
            Direction::F => match amt {
                Amt::One => self.f(),
                Amt::Two => self.f().f(),
                Amt::Rev => self.f().f().f(),
            },
            Direction::B => match amt {
                Amt::One => self.b(),
                Amt::Two => self.b().b(),
                Amt::Rev => self.b().b().b(),
            },
        }
    }
}

impl<A: CanMove, B: CanMove> CanMove for (A, B) {
    fn r(self) -> Self {
        (self.0.r(), self.1.r())
    }

    fn l(self) -> Self {
        (self.0.l(), self.1.l())
    }

    fn u(self) -> Self {
        (self.0.u(), self.1.u())
    }

    fn u_two(self) -> Self {
        (self.0.u_two(), self.1.u_two())
    }

    fn d(self) -> Self {
        (self.0.d(), self.1.d())
    }

    fn d_two(self) -> Self {
        (self.0.d_two(), self.1.d_two())
    }

    fn b(self) -> Self {
        (self.0.b(), self.1.b())
    }

    fn f(self) -> Self {
        (self.0.f(), self.1.f())
    }
}

pub trait CanMove: Sized {
    fn r(self) -> Self;

    fn l(self) -> Self;

    fn u(self) -> Self;

    fn u_two(self) -> Self {
        self.u().u()
    }

    fn d(self) -> Self;

    fn d_two(self) -> Self {
        self.d().d()
    }

    fn b(self) -> Self;

    fn f(self) -> Self;
}

impl CanMove for Cube {
    #[inline(always)]
    fn r(self) -> Self {
        let Self { u, d, l, r, f, b } = self;

        Self {
            l,
            r: LRFace {
                // rotate corners
                ub: r.uf,
                uf: r.df,
                df: r.db,
                db: r.ub,
                // rotate edges
                uc: r.fc,
                fc: r.dc,
                dc: r.bc,
                bc: r.uc,
                // the center abides
                cc: r.cc,
            },
            d: UDFace {
                fr: b.dr,
                rc: b.rc,
                br: b.ur,
                ..d
            },
            u: UDFace {
                fr: f.dr,
                rc: f.rc,
                br: f.ur,
                ..u
            },
            f: FBFace {
                ur: d.fr,
                rc: d.rc,
                dr: d.br,
                ..f
            },
            b: FBFace {
                ur: u.fr,
                rc: u.rc,
                dr: u.br,
                ..b
            },
        }
    }

    #[inline(always)]
    fn l(self) -> Self {
        let Self { u, d, l, r, f, b } = self;

        Self {
            r,
            l: LRFace {
                // rotate the corners
                uf: l.ub,
                ub: l.db,
                db: l.df,
                df: l.uf,
                // rotate the edges
                fc: l.uc,
                uc: l.bc,
                bc: l.dc,
                dc: l.fc,
                // the center abides
                cc: l.cc,
            },
            u: UDFace {
                fl: b.ul,
                lc: b.lc,
                bl: b.dl,
                ..u
            },
            d: UDFace {
                fl: f.ul,
                lc: f.lc,
                bl: f.dl,
                ..d
            },
            f: FBFace {
                ul: u.bl,
                lc: u.lc,
                dl: u.fl,
                ..f
            },
            b: FBFace {
                ul: d.bl,
                lc: d.lc,
                dl: d.fl,
                ..b
            },
        }
    }

    #[inline(always)]
    fn u(self) -> Self {
        let Self { u, d, b, f, r, l } = self;

        Self {
            d,
            u: UDFace {
                // rotate corners
                fr: u.br,
                br: u.bl,
                bl: u.fl,
                fl: u.fr,
                // rotate edges
                fc: u.rc,
                rc: u.bc,
                bc: u.lc,
                lc: u.fc,
                // center abides
                cc: u.cc,
            },
            r: LRFace {
                uf: b.ur,
                uc: b.uc,
                ub: b.ul,
                ..r
            },
            l: LRFace {
                uf: f.ur,
                uc: f.uc,
                ub: f.ul,
                ..l
            },
            f: FBFace {
                ul: r.uf,
                uc: r.uc,
                ur: r.ub,
                ..f
            },
            b: FBFace {
                ur: l.ub,
                uc: l.uc,
                ul: l.uf,
                ..b
            },
        }
    }

    #[inline(always)]
    fn d(self) -> Self {
        let Self { u, d, f, b, r, l } = self;

        Self {
            u,
            d: UDFace {
                // rotate corners
                fr: d.fl,
                fl: d.bl,
                bl: d.br,
                br: d.fr,
                // rotate edges
                fc: d.lc,
                lc: d.bc,
                bc: d.rc,
                rc: d.fc,
                // center abides
                cc: d.cc,
            },
            r: LRFace {
                db: f.dr,
                dc: f.dc,
                df: f.dl,
                ..r
            },
            l: LRFace {
                df: b.dl,
                dc: b.dc,
                db: b.dr,
                ..l
            },
            b: FBFace {
                dl: r.db,
                dc: r.dc,
                dr: r.df,
                ..b
            },
            f: FBFace {
                dl: l.db,
                dc: l.dc,
                dr: l.df,
                ..f
            },
        }
    }

    #[inline(always)]
    fn b(self) -> Self {
        let Self { u, d, b, f, l, r } = self;
        Self {
            f,
            b: FBFace {
                // rotate edges
                ul: b.ur,
                ur: b.dr,
                dr: b.dl,
                dl: b.ul,
                // rotate corners
                uc: b.rc,
                rc: b.dc,
                dc: b.lc,
                lc: b.uc,
                // center abides
                cc: b.cc,
            },
            r: LRFace {
                ub: d.br,
                bc: d.bc,
                db: d.bl,
                ..r
            },
            l: LRFace {
                ub: u.br,
                bc: u.bc,
                db: u.bl,
                ..l
            },
            u: UDFace {
                bl: r.ub,
                bc: r.bc,
                br: r.db,
                ..u
            },
            d: UDFace {
                bl: l.ub,
                bc: l.bc,
                br: l.db,
                ..d
            },
        }
    }

    #[inline(always)]
    fn f(self) -> Self {
        let Self { u, d, b, f, l, r } = self;

        Self {
            b,
            f: FBFace {
                // rotate corners
                ul: f.dl,
                dl: f.dr,
                dr: f.ur,
                ur: f.ul,
                // rotate edges
                uc: f.lc,
                lc: f.dc,
                dc: f.rc,
                rc: f.uc,
                // center abides
                cc: f.cc,
            },
            r: LRFace {
                uf: u.fl,
                fc: u.fc,
                df: u.fr,
                ..r
            },
            l: LRFace {
                df: d.fr,
                fc: d.fc,
                uf: d.fl,
                ..l
            },
            u: UDFace {
                fl: l.df,
                fc: l.fc,
                fr: l.uf,
                ..u
            },
            d: UDFace {
                fl: r.df,
                fc: r.fc,
                fr: r.uf,
                ..d
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct UDFace {
    pub bl: Facelet,
    pub bc: Facelet,
    pub br: Facelet,
    pub lc: Facelet,
    pub cc: Facelet,
    pub rc: Facelet,
    pub fl: Facelet,
    pub fc: Facelet,
    pub fr: Facelet,
}

impl UDFace {
    fn new(
        bl: Facelet,
        bc: Facelet,
        br: Facelet,
        lc: Facelet,
        cc: Facelet,
        rc: Facelet,
        fl: Facelet,
        fc: Facelet,
        fr: Facelet,
    ) -> Self {
        Self {
            bl,
            bc,
            br,
            lc,
            cc,
            rc,
            fl,
            fc,
            fr,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct LRFace {
    pub ub: Facelet,
    pub uc: Facelet,
    pub uf: Facelet,
    pub bc: Facelet,
    pub cc: Facelet,
    pub fc: Facelet,
    pub db: Facelet,
    pub dc: Facelet,
    pub df: Facelet,
}

impl LRFace {
    fn new(
        ub: Facelet,
        uc: Facelet,
        uf: Facelet,
        bc: Facelet,
        cc: Facelet,
        fc: Facelet,
        db: Facelet,
        dc: Facelet,
        df: Facelet,
    ) -> Self {
        Self {
            ub,
            uc,
            uf,
            bc,
            cc,
            fc,
            db,
            dc,
            df,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct FBFace {
    pub ul: Facelet,
    pub uc: Facelet,
    pub ur: Facelet,
    pub lc: Facelet,
    pub cc: Facelet,
    pub rc: Facelet,
    pub dl: Facelet,
    pub dc: Facelet,
    pub dr: Facelet,
}

impl FBFace {
    fn new(
        ul: Facelet,
        uc: Facelet,
        ur: Facelet,
        lc: Facelet,
        cc: Facelet,
        rc: Facelet,
        dl: Facelet,
        dc: Facelet,
        dr: Facelet,
    ) -> Self {
        Self {
            ul,
            uc,
            ur,
            lc,
            cc,
            rc,
            dl,
            dc,
            dr,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Cube {
    pub u: UDFace,
    pub d: UDFace,
    pub l: LRFace,
    pub r: LRFace,
    pub f: FBFace,
    pub b: FBFace,
}

impl Cube {
    fn new(faces: &Vec<Vec<Facelet>>) -> Self {
        let (up, left, front, right, back, down) = (
            &faces[0], &faces[1], &faces[2], &faces[3], &faces[4], &faces[5],
        );

        let up = UDFace::new(
            up[0].clone(),
            up[1].clone(),
            up[2].clone(),
            up[3].clone(),
            up[4].clone(),
            up[5].clone(),
            up[6].clone(),
            up[7].clone(),
            up[8].clone(),
        );
        let left = LRFace::new(
            left[0].clone(),
            left[1].clone(),
            left[2].clone(),
            left[3].clone(),
            left[4].clone(),
            left[5].clone(),
            left[6].clone(),
            left[7].clone(),
            left[8].clone(),
        );
        let front = FBFace::new(
            front[0].clone(),
            front[1].clone(),
            front[2].clone(),
            front[3].clone(),
            front[4].clone(),
            front[5].clone(),
            front[6].clone(),
            front[7].clone(),
            front[8].clone(),
        );
        let right = LRFace::new(
            right[2].clone(),
            right[1].clone(),
            right[0].clone(),
            right[5].clone(),
            right[4].clone(),
            right[3].clone(),
            right[8].clone(),
            right[7].clone(),
            right[6].clone(),
        );
        let back = FBFace::new(
            back[2].clone(),
            back[1].clone(),
            back[0].clone(),
            back[5].clone(),
            back[4].clone(),
            back[3].clone(),
            back[8].clone(),
            back[7].clone(),
            back[6].clone(),
        );
        let down = UDFace::new(
            down[6].clone(),
            down[7].clone(),
            down[8].clone(),
            down[3].clone(),
            down[4].clone(),
            down[5].clone(),
            down[0].clone(),
            down[1].clone(),
            down[2].clone(),
        );

        Self {
            u: up,
            d: down,
            l: left,
            r: right,
            f: front,
            b: back,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct CubePositions {
    pub edges: CubeEdgePositions,
    pub corners: CubeCornerPositions,
}

impl CubePositions {
    pub fn is_solved(&self) -> bool {
        self == &CubePositions::make_solved()
    }

    pub fn directly_solvable(&self) -> bool {
        self.edges.directly_solvable() == self.corners.directly_solvable()
    }

    pub fn make_solved() -> CubePositions {
        CubePositions {
            edges: CubeEdgePositions::make_solved(),
            corners: CubeCornerPositions::make_solved(),
        }
    }

    pub fn from_cube(cube: &Cube) -> Self {
        Self {
            edges: CubeEdgePositions::from_cube(cube),
            corners: CubeCornerPositions::from_cube(cube),
        }
    }
}

impl CanMove for CubePositions {
    fn r(self) -> Self {
        Self {
            edges: self.edges.r(),
            corners: self.corners.r(),
        }
    }

    fn l(self) -> Self {
        Self {
            edges: self.edges.l(),
            corners: self.corners.l(),
        }
    }

    fn u(self) -> Self {
        Self {
            edges: self.edges.u(),
            corners: self.corners.u(),
        }
    }

    fn d(self) -> Self {
        Self {
            edges: self.edges.d(),
            corners: self.corners.d(),
        }
    }

    fn b(self) -> Self {
        Self {
            edges: self.edges.b(),
            corners: self.corners.b(),
        }
    }

    fn f(self) -> Self {
        Self {
            edges: self.edges.f(),
            corners: self.corners.f(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum SideCubelet {
    UF,
    UR,
    UB,
    UL,
    FL,
    FR,
    BL,
    BR,
    DF,
    DR,
    DB,
    DL,
}

impl SideCubelet {
    pub fn to_index(&self) -> u8 {
        match self {
            SideCubelet::UL => 0,
            SideCubelet::FL => 1,
            SideCubelet::DL => 2,
            SideCubelet::BL => 3,

            SideCubelet::UF => 4,
            SideCubelet::DF => 5,
            SideCubelet::DB => 6,
            SideCubelet::UB => 7,

            SideCubelet::UR => 8,
            SideCubelet::BR => 9,
            SideCubelet::DR => 10,
            SideCubelet::FR => 11,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct CubeEdgePositions {
    pub uf: SideCubelet,
    pub ur: SideCubelet,
    pub ub: SideCubelet,
    pub ul: SideCubelet,

    pub fl: SideCubelet,
    pub fr: SideCubelet,
    pub bl: SideCubelet,
    pub br: SideCubelet,

    pub df: SideCubelet,
    pub dr: SideCubelet,
    pub db: SideCubelet,
    pub dl: SideCubelet,
}

impl CubeEdgePositions {
    pub fn make_solved() -> Self {
        Self {
            uf: SideCubelet::UF,
            ur: SideCubelet::UR,
            ub: SideCubelet::UB,
            ul: SideCubelet::UL,
            fl: SideCubelet::FL,
            fr: SideCubelet::FR,
            bl: SideCubelet::BL,
            br: SideCubelet::BR,
            df: SideCubelet::DF,
            dr: SideCubelet::DR,
            db: SideCubelet::DB,
            dl: SideCubelet::DL,
        }
    }

    fn ind(&self, index: u8) -> SideCubelet {
        match index {
            0 => self.ul.clone(),
            1 => self.fl.clone(),
            2 => self.dl.clone(),
            3 => self.bl.clone(),
            4 => self.uf.clone(),
            5 => self.df.clone(),
            6 => self.db.clone(),
            7 => self.ub.clone(),
            8 => self.ur.clone(),
            9 => self.br.clone(),
            10 => self.dr.clone(),
            11 => self.fr.clone(),
            _ => panic!("Out of range index {index}"),
        }
    }

    pub fn directly_solvable(&self) -> bool {
        let mut seen: HashSet<u8> = HashSet::default();
        let mut total_is_even = true;

        for i in 0..12 {
            if seen.contains(&i) {
                continue;
            }

            let mut cycle_length = 0;
            let mut next = i;

            while !seen.contains(&next) {
                seen.insert(next);
                next = self.ind(next).to_index();
                cycle_length += 1;
            }

            if cycle_length % 2 == 0 {
                total_is_even = !total_is_even;
            }
        }

        total_is_even
    }

    pub fn from_cube(cube: &Cube) -> Self {
        let l = &cube.l.cc;
        let r = &cube.r.cc;
        let u = &cube.u.cc;
        let d = &cube.d.cc;
        let f = &cube.f.cc;
        let b = &cube.b.cc;

        let find_pos = |a_in: &Facelet, b_in: &Facelet| -> SideCubelet {
            let is_match = |exp_a: &Facelet, exp_b: &Facelet| -> bool {
                (a_in == exp_a && b_in == exp_b) || (a_in == exp_b && b_in == exp_a)
            };

            if is_match(u, f) {
                SideCubelet::UF
            } else if is_match(u, r) {
                SideCubelet::UR
            } else if is_match(u, l) {
                SideCubelet::UL
            } else if is_match(u, b) {
                SideCubelet::UB
            } else if is_match(f, l) {
                SideCubelet::FL
            } else if is_match(f, r) {
                SideCubelet::FR
            } else if is_match(b, l) {
                SideCubelet::BL
            } else if is_match(b, r) {
                SideCubelet::BR
            } else if is_match(d, f) {
                SideCubelet::DF
            } else if is_match(d, r) {
                SideCubelet::DR
            } else if is_match(d, l) {
                SideCubelet::DL
            } else if is_match(d, b) {
                SideCubelet::DB
            } else {
                panic!(
                    "idk couldn't find a side pos for colors {:?} / {:?}",
                    a_in, b_in
                );
            }
        };

        Self {
            uf: find_pos(&cube.u.fc, &cube.f.uc),
            ur: find_pos(&cube.u.rc, &cube.r.uc),
            ub: find_pos(&cube.u.bc, &cube.b.uc),
            ul: find_pos(&cube.u.lc, &cube.l.uc),

            fl: find_pos(&cube.f.lc, &cube.l.fc),
            fr: find_pos(&cube.f.rc, &cube.r.fc),
            bl: find_pos(&cube.b.lc, &cube.l.bc),
            br: find_pos(&cube.b.rc, &cube.r.bc),

            df: find_pos(&cube.d.fc, &cube.f.dc),
            dr: find_pos(&cube.d.rc, &cube.r.dc),
            db: find_pos(&cube.d.bc, &cube.b.dc),
            dl: find_pos(&cube.d.lc, &cube.l.dc),
        }
    }
}

impl CanMove for CubeEdgePositions {
    fn r(self) -> Self {
        CubeEdgePositions {
            ur: self.fr,
            fr: self.dr,
            dr: self.br,
            br: self.ur,
            ..self
        }
    }

    fn l(self) -> Self {
        CubeEdgePositions {
            ul: self.bl,
            bl: self.dl,
            dl: self.fl,
            fl: self.ul,
            ..self
        }
    }

    fn u(self) -> Self {
        CubeEdgePositions {
            uf: self.ur,
            ur: self.ub,
            ub: self.ul,
            ul: self.uf,
            ..self
        }
    }

    fn d(self) -> Self {
        CubeEdgePositions {
            df: self.dl,
            dl: self.db,
            db: self.dr,
            dr: self.df,
            ..self
        }
    }

    fn b(self) -> Self {
        CubeEdgePositions {
            ub: self.bl,
            bl: self.db,
            db: self.br,
            br: self.ub,
            ..self
        }
    }

    fn f(self) -> Self {
        CubeEdgePositions {
            uf: self.fl,
            fl: self.df,
            df: self.fr,
            fr: self.uf,
            ..self
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum CornerCubelet {
    FUL,
    FUR,
    BUL,
    BUR,
    FDL,
    FDR,
    BDL,
    BDR,
}

impl CornerCubelet {
    pub fn to_index(&self) -> u8 {
        match self {
            CornerCubelet::FUL => 0,
            CornerCubelet::FUR => 1,
            CornerCubelet::BUR => 2,
            CornerCubelet::BUL => 3,
            CornerCubelet::FDL => 4,
            CornerCubelet::FDR => 5,
            CornerCubelet::BDR => 6,
            CornerCubelet::BDL => 7,
        }
    }
}

pub fn next_permutation(nums: &mut Vec<usize>) -> bool {
    let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            nums.reverse();
            return false;
        }
    };

    let swap_with = nums[last_ascending + 1..]
        .binary_search_by(|n| usize::cmp(&nums[last_ascending], n).then(Ordering::Less))
        .unwrap_err();
    nums.swap(last_ascending, last_ascending + swap_with);
    nums[last_ascending + 1..].reverse();

    true
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct CubeCornerPositions {
    pub ful: CornerCubelet,
    pub fur: CornerCubelet,
    pub bul: CornerCubelet,
    pub bur: CornerCubelet,
    pub fdl: CornerCubelet,
    pub fdr: CornerCubelet,
    pub bdl: CornerCubelet,
    pub bdr: CornerCubelet,
}

impl CubeCornerPositions {
    pub fn make_solved() -> Self {
        Self {
            fur: CornerCubelet::FUR,
            ful: CornerCubelet::FUL,
            bur: CornerCubelet::BUR,
            bul: CornerCubelet::BUL,
            fdr: CornerCubelet::FDR,
            fdl: CornerCubelet::FDL,
            bdr: CornerCubelet::BDR,
            bdl: CornerCubelet::BDL,
        }
    }

    fn ind(&self, index: u8) -> CornerCubelet {
        match index {
            0 => self.ful.clone(),
            1 => self.fur.clone(),
            2 => self.bur.clone(),
            3 => self.bul.clone(),
            4 => self.fdl.clone(),
            5 => self.fdr.clone(),
            6 => self.bdr.clone(),
            7 => self.bdl.clone(),
            _ => panic!("Out of range index {index}"),
        }
    }

    pub fn directly_solvable(&self) -> bool {
        let mut seen: HashSet<u8> = HashSet::default();
        let mut total_is_even = true;

        for i in 0..8 {
            if seen.contains(&i) {
                continue;
            }

            let mut cycle_length = 0;
            let mut next = i;

            while !seen.contains(&next) {
                seen.insert(next);
                next = self.ind(next).to_index();
                cycle_length += 1;
            }

            if cycle_length % 2 != 1 {
                total_is_even = !total_is_even;
            }
        }

        total_is_even
    }

    pub fn from_cube(cube: &Cube) -> Self {
        let l = &cube.l.cc;
        let r = &cube.r.cc;
        let u = &cube.u.cc;
        let d = &cube.d.cc;
        let f = &cube.f.cc;
        let b = &cube.b.cc;

        let find_pos = |a_in: &Facelet, b_in: &Facelet, c_in: &Facelet| -> CornerCubelet {
            let is_match = |exp_a: &Facelet, exp_b: &Facelet, exp_c: &Facelet| -> bool {
                let act = vec![a_in, b_in, c_in];
                let mut idxes = vec![0, 1, 2];

                loop {
                    let mut expes = Vec::new();

                    for idx in idxes.iter() {
                        expes.push(match idx {
                            0 => exp_a,
                            1 => exp_b,
                            2 => exp_c,
                            _ => panic!("impossible"),
                        });
                    }

                    if act == expes {
                        return true;
                    }

                    if !next_permutation(&mut idxes) {
                        break;
                    }
                }

                false
            };

            if is_match(f, u, r) {
                CornerCubelet::FUR
            } else if is_match(f, u, l) {
                CornerCubelet::FUL
            } else if is_match(f, d, r) {
                CornerCubelet::FDR
            } else if is_match(f, d, l) {
                CornerCubelet::FDL
            } else if is_match(b, u, r) {
                CornerCubelet::BUR
            } else if is_match(b, u, l) {
                CornerCubelet::BUL
            } else if is_match(b, d, r) {
                CornerCubelet::BDR
            } else if is_match(b, d, l) {
                CornerCubelet::BDL
            } else {
                panic!(
                    "idk couldn't find a corner pos for colors {:?} / {:?} / {:?}",
                    a_in, b_in, c_in
                );
            }
        };

        Self {
            fur: find_pos(&cube.f.ur, &cube.u.fr, &cube.r.uf),
            ful: find_pos(&cube.f.ul, &cube.u.fl, &cube.l.uf),
            fdr: find_pos(&cube.f.dr, &cube.d.fr, &cube.r.df),
            fdl: find_pos(&cube.f.dl, &cube.d.fl, &cube.l.df),

            bur: find_pos(&cube.b.ur, &cube.u.br, &cube.r.ub),
            bul: find_pos(&cube.b.ul, &cube.u.bl, &cube.l.ub),
            bdr: find_pos(&cube.b.dr, &cube.d.br, &cube.r.db),
            bdl: find_pos(&cube.b.dl, &cube.d.bl, &cube.l.db),
        }
    }
}

impl CanMove for CubeCornerPositions {
    fn r(self) -> Self {
        Self {
            fur: self.fdr,
            fdr: self.bdr,
            bdr: self.bur,
            bur: self.fur,
            ..self
        }
    }

    fn l(self) -> Self {
        Self {
            ful: self.bul,
            bul: self.bdl,
            bdl: self.fdl,
            fdl: self.ful,
            ..self
        }
    }

    fn u(self) -> Self {
        Self {
            fur: self.bur,
            bur: self.bul,
            bul: self.ful,
            ful: self.fur,
            ..self
        }
    }

    fn d(self) -> Self {
        Self {
            fdr: self.fdl,
            fdl: self.bdl,
            bdl: self.bdr,
            bdr: self.fdr,
            ..self
        }
    }

    fn b(self) -> Self {
        Self {
            bur: self.bdr,
            bdr: self.bdl,
            bdl: self.bul,
            bul: self.bur,
            ..self
        }
    }

    fn f(self) -> Self {
        Self {
            fur: self.ful,
            ful: self.fdl,
            fdl: self.fdr,
            fdr: self.fur,
            ..self
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EdgeOrientationState {
    pub uf: bool,
    pub ub: bool,
    pub ul: bool,
    pub ur: bool,

    pub fl: bool,
    pub fr: bool,
    pub bl: bool,
    pub br: bool,

    pub df: bool,
    pub db: bool,
    pub dl: bool,
    pub dr: bool,
}

impl EdgeOrientationState {
    pub fn from_cube(cube: &Cube) -> Self {
        let l_color = cube.l.cc.clone();
        let r_color = cube.r.cc.clone();
        let u_color = cube.u.cc.clone();
        let d_color = cube.d.cc.clone();
        let f_color = cube.f.cc.clone();
        let b_color = cube.b.cc.clone();

        let is_lr_color = |f: &Facelet| f == &l_color || f == &r_color;
        let is_ud_color = |f: &Facelet| f == &u_color || f == &d_color;
        let is_fb_color = |f: &Facelet| f == &f_color || f == &b_color;

        let lr_good = |lr: &Facelet, other: &Facelet| {
            (!is_fb_color(lr)) && !(is_ud_color(lr) && is_lr_color(other))
        };

        let ud_mid_good = |ud: &Facelet, fb: &Facelet| {
            (!is_fb_color(ud)) && !(is_ud_color(ud) && is_lr_color(fb))
        };

        Self {
            uf: ud_mid_good(&cube.u.fc, &cube.f.uc),
            ub: ud_mid_good(&cube.u.bc, &cube.b.uc),
            df: ud_mid_good(&cube.d.fc, &cube.f.dc),
            db: ud_mid_good(&cube.d.bc, &cube.b.dc),

            ul: lr_good(&cube.l.uc, &cube.u.lc),
            fl: lr_good(&cube.l.fc, &cube.f.lc),
            bl: lr_good(&cube.l.bc, &cube.b.lc),
            dl: lr_good(&cube.l.dc, &cube.d.lc),

            ur: lr_good(&cube.r.uc, &cube.u.rc),
            fr: lr_good(&cube.r.fc, &cube.f.rc),
            br: lr_good(&cube.r.bc, &cube.b.rc),
            dr: lr_good(&cube.r.dc, &cube.d.rc),
        }
    }

    #[inline(always)]
    pub fn make_solved() -> Self {
        Self {
            uf: true,
            ub: true,
            ul: true,
            ur: true,
            fl: true,
            fr: true,
            bl: true,
            br: true,
            df: true,
            db: true,
            dl: true,
            dr: true,
        }
    }

    pub fn is_solvable(&self) -> bool {
        let is_flipped = self.uf
            ^ self.ub
            ^ self.ul
            ^ self.ur
            ^ self.fl
            ^ self.fr
            ^ self.bl
            ^ self.br
            ^ self.df
            ^ self.db
            ^ self.dl
            ^ self.dr;

        !is_flipped
    }

    pub fn is_solved(&self) -> bool {
        self.uf
            && self.ub
            && self.ul
            && self.ur
            && self.fl
            && self.fr
            && self.bl
            && self.br
            && self.df
            && self.db
            && self.dl
            && self.dr
    }
}

impl CanMove for EdgeOrientationState {
    fn r(self) -> Self {
        Self {
            ur: self.fr,
            fr: self.dr,
            dr: self.br,
            br: self.ur,
            ..self
        }
    }

    fn l(self) -> Self {
        Self {
            ul: self.bl,
            bl: self.dl,
            dl: self.fl,
            fl: self.ul,
            ..self
        }
    }

    fn u(self) -> Self {
        Self {
            uf: !self.ur,
            ur: !self.ub,
            ub: !self.ul,
            ul: !self.uf,
            ..self
        }
    }

    fn d(self) -> Self {
        Self {
            df: !self.dl,
            dl: !self.db,
            db: !self.dr,
            dr: !self.df,
            ..self
        }
    }

    fn b(self) -> Self {
        Self {
            ub: self.br,
            br: self.db,
            db: self.bl,
            bl: self.ub,
            ..self
        }
    }

    fn f(self) -> Self {
        Self {
            uf: self.fl,
            fl: self.df,
            df: self.fr,
            fr: self.uf,
            ..self
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EdgeMidSliceState {
    uf: bool,
    ub: bool,
    ul: bool,
    ur: bool,

    fl: bool,
    fr: bool,
    bl: bool,
    br: bool,

    df: bool,
    db: bool,
    dl: bool,
    dr: bool,
}

impl EdgeMidSliceState {
    #[inline(always)]
    pub fn solved() -> Self {
        Self {
            uf: true,
            ub: true,
            ul: false,
            ur: false,
            fl: false,
            fr: false,
            bl: false,
            br: false,
            df: true,
            db: true,
            dl: false,
            dr: false,
        }
    }

    pub fn is_solved(&self) -> bool {
        self == &Self::solved()
    }

    pub fn from_cube(cube: &Cube) -> EdgeMidSliceState {
        let l_color = cube.l.cc.clone();
        let r_color = cube.r.cc.clone();
        let is_lr_color = |f: &Facelet| f == &l_color || f == &r_color;
        let is_mid_slice = |a: &Facelet, b: &Facelet| !is_lr_color(a) && !is_lr_color(b);

        EdgeMidSliceState {
            uf: is_mid_slice(&cube.u.fc, &cube.f.uc),
            ub: is_mid_slice(&cube.u.bc, &cube.b.uc),
            ul: is_mid_slice(&cube.u.lc, &cube.l.uc),
            ur: is_mid_slice(&cube.u.rc, &cube.r.uc),
            fl: is_mid_slice(&cube.f.lc, &cube.l.fc),
            fr: is_mid_slice(&cube.f.rc, &cube.r.fc),
            bl: is_mid_slice(&cube.b.lc, &cube.l.bc),
            br: is_mid_slice(&cube.b.rc, &cube.r.bc),
            df: is_mid_slice(&cube.d.fc, &cube.f.dc),
            db: is_mid_slice(&cube.d.bc, &cube.b.dc),
            dl: is_mid_slice(&cube.d.lc, &cube.l.dc),
            dr: is_mid_slice(&cube.d.rc, &cube.r.dc),
        }
    }
}

impl CanMove for EdgeMidSliceState {
    fn r(self) -> Self {
        EdgeMidSliceState {
            ur: self.fr,
            fr: self.dr,
            dr: self.br,
            br: self.ur,
            ..self
        }
    }

    fn l(self) -> Self {
        EdgeMidSliceState {
            ul: self.bl,
            bl: self.dl,
            dl: self.fl,
            fl: self.ul,
            ..self
        }
    }

    fn u(self) -> Self {
        EdgeMidSliceState {
            uf: self.ur,
            ur: self.ub,
            ub: self.ul,
            ul: self.uf,
            ..self
        }
    }

    fn d(self) -> Self {
        EdgeMidSliceState {
            df: self.dl,
            dl: self.db,
            db: self.dr,
            dr: self.df,
            ..self
        }
    }

    fn b(self) -> Self {
        EdgeMidSliceState {
            ub: self.br,
            br: self.db,
            db: self.bl,
            bl: self.ub,
            ..self
        }
    }

    fn f(self) -> Self {
        EdgeMidSliceState {
            uf: self.fl,
            fl: self.df,
            df: self.fr,
            fr: self.uf,
            ..self
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum CornerOrientation {
    Good,
    CW,
    CCW,
}

impl Add for CornerOrientation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            CornerOrientation::Good => rhs,
            CornerOrientation::CW => match rhs {
                CornerOrientation::Good => CornerOrientation::CW,
                CornerOrientation::CW => CornerOrientation::CCW,
                CornerOrientation::CCW => CornerOrientation::Good,
            },
            CornerOrientation::CCW => match rhs {
                CornerOrientation::Good => CornerOrientation::CCW,
                CornerOrientation::CW => CornerOrientation::Good,
                CornerOrientation::CCW => CornerOrientation::CW,
            },
        }
    }
}

impl Sum<CornerOrientation> for CornerOrientation {
    fn sum<I: Iterator<Item = CornerOrientation>>(iter: I) -> Self {
        iter.fold(CornerOrientation::Good, |a, b| a + b)
    }
}

impl CornerOrientation {
    fn cw(self) -> Self {
        match self {
            CornerOrientation::Good => CornerOrientation::CW,
            CornerOrientation::CW => CornerOrientation::CCW,
            CornerOrientation::CCW => CornerOrientation::Good,
        }
    }

    fn ccw(self) -> Self {
        match self {
            CornerOrientation::Good => CornerOrientation::CCW,
            CornerOrientation::CW => CornerOrientation::Good,
            CornerOrientation::CCW => CornerOrientation::CW,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CornerOrientationState {
    pub ful: CornerOrientation,
    pub fur: CornerOrientation,
    pub fdl: CornerOrientation,
    pub fdr: CornerOrientation,

    pub bul: CornerOrientation,
    pub bur: CornerOrientation,
    pub bdl: CornerOrientation,
    pub bdr: CornerOrientation,
}

impl CornerOrientationState {
    #[inline(always)]
    pub fn solved() -> Self {
        Self {
            ful: CornerOrientation::Good,
            fur: CornerOrientation::Good,
            fdl: CornerOrientation::Good,
            fdr: CornerOrientation::Good,
            bul: CornerOrientation::Good,
            bur: CornerOrientation::Good,
            bdl: CornerOrientation::Good,
            bdr: CornerOrientation::Good,
        }
    }

    fn total_orientation(&self) -> CornerOrientation {
        [
            self.ful, self.fur, self.fdl, self.fdr, self.bul, self.bur, self.bdl, self.bdr,
        ]
        .iter()
        .copied()
        .sum()
    }

    pub fn is_solvable(&self) -> bool {
        self.total_orientation() == CornerOrientation::Good
    }

    pub fn is_solved(&self) -> bool {
        self == &Self::solved()
    }

    pub fn from_cube(cube: &Cube) -> CornerOrientationState {
        let l_color = cube.l.cc.clone();
        let r_color = cube.r.cc.clone();
        let is_lr_color = |f: &Facelet| f == &l_color || f == &r_color;

        let orientation = |side: &Facelet, next: &Facelet| {
            if is_lr_color(side) {
                CornerOrientation::Good
            } else if is_lr_color(next) {
                CornerOrientation::CW
            } else {
                CornerOrientation::CCW
            }
        };

        CornerOrientationState {
            ful: orientation(&cube.l.uf, &cube.u.fl),
            fur: orientation(&cube.r.uf, &cube.f.ur),
            fdl: orientation(&cube.l.df, &cube.f.dl),
            fdr: orientation(&cube.r.df, &cube.d.fr),
            bul: orientation(&cube.l.ub, &cube.b.ul),
            bur: orientation(&cube.r.ub, &cube.u.br),
            bdl: orientation(&cube.l.db, &cube.d.bl),
            bdr: orientation(&cube.r.db, &cube.b.dr),
        }
    }
}

impl CanMove for CornerOrientationState {
    fn r(self) -> Self {
        CornerOrientationState {
            fur: self.fdr,
            fdr: self.bdr,
            bdr: self.bur,
            bur: self.fur,
            ..self
        }
    }

    fn l(self) -> Self {
        CornerOrientationState {
            ful: self.bul,
            bul: self.bdl,
            bdl: self.fdl,
            fdl: self.ful,
            ..self
        }
    }

    fn u(self) -> Self {
        CornerOrientationState {
            ful: self.fur.ccw(),
            fur: self.bur.cw(),
            bur: self.bul.ccw(),
            bul: self.ful.cw(),
            ..self
        }
    }

    fn u_two(self) -> Self {
        CornerOrientationState {
            ful: self.bur,
            bur: self.ful,
            fur: self.bul,
            bul: self.fur,
            ..self
        }
    }

    fn d(self) -> Self {
        CornerOrientationState {
            fdl: self.bdl.cw(),
            bdl: self.bdr.ccw(),
            bdr: self.fdr.cw(),
            fdr: self.fdl.ccw(),
            ..self
        }
    }

    fn b(self) -> Self {
        CornerOrientationState {
            bul: self.bur.ccw(),
            bur: self.bdr.cw(),
            bdr: self.bdl.ccw(),
            bdl: self.bul.cw(),
            ..self
        }
    }

    fn f(self) -> Self {
        CornerOrientationState {
            ful: self.fdl.cw(),
            fdl: self.fdr.ccw(),
            fdr: self.fur.cw(),
            fur: self.ful.ccw(),
            ..self
        }
    }
}

pub struct HeuristicCache<StateType: Hash> {
    known_costs: HashMap<StateType, usize>,
}

impl<StateType> HeuristicCache<StateType>
where
    StateType: Hash + Eq + Clone + CanMove,
{
    pub fn from_goal(
        goal_state: StateType,
        free_dirs: &[Direction],
        half_dirs: &[Direction],
    ) -> Self {
        let mut goal_states = HashSet::default();
        goal_states.insert(goal_state);

        Self::from_set(&goal_states, free_dirs, half_dirs)
    }

    pub fn from_set(
        goal_states: &HashSet<StateType>,
        free_dirs: &[Direction],
        half_dirs: &[Direction],
    ) -> Self {
        let mut known_costs = HashMap::default();
        let mut to_process: VecDeque<(StateType, usize)> = VecDeque::new();

        for goal_state in goal_states {
            to_process.push_back((goal_state.clone(), 0));
        }

        while let Some((pos, cost)) = to_process.pop_front() {
            let existing = known_costs.get(&pos);

            if existing.is_some() {
                continue;
            }

            known_costs.insert(pos.clone(), cost);

            for direction in free_dirs.iter().copied() {
                for amt in ALL_AMTS {
                    let fm = FullMove { direction, amt };
                    let next = pos.clone().apply(fm);
                    let next_cost = cost + 1;

                    to_process.push_back((next, next_cost));
                }
            }

            for direction in half_dirs.iter().copied() {
                let amt = Amt::Two;
                let fm = FullMove { direction, amt };
                let next = pos.clone().apply(fm);
                let next_cost = cost + 1;

                to_process.push_back((next, next_cost));
            }
        }

        Self { known_costs }
    }
}

pub trait Heuristic<StateType> {
    fn evaluate(&self, state: &StateType) -> usize;
}

impl<StateType: Hash + Eq + PartialEq> Heuristic<StateType> for HeuristicCache<StateType> {
    fn evaluate(&self, state: &StateType) -> usize {
        if let Some(&cost) = self.known_costs.get(&state) {
            return cost;
        }

        panic!("Should have covered everything really nicely");
    }
}

fn can_follow(last: Option<Direction>, next: Direction) -> bool {
    if last.is_none() {
        return true;
    }

    let last = last.unwrap();

    if last == next {
        false
    } else if last == Direction::F && next == Direction::B {
        false
    } else if last == Direction::R && next == Direction::L {
        false
    } else if last == Direction::U && next == Direction::D {
        false
    } else {
        true
    }
}

pub fn process_dfs<
    StateType: CanMove + Clone,
    IsSolved: Fn(&StateType) -> bool,
    CostHeuristic: Heuristic<StateType>,
>(
    start_state: StateType,
    free_dirs: &[Direction],
    half_move_dirs: &[Direction],
    is_solved: IsSolved,
    cost_heuristic: &CostHeuristic,
    max_fuel: usize,
) -> Vec<FullMove> {
    struct IdaState<'a, IsSolved, CostHeuristic> {
        free_dirs: &'a [Direction],
        half_move_dirs: &'a [Direction],
        is_solved: IsSolved,
        cost_heuristic: &'a CostHeuristic,
    }

    fn ida<
        'a,
        StateType: CanMove + Clone,
        IsSolved: Fn(&StateType) -> bool,
        CostHeuristic: Heuristic<StateType>,
    >(
        ida_state: &mut IdaState<'a, IsSolved, CostHeuristic>,
        cube: &StateType,
        running: &mut Vec<FullMove>,
        max_depth: usize,
    ) -> bool {
        if (ida_state.is_solved)(cube) {
            return true;
        } else if running.len() + (ida_state.cost_heuristic).evaluate(cube) >= max_depth {
            return false;
        }

        for direction in ida_state.half_move_dirs.iter().copied() {
            if !can_follow(running.last().map(|fm| fm.direction), direction) {
                continue;
            }

            let amt = Amt::Two;

            let fm = FullMove { amt, direction };

            let next = cube.clone().apply(fm);

            running.push(fm);

            let found_solution = ida(ida_state, &next, running, max_depth);

            if found_solution {
                return true;
            }

            running.pop();
        }

        for direction in ida_state.free_dirs.iter().copied() {
            if !can_follow(running.last().map(|fm| fm.direction), direction) {
                continue;
            }

            for amt in ALL_AMTS.iter().copied() {
                let fm = FullMove { amt, direction };
                let next = cube.clone().apply(fm);

                running.push(fm);

                let found_solution = ida(ida_state, &next, running, max_depth);

                if found_solution {
                    return true;
                }

                running.pop();
            }
        }

        false
    }

    let mut ida_state = IdaState {
        free_dirs,
        half_move_dirs,
        is_solved,
        cost_heuristic,
    };

    for fuel in 0..=max_fuel {
        let mut running = Vec::new();
        let solved = ida(&mut ida_state, &start_state, &mut running, fuel);

        if solved {
            return running;
        }
    }

    panic!("Couldn't solve it I guess lol")
}

fn parse_facelet(val: i64) -> Facelet {
    match val {
        1 => Facelet::White,
        2 => Facelet::Green,
        3 => Facelet::Red,
        4 => Facelet::Blue,
        5 => Facelet::Orange,
        6 => Facelet::Yellow,
        _ => unreachable!("Invalid facelet value"),
    }
}

pub struct Phase1Cache {
    heuristic_cache: HeuristicCache<EdgeOrientationState>,
}

struct Phase1 {
    cache: Phase1Cache,
}

impl Phase1 {
    fn preprocess() -> Self {
        Self {
            cache: Phase1Cache {
                heuristic_cache: HeuristicCache::from_goal(
                    EdgeOrientationState::make_solved(),
                    &ALL_DIRECTIONS,
                    &[],
                ),
            },
        }
    }

    fn process(&self, cube: &Cube) -> Vec<FullMove> {
        let start_state = EdgeOrientationState::from_cube(cube);

        process_dfs(
            start_state,
            &ALL_DIRECTIONS,
            &[],
            |state| state.is_solved(),
            &self.cache.heuristic_cache,
            8,
        )
    }
}

const FREE_DIRECTIONS: [Direction; 4] = [Direction::B, Direction::F, Direction::L, Direction::R];
const HALF_DIRECTIONS: [Direction; 2] = [Direction::U, Direction::D];

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Phase2State {
    pub edges: EdgeMidSliceState,
    pub corners: CornerOrientationState,
}

impl Phase2State {
    pub fn is_solved(&self) -> bool {
        self.edges.is_solved() && self.corners.is_solved()
    }

    pub fn from_cube(cube: &Cube) -> Phase2State {
        Phase2State {
            edges: EdgeMidSliceState::from_cube(cube),
            corners: CornerOrientationState::from_cube(cube),
        }
    }
}

impl CanMove for Phase2State {
    fn r(self) -> Self {
        Self {
            corners: self.corners.r(),
            edges: self.edges.r(),
        }
    }

    fn l(self) -> Self {
        Self {
            corners: self.corners.l(),
            edges: self.edges.l(),
        }
    }

    fn u(self) -> Self {
        panic!("U not supported")
    }

    fn u_two(self) -> Self {
        Self {
            corners: self.corners.u_two(),
            edges: self.edges.u_two(),
        }
    }

    fn d(self) -> Self {
        panic!("D not supported")
    }

    fn d_two(self) -> Self {
        Self {
            corners: self.corners.d_two(),
            edges: self.edges.d_two(),
        }
    }

    fn b(self) -> Self {
        Self {
            corners: self.corners.b(),
            edges: self.edges.b(),
        }
    }

    fn f(self) -> Self {
        Self {
            corners: self.corners.f(),
            edges: self.edges.f(),
        }
    }
}

pub struct Phase2Cache {
    edge_heuristic: HeuristicCache<EdgeMidSliceState>,
    corner_heuristic: HeuristicCache<CornerOrientationState>,
}

impl Heuristic<Phase2State> for Phase2Cache {
    fn evaluate(&self, state: &Phase2State) -> usize {
        let e = self.edge_heuristic.evaluate(&state.edges);
        let c = self.corner_heuristic.evaluate(&state.corners);

        e.max(c)
    }
}

struct Phase2 {
    cache: Phase2Cache,
}

impl Phase2 {
    fn preprocess() -> Self {
        Self {
            cache: Phase2Cache {
                edge_heuristic: HeuristicCache::from_goal(
                    EdgeMidSliceState::solved(),
                    &FREE_DIRECTIONS,
                    &HALF_DIRECTIONS,
                ),
                corner_heuristic: HeuristicCache::from_goal(
                    CornerOrientationState::solved(),
                    &FREE_DIRECTIONS,
                    &HALF_DIRECTIONS,
                ),
            },
        }
    }

    fn process(&self, cube: &Cube) -> Vec<FullMove> {
        process_dfs(
            Phase2State::from_cube(cube),
            &FREE_DIRECTIONS,
            &HALF_DIRECTIONS,
            |state| state.is_solved(),
            &self.cache,
            11,
        )
    }
}

const PHASE3_FREE_DIRECTIONS: [Direction; 2] = [Direction::L, Direction::R];
const PHASE3_DOUBLE_DIRECTIONS: [Direction; 4] =
    [Direction::U, Direction::D, Direction::F, Direction::B];

pub struct Phase3Cache {
    edges: HashSet<CubeEdgePositions>,
    edge_heuristic: HeuristicCache<CubeEdgePositions>,
    corners: HashSet<CubeCornerPositions>,
    corner_heuristic: HeuristicCache<CubeCornerPositions>,
}

impl Heuristic<CubePositions> for Phase3Cache {
    fn evaluate(&self, state: &CubePositions) -> usize {
        let e = self.edge_heuristic.evaluate(&state.edges);
        let c = self.corner_heuristic.evaluate(&state.corners);

        e.max(c)
    }
}

struct Phase3 {
    cache: Phase3Cache,
}

impl Phase3 {
    fn preprocess() -> Self {
        let start: CubePositions = CubePositions::make_solved();

        let mut full_states: HashSet<CubePositions> = HashSet::default();
        full_states.insert(start.clone());

        let mut to_process = VecDeque::new();
        to_process.push_back(start);

        while let Some(next) = to_process.pop_front() {
            for direction in ALL_DIRECTIONS {
                let fm = FullMove {
                    direction,
                    amt: Amt::Two,
                };
                let applied = next.clone().apply(fm);

                if full_states.insert(applied.clone()) {
                    to_process.push_back(applied);
                }
            }
        }

        let mut edge_states = HashSet::default();
        let mut corner_states = HashSet::default();

        for state in full_states.iter().cloned() {
            edge_states.insert(state.edges);
            corner_states.insert(state.corners);
        }

        let corner_heuristic = HeuristicCache::from_set(
            &corner_states,
            &PHASE3_FREE_DIRECTIONS,
            &PHASE3_DOUBLE_DIRECTIONS,
        );
        let edge_heuristic = HeuristicCache::from_set(
            &edge_states,
            &PHASE3_FREE_DIRECTIONS,
            &PHASE3_DOUBLE_DIRECTIONS,
        );

        Self {
            cache: Phase3Cache {
                edges: edge_states,
                edge_heuristic,
                corners: corner_states,
                corner_heuristic,
            },
        }
    }

    fn process(&self, cube: &Cube) -> Vec<FullMove> {
        process_dfs(
            CubePositions::from_cube(cube),
            &PHASE3_FREE_DIRECTIONS,
            &PHASE3_DOUBLE_DIRECTIONS,
            |state| {
                self.cache.edges.contains(&state.edges)
                    && self.cache.corners.contains(&state.corners)
            },
            &self.cache,
            14,
        )
    }
}

pub struct Phase4Cache {
    corner_heuristic: HeuristicCache<CubeCornerPositions>,
    edge_heuristic: HeuristicCache<CubeEdgePositions>,
}

impl Heuristic<CubePositions> for Phase4Cache {
    fn evaluate(&self, state: &CubePositions) -> usize {
        let e = self.edge_heuristic.evaluate(&state.edges);
        let c = self.corner_heuristic.evaluate(&state.corners);

        e.max(c)
    }
}

struct Phase4 {
    cache: Phase4Cache,
}

impl Phase4 {
    fn preprocess() -> Self {
        Self {
            cache: Phase4Cache {
                corner_heuristic: HeuristicCache::from_goal(
                    CubeCornerPositions::make_solved(),
                    &[],
                    &ALL_DIRECTIONS,
                ),
                edge_heuristic: HeuristicCache::from_goal(
                    CubeEdgePositions::make_solved(),
                    &[],
                    &ALL_DIRECTIONS,
                ),
            },
        }
    }

    fn process(&self, cube: &Cube) -> Vec<FullMove> {
        process_dfs(
            CubePositions::from_cube(cube),
            &[],
            &ALL_DIRECTIONS,
            |state| state.is_solved(),
            &self.cache,
            16,
        )
    }
}

// Reference: https://www.jaapsch.net/puzzles/thistle.htm
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let phase1 = Phase1::preprocess();
    let phase2 = Phase2::preprocess();
    let phase3 = Phase3::preprocess();
    let phase4 = Phase4::preprocess();

    for _ in 0..t {
        let mut face_up = Vec::with_capacity(9);
        let mut face_down = Vec::with_capacity(9);
        let mut face_left = Vec::with_capacity(9);
        let mut face_right = Vec::with_capacity(9);
        let mut face_front = Vec::with_capacity(9);
        let mut face_back = Vec::with_capacity(9);

        for i in 1..=54 {
            if i >= 1 && i <= 9 {
                face_up.push(parse_facelet(scan.token::<i64>()));
            }

            if (i >= 10 && i <= 12) || (i >= 22 && i <= 24) || (i >= 34 && i <= 36) {
                face_left.push(parse_facelet(scan.token::<i64>()));
            }

            if (i >= 13 && i <= 15) || (i >= 25 && i <= 27) || (i >= 37 && i <= 39) {
                face_front.push(parse_facelet(scan.token::<i64>()));
            }

            if (i >= 16 && i <= 18) || (i >= 28 && i <= 30) || (i >= 40 && i <= 42) {
                face_right.push(parse_facelet(scan.token::<i64>()));
            }

            if (i >= 19 && i <= 21) || (i >= 31 && i <= 33) || (i >= 43 && i <= 45) {
                face_back.push(parse_facelet(scan.token::<i64>()));
            }

            if i >= 46 && i <= 54 {
                face_down.push(parse_facelet(scan.token::<i64>()));
            }
        }

        let cube = Cube::new(&vec![
            face_up, face_left, face_front, face_right, face_back, face_down,
        ]);

        let phase1_moves = phase1.process(&cube);
        let phase1_solved = cube.clone().apply_many(&phase1_moves);
        let phase2_moves = phase2.process(&phase1_solved);
        let phase2_solved = phase1_solved.clone().apply_many(&phase2_moves);
        let phase3_moves = phase3.process(&phase2_solved);
        let phase3_solved = phase2_solved.clone().apply_many(&phase3_moves);
        let phase4_moves = phase4.process(&phase3_solved);

        let mut moves = phase1_moves;
        moves.extend(phase2_moves);
        moves.extend(phase3_moves);
        moves.extend(phase4_moves);

        for mv in moves {
            write!(
                out,
                "{}",
                match mv.direction {
                    Direction::U => "U",
                    Direction::D => "D",
                    Direction::L => "L",
                    Direction::R => "R",
                    Direction::F => "F",
                    Direction::B => "B",
                }
            )
            .unwrap();

            write!(
                out,
                "{}",
                match mv.amt {
                    Amt::One => " ",
                    Amt::Two => "2 ",
                    Amt::Rev => "' ",
                }
            )
            .unwrap();
        }

        writeln!(out).unwrap();
    }
}
