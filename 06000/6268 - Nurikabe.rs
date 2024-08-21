use io::Write;
use std::{
    collections::{HashSet, VecDeque},
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Coord {
    y: usize,
    x: usize,
}

impl Coord {
    pub fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum State {
    White,
    Black,
    Numbered(usize),
}

impl State {
    fn is_numbered(self) -> bool {
        matches!(self, Self::Numbered(_))
    }

    fn opposite(self) -> Self {
        match self {
            Self::White | Self::Numbered(_) => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Cell {
    state: Option<State>,
    region: Option<RegionID>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: None,
            region: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct RegionID(usize);

impl RegionID {
    pub fn to_index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
struct Region {
    state: State,
    coords: Vec<Coord>,
    unknowns: Vec<Coord>,
}

impl Region {
    fn len(&self) -> usize {
        self.coords.len()
    }

    fn unknowns_len(&self) -> usize {
        self.unknowns.len()
    }

    fn is_closed(&self) -> bool {
        self.unknowns.is_empty()
    }
}

struct MarkSet {
    mark_as_black: HashSet<Coord>,
    mark_as_white: HashSet<Coord>,
}

impl MarkSet {
    fn new() -> Self {
        Self {
            mark_as_black: HashSet::new(),
            mark_as_white: HashSet::new(),
        }
    }

    fn insert(&mut self, coord: Coord, state: State) -> bool {
        match state {
            State::White | State::Numbered(_) => &mut self.mark_as_white,
            State::Black => &mut self.mark_as_black,
        }
        .insert(coord)
    }

    fn apply(self, solver: &mut NurikabeSolver) -> Result<bool, SolverError> {
        let ret = !self.mark_as_white.is_empty() || !self.mark_as_black.is_empty();

        for coord in self.mark_as_black {
            solver.mark_cell(coord, State::Black)?;
        }

        for coord in self.mark_as_white {
            solver.mark_cell(coord, State::White)?;
        }

        Ok(ret)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverError {
    Contradiction,
    NoStrategyApplies,
}

#[derive(Debug, Clone)]
struct NurikabeSolver {
    height: usize,
    width: usize,
    cells: Box<[Cell]>,
    regions: Vec<Option<Region>>,
    available_region_ids: Vec<RegionID>,
    total_black_cells: usize,
}

impl NurikabeSolver {
    fn new(height: usize, width: usize, puzzle: Vec<Vec<char>>) -> Self {
        let mut solver = NurikabeSolver {
            height,
            width,
            cells: vec![Default::default(); height * width].into_boxed_slice(),
            regions: Vec::new(),
            available_region_ids: Vec::new(),
            total_black_cells: height * width,
        };

        for i in 0..height {
            for j in 0..width {
                let state = match puzzle[i][j] {
                    '.' => State::White,
                    '#' => State::Black,
                    '1'..='9' => State::Numbered(puzzle[i][j].to_digit(10).unwrap() as usize),
                    _ => unreachable!(),
                };

                if let State::Numbered(n) = state {
                    let coord = Coord { y: i, x: j };
                    let region_id = solver.add_region(Region {
                        state,
                        coords: vec![coord],
                        unknowns: solver.valid_neighbors(coord).collect(),
                    });

                    *solver.cell_mut(coord) = Cell {
                        state: Some(state),
                        region: Some(region_id),
                    };

                    solver.total_black_cells -= n;
                }
            }
        }

        solver
    }

    fn coord_to_index(&self, coord: Coord) -> usize {
        coord.y * self.width + coord.x
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        Coord::new(index / self.width, index % self.width)
    }

    fn valid_neighbors(&self, coord: Coord) -> impl Iterator<Item = Coord> {
        let height = self.height as isize;
        let width = self.width as isize;
        let neighbors = [
            (coord.y as isize - 1, coord.x as isize),
            (coord.y as isize + 1, coord.x as isize),
            (coord.y as isize, coord.x as isize - 1),
            (coord.y as isize, coord.x as isize + 1),
        ];

        neighbors.into_iter().filter_map(move |(y, x)| {
            if y >= 0 && y < height && x >= 0 && x < width {
                Some(Coord::new(y as usize, x as usize))
            } else {
                None
            }
        })
    }

    fn valid_unknown_neighbors(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        self.valid_neighbors(coord)
            .filter(move |&coord| self.cell(coord).state == None)
    }

    fn cell(&self, coord: Coord) -> &Cell {
        &self.cells[self.coord_to_index(coord)]
    }

    fn cell_mut(&mut self, coord: Coord) -> &mut Cell {
        &mut self.cells[self.coord_to_index(coord)]
    }

    fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }

    fn iter(&self) -> impl Iterator<Item = (Coord, &Cell)> {
        self.cells()
            .enumerate()
            .map(move |(index, cell)| (self.index_to_coord(index), cell))
    }

    fn region(&self, region_id: RegionID) -> Option<&Region> {
        self.regions[region_id.to_index()].as_ref()
    }

    fn region_mut(&mut self, region_id: RegionID) -> Option<&mut Region> {
        self.regions[region_id.to_index()].as_mut()
    }

    fn regions(&self) -> impl Iterator<Item = &Region> {
        self.regions.iter().filter_map(Option::as_ref)
    }

    fn regions_iter(&self) -> impl Iterator<Item = (RegionID, &Region)> {
        self.regions
            .iter()
            .enumerate()
            .filter_map(|(index, region)| region.as_ref().map(|region| (RegionID(index), region)))
    }

    fn add_region(&mut self, region: Region) -> RegionID {
        if let Some(region_id) = self.available_region_ids.pop() {
            self.regions[region_id.to_index()] = Some(region);
            region_id
        } else {
            let region_id = RegionID(self.regions.len());
            self.regions.push(Some(region));
            region_id
        }
    }

    fn remove_region(&mut self, region_id: RegionID) -> Option<Region> {
        self.available_region_ids.push(region_id);
        self.regions[region_id.to_index()].take()
    }

    fn mark_cell(&mut self, coord: Coord, state: State) -> Result<(), SolverError> {
        if self.cell(coord).state.is_some() {
            return Err(SolverError::Contradiction);
        }

        let mut region_id = self.add_region(Region {
            state,
            coords: vec![coord],
            unknowns: self.valid_unknown_neighbors(coord).collect(),
        });

        self.cell_mut(coord).state = Some(state);
        self.cell_mut(coord).region = Some(region_id);

        for adjacent_coord in self.valid_neighbors(coord) {
            if let Some(adjacent_region_id) = self.cell(adjacent_coord).region {
                let adjacent_region = self.region_mut(adjacent_region_id).unwrap();

                adjacent_region.unknowns.retain(|&unknown| unknown != coord);

                let is_adjacent_state_equivalent = match adjacent_region.state {
                    State::White | State::Numbered(_) => state == State::White,
                    State::Black => state == State::Black,
                };

                if is_adjacent_state_equivalent {
                    region_id = self.fuse_regions(adjacent_region_id, region_id)?;
                }
            }
        }

        Ok(())
    }

    fn fuse_regions(
        &mut self,
        region_id1: RegionID,
        region_id2: RegionID,
    ) -> Result<RegionID, SolverError> {
        if region_id1 == region_id2 {
            return Ok(region_id1);
        }

        let region1 = self.region(region_id1).unwrap();
        let region2 = self.region(region_id2).unwrap();

        match (region1.state, region2.state) {
            (State::Numbered(_), State::Numbered(_)) => {
                return Err(SolverError::Contradiction);
            }
            (_, State::Numbered(_)) => {
                return self.fuse_regions(region_id2, region_id1);
            }
            (State::Numbered(number), State::White) => {
                if region1.len() + region2.len() > number {
                    return Err(SolverError::Contradiction);
                }
            }
            (State::White, State::White) | (State::Black, State::Black) => {
                // Do nothing
            }
            _ => {
                return Err(SolverError::Contradiction);
            }
        }

        let region2 = self.remove_region(region_id2).unwrap();
        let region1 = self.region_mut(region_id1).unwrap();

        for coord in region2.unknowns {
            if !region1.unknowns.contains(&coord) {
                region1.unknowns.push(coord);
            }
        }

        region1.coords.extend(&region2.coords);

        for coord in region2.coords {
            self.cell_mut(coord).region = Some(region_id1);
        }

        Ok(region_id1)
    }

    fn is_complete(&self) -> bool {
        let total_cells = self.height * self.width;
        let marked_cells = self.regions().map(|region| region.len()).sum::<usize>();

        total_cells == marked_cells
    }

    fn is_cell_unreachable(
        &self,
        coord: Coord,
        assume_black: impl IntoIterator<Item = Coord>,
    ) -> bool {
        if self.cell(coord).state != None {
            return false;
        }

        let max_white_region_len = self
            .regions()
            .filter_map(|region| {
                if let State::Numbered(max_region_len) = region.state {
                    Some(max_region_len.saturating_sub(region.len()))
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);

        let mut explored = HashSet::from([coord]);
        explored.extend(assume_black);

        let mut queue = VecDeque::from([(coord, 1)]);

        while let Some((cur_coord, depth)) = queue.pop_front() {
            let mut adj_numbered_regions = HashSet::new();
            let mut adj_white_regions = HashSet::new();

            for adj_coord in self.valid_neighbors(cur_coord) {
                if let Some(adj_region_id) = self.cell(adj_coord).region {
                    match self.region(adj_region_id).unwrap().state {
                        State::Numbered(_) => {
                            adj_numbered_regions.insert(adj_region_id);
                        }
                        State::White => {
                            adj_white_regions.insert(adj_region_id);
                        }
                        State::Black => {}
                    };
                }
            }

            if adj_numbered_regions.len() >= 2 {
                continue;
            }

            let mut extra_region_len = depth;

            for &region_id in &adj_white_regions {
                extra_region_len += self.region(region_id).unwrap().len();
            }

            if !adj_numbered_regions.is_empty() {
                let region_id = *adj_numbered_regions.iter().next().unwrap();
                let region = self.region(region_id).unwrap();

                if let State::Numbered(max_region_len) = region.state {
                    if extra_region_len + region.len() <= max_region_len {
                        return false;
                    } else {
                        continue;
                    }
                } else {
                    unreachable!();
                }
            }

            if !adj_white_regions.is_empty() {
                if extra_region_len + 1 <= max_white_region_len {
                    return false;
                } else {
                    continue;
                }
            }

            for adj_coord in self.valid_unknown_neighbors(cur_coord) {
                if !explored.contains(&adj_coord) {
                    explored.insert(adj_coord);
                    queue.push_back((adj_coord, depth + 1));
                }
            }
        }

        true
    }

    fn is_region_confined(
        &self,
        region_id: RegionID,
        assume_visited: impl IntoIterator<Item = Coord>,
    ) -> Result<bool, SolverError> {
        let region = self.region(region_id).unwrap();
        let mut open = VecDeque::from_iter(region.unknowns.iter().copied());

        let mut visited = HashSet::new();
        visited.extend(region.coords.iter().copied());
        visited.extend(assume_visited);

        let mut closed = HashSet::new();
        closed.extend(region.coords.iter().copied());

        while let Some(coord) = open.pop_front() {
            if !visited.insert(coord) {
                continue;
            }

            if !self.is_region_like_incomplete(region.state, closed.len()) {
                return Ok(false);
            }

            let other_region = self
                .cell(coord)
                .region
                .and_then(|region_id| self.region(region_id));

            match region.state {
                State::Numbered(_) => match other_region.map(|region| region.state) {
                    Some(State::Numbered(_)) => {
                        return Err(SolverError::Contradiction);
                    }
                    Some(State::White) => {
                        // Do nothing
                    }
                    Some(State::Black) => {
                        continue;
                    }
                    None => {
                        if self
                            .valid_neighbors(coord)
                            .filter_map(|adj_coord| self.cell(adj_coord).region)
                            .filter(|adj_region_id| {
                                self.region(*adj_region_id)
                                    .map(|adj_region| adj_region.state.is_numbered())
                                    .unwrap_or(false)
                            })
                            .any(|adj_region_id| region_id != adj_region_id)
                        {
                            continue;
                        }
                    }
                },
                State::White => match other_region.map(|region| region.state) {
                    Some(State::Numbered(_)) => {
                        return Ok(false);
                    }
                    Some(State::White) | None => {
                        // Do nothing
                    }
                    Some(State::Black) => {
                        continue;
                    }
                },
                State::Black => match other_region.map(|region| region.state) {
                    Some(State::Numbered(_) | State::White) => {
                        continue;
                    }
                    Some(State::Black) | None => {
                        // Do nothing
                    }
                },
            }

            if let Some(other_region) = other_region {
                closed.extend(other_region.coords.iter().copied());
                visited.extend(other_region.coords.iter().copied());
                open.extend(other_region.unknowns.iter().copied());
            } else {
                closed.insert(coord);
                visited.insert(coord);
                open.extend(self.valid_neighbors(coord));
            }
        }

        Ok(self.is_region_like_incomplete(region.state, closed.len()))
    }

    fn is_region_incomplete(&self, region: &Region) -> bool {
        self.is_region_like_incomplete(region.state, region.len())
    }

    fn is_region_like_incomplete(&self, region_state: State, region_len: usize) -> bool {
        match region_state {
            State::White => true,
            State::Black => region_len < self.total_black_cells,
            State::Numbered(number) => region_len < number,
        }
    }

    fn is_region_overfilled(&self, region: &Region) -> bool {
        match region.state {
            State::White => false,
            State::Black => region.len() > self.total_black_cells,
            State::Numbered(number) => region.len() > number,
        }
    }

    fn detect_contradictions(&self) -> Result<(), SolverError> {
        for region in self.regions() {
            if region.is_closed() && self.is_region_incomplete(region) {
                return Err(SolverError::Contradiction);
            }

            if self.is_region_overfilled(region) {
                return Err(SolverError::Contradiction);
            }
        }

        Ok(())
    }

    fn solve(&mut self, is_hypothetical: bool) -> Result<(), SolverError> {
        while !self.is_complete() {
            if self.analyze_complete_islands()? {
                continue;
            }

            if self.analyze_single_liberties()? {
                continue;
            }

            if self.analyze_dual_liberties()? {
                continue;
            }

            if self.analyze_potential_pools()? {
                continue;
            }

            if self.analyze_unreachable_cells()? {
                continue;
            }

            if self.analyze_confinement()? {
                continue;
            }

            if is_hypothetical && self.analyze_hypotheticals()? {
                continue;
            }

            if self.detect_contradictions().is_err() {
                return Err(SolverError::Contradiction);
            }

            if self.guess_and_backtrack().is_err() {
                return Err(SolverError::NoStrategyApplies);
            }
        }

        self.detect_contradictions()
    }

    fn analyze_complete_islands(&mut self) -> Result<bool, SolverError> {
        let mut mark_set = MarkSet::new();

        for region in self.regions() {
            if region.state.is_numbered() && !self.is_region_incomplete(region) {
                mark_set.mark_as_black.extend(region.unknowns.iter());
            }
        }

        mark_set.apply(self)
    }

    fn analyze_single_liberties(&mut self) -> Result<bool, SolverError> {
        let mut mark_set = MarkSet::new();

        for region in self.regions() {
            if self.is_region_incomplete(region) && region.unknowns_len() == 1 {
                mark_set.insert(region.unknowns[0], region.state);
            }
        }

        mark_set.apply(self)
    }

    fn analyze_dual_liberties(&mut self) -> Result<bool, SolverError> {
        let mut mark_set = MarkSet::new();

        for region in self.regions() {
            if let State::Numbered(number) = region.state {
                if region.len() + 1 == number && region.unknowns_len() == 2 {
                    let adj1 = self.valid_unknown_neighbors(region.unknowns[0]);
                    let adj2 = self
                        .valid_unknown_neighbors(region.unknowns[1])
                        .collect::<Vec<_>>();

                    for coord in adj1 {
                        if adj2.contains(&coord) {
                            mark_set.insert(coord, State::Black);
                            break;
                        }
                    }
                }
            }
        }

        mark_set.apply(self)
    }

    fn analyze_unreachable_cells(&mut self) -> Result<bool, SolverError> {
        let mut mark_set = MarkSet::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let coord = Coord::new(y, x);
                if self.is_cell_unreachable(coord, mark_set.mark_as_black.iter().copied()) {
                    mark_set.insert(coord, State::Black);
                }
            }
        }

        mark_set.apply(self)
    }

    fn analyze_potential_pools(&mut self) -> Result<bool, SolverError> {
        let mut mark_set = MarkSet::new();

        for x in 1..self.width {
            for y in 1..self.height {
                let mut cells = [
                    Coord::new(y - 1, x - 1),
                    Coord::new(y - 1, x),
                    Coord::new(y, x - 1),
                    Coord::new(y, x),
                ]
                .map(|c| (c, self.cell(c).state));

                assert!(Some(State::Black) > None);
                cells.sort_unstable_by_key(|(_, state)| *state);

                match cells {
                    [(coord, None), (_, Some(State::Black)), (_, Some(State::Black)), (_, Some(State::Black))] =>
                    {
                        mark_set.insert(coord, State::White);
                    }
                    [(coord1, None), (coord2, None), (_, Some(State::Black)), (_, Some(State::Black))] => {
                        if self.is_cell_unreachable(coord1, [coord2]) {
                            mark_set.insert(coord2, State::White);
                        } else if self.is_cell_unreachable(coord2, [coord1]) {
                            mark_set.insert(coord1, State::White);
                        }
                    }
                    [(_, Some(State::Black)), (_, Some(State::Black)), (_, Some(State::Black)), (_, Some(State::Black))] =>
                    {
                        return Err(SolverError::Contradiction);
                    }
                    _ => {}
                }
            }
        }

        mark_set.apply(self)
    }

    fn analyze_confinement(&mut self) -> Result<bool, SolverError> {
        let mut mark_set = MarkSet::new();

        self.iter()
            .filter(|(_, cell)| cell.state.is_none())
            .try_for_each(|(coord, _)| {
                self.regions_iter().try_for_each(|(region_id, region)| {
                    if self.is_region_confined(region_id, [coord])? {
                        mark_set.insert(coord, region.state);
                    }

                    Ok(())
                })
            })?;

        self.regions_iter()
            .filter(|(_, region)| matches!(region.state, State::Numbered(number) if region.len() < number))
            .try_for_each(|(region_id, region)| {
                region.unknowns.iter().try_for_each(|&coord| {
                    let mut assume_visited = vec![coord];
                    assume_visited.extend(self.valid_unknown_neighbors(coord));

                    self.valid_neighbors(coord)
                        .map(|coord| self.cell(coord))
                        .filter(|cell| matches!(cell.state, Some(State::White)))
                        .for_each(|cell| {
                            let region = self.region(cell.region.unwrap()).unwrap();
                            assume_visited.extend(region.unknowns.iter().copied());
                        });

                        self.regions_iter()
                        .filter(|(other_region_id, _)| *other_region_id != region_id)
                        .filter(|(_, other_region)| other_region.state.is_numbered())
                        .try_for_each(|(other_region_id, _)| {
                            if self.is_region_confined(other_region_id, assume_visited.iter().copied())? {
                                mark_set.insert(coord, State::Black);
                            }

                            Ok(())
                        })
                })
            })?;

        mark_set.apply(self)
    }

    fn analyze_hypotheticals(&mut self) -> Result<bool, SolverError> {
        let ret = self
            .iter()
            .filter(|(_, cell)| cell.state.is_none())
            .flat_map(|(coord, _)| {
                [State::Black, State::White]
                    .iter()
                    .map(move |state| (coord, *state))
            })
            .find_map(|(coord, state)| {
                let mut hypothetical_solver = self.clone();
                let result = hypothetical_solver
                    .mark_cell(coord, state)
                    .and_then(|_| hypothetical_solver.solve(false));
                match result {
                    Ok(_) => Some((coord, state)),
                    Err(SolverError::Contradiction) => Some((coord, state.opposite())),
                    Err(SolverError::NoStrategyApplies) => None,
                }
            });

        match ret {
            Some((coord, state)) => {
                self.mark_cell(coord, state).unwrap();
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn guess_and_backtrack(&mut self) -> Result<(), SolverError> {
        for (coord, cell) in self.clone().iter() {
            if cell.state.is_none() {
                let mut solver_white = self.clone();

                if solver_white.mark_cell(coord, State::White).is_ok() {
                    if solver_white.solve(false).is_ok() {
                        *self = solver_white;
                        return Ok(());
                    }
                }

                let mut solver_black = self.clone();

                if solver_black.mark_cell(coord, State::Black).is_ok() {
                    if solver_black.solve(false).is_ok() {
                        *self = solver_black;
                        return Ok(());
                    }
                }

                return Err(SolverError::Contradiction);
            }
        }

        Err(SolverError::NoStrategyApplies)
    }
}

// Reference: https://github.com/microsoft/nurikabe
// Reference: https://github.com/Mesoptier/nurikabe
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut t = 1;

    loop {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

        if n == 0 && m == 0 {
            break;
        }

        if t != 1 {
            writeln!(out).unwrap();
        }

        let mut puzzle = vec![vec![' '; m]; n];

        for i in 0..n {
            let line = scan.token::<String>();

            for (j, c) in line.chars().enumerate() {
                puzzle[i][j] = c;
            }
        }

        let mut nurikabe = NurikabeSolver::new(n, m, puzzle);
        let result = nurikabe.solve(true);

        if let Ok(_) = result {
            for i in 0..n {
                for j in 0..m {
                    write!(
                        out,
                        "{}",
                        if let Some(state) = nurikabe.cell(Coord::new(i, j)).state {
                            match state {
                                State::Black => '#',
                                State::White => '.',
                                State::Numbered(n) => n.to_string().chars().next().unwrap(),
                            }
                        } else {
                            unreachable!()
                        }
                    )
                    .unwrap();
                }

                writeln!(out).unwrap();
            }
        }

        t += 1;
    }
}
