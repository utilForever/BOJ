use io::Write;
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    error::Error,
    fmt, io, str,
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
}

mod utils {
    #[inline]
    pub fn add(x: &[f64], y: &[f64]) -> Vec<f64> {
        x.iter().zip(y.iter()).map(|(a, b)| a + b).collect()
    }

    #[inline]
    pub fn sub(x: &[f64], y: &[f64]) -> Vec<f64> {
        x.iter().zip(y.iter()).map(|(a, b)| a - b).collect()
    }

    #[inline]
    pub fn mul(p: &[f64], c: f64) -> Vec<f64> {
        p.iter().map(|a| a * c).collect()
    }

    #[inline]
    pub fn dot(x: &[f64], y: &[f64]) -> f64 {
        x.iter()
            .zip(y.iter())
            .map(|(a, b)| a * b)
            .fold(0.0, |sum, c| sum + c)
    }

    #[inline]
    pub fn cross(x: &[f64], y: &[f64]) -> Vec<f64> {
        match x.len() {
            2 => vec![x[0] * y[1] - x[1] * y[0]],
            3 => vec![
                x[1] * y[2] - x[2] * y[1],
                x[2] * y[0] - x[0] * y[2],
                x[0] * y[1] - x[1] * y[0],
            ],
            _ => unimplemented!("Not implemented for dimension {}", x.len()),
        }
    }

    #[inline]
    pub fn abs(p: &[f64]) -> f64 {
        dot(p, p).sqrt()
    }

    #[inline]
    pub fn normalize(p: &[f64]) -> Vec<f64> {
        let length = abs(p);
        p.iter().map(|a| a / length).collect()
    }

    #[inline]
    pub fn det(mat: &[Vec<f64>]) -> f64 {
        let column_dim = mat[0].len();

        match column_dim {
            1 => mat[0][0],
            2 => mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0],
            3 => {
                mat[0][0] * (mat[1][1] * mat[2][2] - mat[1][2] * mat[2][1])
                    - mat[1][0] * (mat[0][1] * mat[2][2] - mat[0][2] * mat[2][1])
                    + mat[2][0] * (mat[0][1] * mat[1][2] - mat[0][2] * mat[1][1])
            }
            4 => {
                mat[0][0]
                    * (mat[1][1] * mat[2][2] * mat[3][3]
                        + mat[1][2] * mat[2][3] * mat[3][1]
                        + mat[1][3] * mat[2][1] * mat[3][2]
                        - mat[1][3] * mat[2][2] * mat[3][1]
                        - mat[1][2] * mat[2][1] * mat[3][3]
                        - mat[1][1] * mat[2][3] * mat[3][2])
                    - mat[1][0]
                        * (mat[0][1] * mat[2][2] * mat[3][3]
                            + mat[0][2] * mat[2][3] * mat[3][1]
                            + mat[0][3] * mat[2][1] * mat[3][2]
                            - mat[0][3] * mat[2][2] * mat[3][1]
                            - mat[0][2] * mat[2][1] * mat[3][3]
                            - mat[0][1] * mat[2][3] * mat[3][2])
                    + mat[2][0]
                        * (mat[0][1] * mat[1][2] * mat[3][3]
                            + mat[0][2] * mat[1][3] * mat[3][1]
                            + mat[0][3] * mat[1][1] * mat[3][2]
                            - mat[0][3] * mat[1][2] * mat[3][1]
                            - mat[0][2] * mat[1][1] * mat[3][3]
                            - mat[0][1] * mat[1][3] * mat[3][2])
                    - mat[3][0]
                        * (mat[0][1] * mat[1][2] * mat[2][3]
                            + mat[0][2] * mat[1][3] * mat[2][1]
                            + mat[0][3] * mat[1][1] * mat[2][2]
                            - mat[0][3] * mat[1][2] * mat[2][1]
                            - mat[0][2] * mat[1][1] * mat[2][3]
                            - mat[0][1] * mat[1][3] * mat[2][2])
            }
            _ => unimplemented!("Not implemented for dimension {column_dim}"),
        }
    }

    #[inline]
    pub fn det_correlation_matrix(mat: &[Vec<f64>]) -> f64 {
        let dim = mat[0].len();
        let num = mat.len();
        let mut cor = Vec::new();

        for i in 0..num {
            let mut column = Vec::new();

            for j in 0..num {
                let mut c = 0.0;

                for k in 0..dim {
                    c = c + mat[i][k].clone() * mat[j][k].clone();
                }

                column.push(c);
            }

            cor.push(column);
        }

        det(&cor)
    }

    pub fn normal(points_of_facet: &[Vec<f64>]) -> Vec<f64> {
        let num_points = points_of_facet.len();
        let dim = points_of_facet[0].len();
        let mut vectors = Vec::new();

        for i in 1..num_points {
            let vector = sub(&points_of_facet[i], &points_of_facet[i - 1]);
            vectors.push(vector);
        }

        let mut sign = 1.0;
        let mut normal = Vec::new();

        for i in 0..dim {
            let mut mat = Vec::new();

            for vector in vectors.iter() {
                let mut column = Vec::new();

                for (j, element) in vector.iter().enumerate() {
                    if i == j {
                        continue;
                    }

                    column.push(*element);
                }

                mat.push(column);
            }

            let cofactor = det(&mat);
            normal.push(sign * cofactor);
            sign *= -1.0;
        }

        normal
    }

    #[inline]
    pub fn is_same_dimension(points: &[Vec<f64>]) -> bool {
        if points.len() == 0 {
            return true;
        }

        let dim = points[0].len();

        if points.iter().skip(1).find(|p| p.len() != dim).is_some() {
            false
        } else {
            true
        }
    }

    pub fn is_degenerate(points: &[Vec<f64>], threshold: f64) -> bool {
        let dim = points[0].len();
        let ex_vec: Vec<Vec<_>> = points
            .iter()
            .map(|v| {
                let mut v = v.to_vec();
                v.push(1.0);
                v
            })
            .collect();
        let num = ex_vec.len();

        if dim >= num {
            return true;
        }

        let mut mat = Vec::new();

        for i in 0..dim + 1 {
            let mut row = Vec::new();

            for j in 0..dim + 1 {
                let mut c = 0.0;

                for k in 0..num {
                    c = c + ex_vec[k][i].clone() * ex_vec[k][j].clone();
                }

                row.push(c);
            }

            mat.push(row);
        }

        if det(&mat) <= threshold.clone() && det(&mat) >= -threshold.clone() {
            true
        } else {
            false
        }
    }

    pub fn min_max_index_each_axis(points: &[Vec<f64>]) -> Vec<(usize, usize)> {
        let dim = points[0].len();
        let mut min_index = vec![0; dim];
        let mut max_index = vec![0; dim];
        let mut min = vec![0.0; dim];
        let mut max = vec![0.0; dim];

        for (index, point) in points.iter().enumerate() {
            for (j, element) in point.iter().enumerate() {
                if index == 0 || *element < min[j] {
                    min[j] = element.clone();
                    min_index[j] = index;
                }

                if index == 0 || *element > max[j] {
                    max[j] = element.clone();
                    max_index[j] = index;
                }
            }
        }

        min_index.into_iter().zip(max_index.into_iter()).collect()
    }
}

#[derive(Debug, Clone)]
struct Facet {
    indices: Vec<usize>,
    outside_points: Vec<(usize, f64)>,
    neighbor_facets: Vec<usize>,
    normal: Vec<f64>,
    origin: f64,
}

impl Facet {
    fn new(points: &[Vec<f64>], indices: &[usize]) -> Self {
        let points_of_face = indices
            .iter()
            .map(|i| points[*i].to_vec())
            .collect::<Vec<_>>();
        let normal = utils::normal(&points_of_face);
        let origin = utils::dot(&normal, &points_of_face[0]);

        Self {
            indices: indices.to_vec(),
            outside_points: Vec::new(),
            neighbor_facets: Vec::new(),
            normal,
            origin,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Empty,
    LessThanTwoDim,
    Degenerated,
    WrongDimension,
    RoundOffError(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ErrorKind::Empty => write!(f, "Empty"),
            ErrorKind::LessThanTwoDim => write!(f, "Less than two dimention"),
            ErrorKind::Degenerated => write!(f, "Degenerated"),
            ErrorKind::WrongDimension => write!(f, "Wrong dimension"),
            ErrorKind::RoundOffError(msg) => {
                write!(f, "Erroneous results by roundoff error: {msg}")
            }
        }
    }
}

impl Error for ErrorKind {}

#[derive(Debug, Clone)]
struct ConvexHull3 {
    points: Vec<Vec<f64>>,
    facets: BTreeMap<usize, Facet>,
}

impl ConvexHull3 {
    fn try_new(
        points: &[Vec<f64>],
        threshold: f64,
        max_iter: Option<usize>,
    ) -> Result<Self, ErrorKind> {
        let num_points = points.len();

        if num_points == 0 {
            return Err(ErrorKind::Empty);
        }

        let dim = points[0].len();

        if dim < 2 {
            return Err(ErrorKind::LessThanTwoDim);
        }

        if !utils::is_same_dimension(&points) {
            return Err(ErrorKind::WrongDimension);
        }

        if num_points <= dim || utils::is_degenerate(&points, threshold.clone()) {
            return Err(ErrorKind::Degenerated);
        }

        let mut convex_hull = Self::create_simplex(points, threshold.clone())?;
        convex_hull.update(threshold, max_iter)?;
        convex_hull.remove_unused_points();

        if convex_hull.points.len() <= dim {
            return Err(ErrorKind::Degenerated);
        }

        Ok(convex_hull)
    }

    fn create_simplex(points: &[Vec<f64>], threshold: f64) -> Result<Self, ErrorKind> {
        let indices_set = Self::select_vertices_for_simplex(&points, threshold.clone())?;
        let dim = points[0].len();
        let mut facet_add_count = 0;
        let mut facets = BTreeMap::new();

        for idx_facet in 0..dim + 1 {
            let mut facet_indices = Vec::new();

            for j in 0..dim + 1 {
                if j == idx_facet {
                    continue;
                }

                facet_indices.push(indices_set[j]);
            }

            let mut facet = Facet::new(points, &facet_indices);
            let rem_point = indices_set[idx_facet];
            let pos = Self::position_from_facet(points, &facet, rem_point);

            if pos > threshold.clone() {
                facet.indices.swap(0, 1);
                facet.normal = facet.normal.iter().map(|x| -x.clone()).collect();
                facet.origin = -facet.origin;
            }

            if dim != facet.indices.len() {
                return Err(ErrorKind::RoundOffError(
                    "Number of facet's vartices should be dim".to_string(),
                ));
            }

            facets.insert(facet_add_count, facet);
            facet_add_count += 1;
        }

        let simplex_facet_key: Vec<_> = facets.keys().map(|k| *k).collect();

        for (key, facet) in &mut facets.iter_mut() {
            facet
                .outside_points
                .sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

            for neighbors_key in simplex_facet_key
                .iter()
                .filter(|neighbor_key| *neighbor_key != key)
            {
                facet.neighbor_facets.push(*neighbors_key);
            }
        }

        let simplex = Self {
            points: points.to_vec(),
            facets,
        };

        Ok(simplex)
    }

    fn update(&mut self, threshold: f64, max_iter: Option<usize>) -> Result<(), ErrorKind> {
        let dim = self.points[0].len();
        let mut facet_add_count = *self.facets.iter().last().map(|(k, _v)| k).unwrap() + 1;
        let mut num_iter = 0;
        let mut assigned_point_indices = HashSet::new();

        for facet in self.facets.values() {
            for index in &facet.indices {
                assigned_point_indices.insert(*index);
            }
        }

        for (_, facet) in &mut self.facets.iter_mut() {
            for (i, _point) in self.points.iter().enumerate() {
                if assigned_point_indices.contains(&i) {
                    continue;
                }

                let pos = Self::position_from_facet(&self.points, facet, i);

                if pos > threshold.clone() {
                    facet.outside_points.push((i, pos));
                }
            }
        }

        let (max_iter, truncate) = if let Some(iter) = max_iter {
            (iter, true)
        } else {
            (0, false)
        };

        while let Some((key, facet)) = self
            .facets
            .iter()
            .find(|(_, facet)| !facet.outside_points.is_empty())
            .map(|(a, b)| (*a, b.clone()))
        {
            if truncate && num_iter >= max_iter {
                break;
            }

            num_iter += 1;

            let (furthest_point_index, _) = *facet.outside_points.last().unwrap();
            let visible_set = Self::initialize_visible_set(
                &self.points,
                furthest_point_index,
                &self.facets,
                key,
                &facet,
                threshold.clone(),
            );
            let horizon = Self::get_horizon(&visible_set, &self.facets, dim)?;
            let mut new_keys = Vec::new();

            for (ridge, unvisible) in horizon {
                let mut new_facet = vec![furthest_point_index];

                assigned_point_indices.insert(furthest_point_index);

                for point in ridge {
                    new_facet.push(point);
                    assigned_point_indices.insert(point);
                }

                if new_facet.len() != dim {
                    return Err(ErrorKind::RoundOffError(
                        "Number of new facet's vertices should be dim".to_string(),
                    ));
                }

                let mut new_facet = Facet::new(&self.points, &new_facet);
                new_facet.neighbor_facets.push(unvisible);

                let new_key = facet_add_count;

                facet_add_count += 1;
                self.facets.insert(new_key, new_facet);

                let unvisible_faset = self.facets.get_mut(&unvisible).unwrap();

                unvisible_faset.neighbor_facets.push(new_key);
                new_keys.push(new_key);
            }

            if new_keys.len() < dim {
                return Err(ErrorKind::RoundOffError(
                    "Number of new facets should be grater than dim".to_string(),
                ));
            }

            for (i, key_a) in new_keys.iter().enumerate() {
                let points_of_new_facet_a: HashSet<_> = self
                    .facets
                    .get(key_a)
                    .unwrap()
                    .indices
                    .iter()
                    .map(|k| *k)
                    .collect();

                for key_b in new_keys.iter().skip(i + 1) {
                    let points_of_new_facet_b: HashSet<_> = self
                        .facets
                        .get(key_b)
                        .unwrap()
                        .indices
                        .iter()
                        .map(|k| *k)
                        .collect();
                    let num_intersection_points = points_of_new_facet_a
                        .intersection(&points_of_new_facet_b)
                        .collect::<Vec<_>>()
                        .len();

                    if num_intersection_points == dim - 1 {
                        {
                            let facet_a = self.facets.get_mut(key_a).unwrap();
                            facet_a.neighbor_facets.push(*key_b);
                        }

                        let facet_b = self.facets.get_mut(key_b).unwrap();
                        facet_b.neighbor_facets.push(*key_a);
                    }
                }

                let facet_a = self.facets.get(key_a).unwrap();

                if facet_a.neighbor_facets.len() != dim {
                    return Err(ErrorKind::RoundOffError(
                        "Number of neighbors should be dim".to_string(),
                    ));
                }
            }

            for new_key in &new_keys {
                let new_facet = self.facets.get(new_key).unwrap();
                let mut degenerate = true;

                for assigned_point_index in &assigned_point_indices {
                    let position =
                        Self::position_from_facet(&self.points, &new_facet, *assigned_point_index);

                    if position.clone() <= threshold.clone()
                        && position.clone() >= -threshold.clone()
                    {
                        continue;
                    } else if position > 0.0 {
                        let new_facet = self.facets.get_mut(new_key).unwrap();
                        new_facet.indices.swap(0, 1);
                        new_facet.normal = new_facet.normal.iter().map(|x| -x.clone()).collect();
                        new_facet.origin = -new_facet.origin.clone();

                        degenerate = false;
                        break;
                    } else {
                        degenerate = false;
                        break;
                    }
                }

                if degenerate {
                    return Err(ErrorKind::Degenerated);
                }
            }

            let mut visible_facets = Vec::new();

            for visible in &visible_set {
                visible_facets.push(self.facets.get(&visible).unwrap().clone());
            }

            for new_key in &new_keys {
                let new_facet = self.facets.get_mut(&new_key).unwrap();
                let mut checked_point_set = HashSet::new();

                for visible_facet in &visible_facets {
                    for (outside_point_index, _) in visible_facet.outside_points.iter() {
                        if assigned_point_indices.contains(outside_point_index) {
                            continue;
                        }

                        if checked_point_set.contains(outside_point_index) {
                            continue;
                        } else {
                            checked_point_set.insert(outside_point_index);
                        }

                        let pos = Self::position_from_facet(
                            &self.points,
                            new_facet,
                            *outside_point_index,
                        );

                        if pos > threshold.clone() {
                            new_facet.outside_points.push((*outside_point_index, pos));
                        }
                    }
                }

                new_facet
                    .outside_points
                    .sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
            }

            for visible in visible_set {
                let visible_facet = self.facets.get(&visible).unwrap().clone();

                for neighbor_key in visible_facet.neighbor_facets {
                    let neighbor = self.facets.get_mut(&neighbor_key).unwrap();
                    let index = neighbor
                        .neighbor_facets
                        .iter()
                        .enumerate()
                        .find(|(_, k)| **k == visible)
                        .map(|(i, _)| i)
                        .unwrap();

                    neighbor.neighbor_facets.swap_remove(index);
                }

                self.facets.remove(&visible);
            }
        }

        if !self.is_convex(threshold.clone()) {
            return Err(ErrorKind::RoundOffError("Concave".to_string()));
        }

        Ok(())
    }

    pub fn remove_unused_points(&mut self) {
        let mut indices_list = BTreeSet::new();

        for facet in self.facets.values() {
            for i in &facet.indices {
                indices_list.insert(*i);
            }
        }

        let indices_list: BTreeMap<usize, usize> = indices_list
            .into_iter()
            .enumerate()
            .map(|(i, index)| (index, i))
            .collect();

        for facet in self.facets.values_mut() {
            let mut new_facet_indices = Vec::new();

            for i in &facet.indices {
                new_facet_indices.push(*indices_list.get(&i).unwrap());
            }

            std::mem::swap(&mut facet.indices, &mut new_facet_indices);
        }

        let mut vertices = Vec::new();

        for (index, _i) in indices_list.iter() {
            let point = self.points[*index].to_vec();
            vertices.push(point);
        }

        self.points = vertices;
    }

    fn is_convex(&self, threshold: f64) -> bool {
        for (_, facet) in &self.facets {
            let pos = Self::position_from_facet(&self.points, &facet, 0);

            if pos > threshold.clone() {
                return false;
            }
        }

        true
    }

    fn select_vertices_for_simplex(
        points: &[Vec<f64>],
        threshold: f64,
    ) -> Result<Vec<usize>, ErrorKind> {
        let min_max_index_each_axis = utils::min_max_index_each_axis(points);
        let mut vertex_indices_for_simplex = Vec::new();

        for (k, (min_index, max_index)) in min_max_index_each_axis.iter().enumerate() {
            vertex_indices_for_simplex.push(*max_index);

            if k == 0 {
                vertex_indices_for_simplex.push(*min_index);
            }
        }

        if utils::is_degenerate(
            &vertex_indices_for_simplex
                .iter()
                .map(|i| points[*i].to_vec())
                .collect::<Vec<_>>(),
            threshold.clone(),
        ) {
            if let Some(indices) = Self::non_degenerate_indices(points, threshold) {
                vertex_indices_for_simplex = indices;
            } else {
                return Err(ErrorKind::Degenerated);
            }
        }

        if points[0].len() + 1 != vertex_indices_for_simplex.len() {
            return Err(ErrorKind::RoundOffError(
                "number of simplex's vertices should be dim+1".to_string(),
            ));
        }

        Ok(vertex_indices_for_simplex)
    }

    fn initialize_visible_set(
        points: &[Vec<f64>],
        furthest_point_index: usize,
        facets: &BTreeMap<usize, Facet>,
        faset_key: usize,
        facet: &Facet,
        threshold: f64,
    ) -> HashSet<usize> {
        let mut visible_set = HashSet::new();
        visible_set.insert(faset_key);

        let mut neighbor_stack: Vec<_> = facet.neighbor_facets.iter().map(|k| *k).collect();
        let mut visited_neighbor = HashSet::new();

        while let Some(neighbor_key) = neighbor_stack.pop() {
            if visited_neighbor.contains(&neighbor_key) {
                continue;
            } else {
                visited_neighbor.insert(neighbor_key);
            }

            let neighbor = facets.get(&neighbor_key).unwrap();
            let pos = Self::position_from_facet(points, neighbor, furthest_point_index);

            if pos > threshold.clone() {
                visible_set.insert(neighbor_key);
                neighbor_stack.append(&mut neighbor.neighbor_facets.iter().map(|k| *k).collect());
            }
        }

        visible_set
    }

    fn get_horizon(
        visible_set: &HashSet<usize>,
        facets: &BTreeMap<usize, Facet>,
        dim: usize,
    ) -> Result<Vec<(Vec<usize>, usize)>, ErrorKind> {
        let mut horizon = Vec::new();

        for visible_key in visible_set {
            let visible_facet = facets.get(visible_key).unwrap();
            let points_of_visible_facet: HashSet<_> =
                visible_facet.indices.iter().map(|i| *i).collect();

            if dim != points_of_visible_facet.len() {
                return Err(ErrorKind::RoundOffError(
                    "Number of visible facet's vartices should be dim".to_string(),
                ));
            }

            for neighbor_key in &visible_facet.neighbor_facets {
                if !visible_set.contains(neighbor_key) {
                    let unvisible_neighbor = facets.get(neighbor_key).unwrap();
                    let points_of_unvisible_neighbor: HashSet<_> =
                        unvisible_neighbor.indices.iter().map(|i| *i).collect();

                    if dim != points_of_unvisible_neighbor.len() {
                        return Err(ErrorKind::RoundOffError(
                            "Number of unvisible facet's vartices should be dim".to_string(),
                        ));
                    }

                    let horizon_ridge: Vec<_> = points_of_unvisible_neighbor
                        .intersection(&points_of_visible_facet)
                        .map(|key| *key)
                        .collect();

                    if dim - 1 != horizon_ridge.len() {
                        return Err(ErrorKind::RoundOffError(
                            "Number of ridge's vartices should be dim-1".to_string(),
                        ));
                    }

                    horizon.push((horizon_ridge, *neighbor_key));
                }
            }
        }

        if horizon.len() < dim {
            return Err(ErrorKind::RoundOffError("Horizon len < dim".to_string()));
        }

        Ok(horizon)
    }

    fn non_degenerate_indices(vertices: &[Vec<f64>], threshold: f64) -> Option<Vec<usize>> {
        let dim = vertices[0].len();
        let num_points = vertices.len();

        if dim >= num_points {
            return None;
        }

        let mut indices = vec![0];
        let mut axes = Vec::new();

        for i in 1..num_points {
            let vector = utils::sub(&vertices[i], &vertices[0]);
            let sq_norm = utils::dot(&vector, &vector);

            if sq_norm.clone() <= threshold.clone() {
                continue;
            }

            axes.push(vector);

            let det_cor = utils::det_correlation_matrix(&axes);

            if det_cor <= threshold.clone() {
                axes.pop();
                continue;
            }

            indices.push(i);

            if axes.len() == dim {
                return Some(indices);
            }
        }

        None
    }

    fn position_from_facet(points: &[Vec<f64>], facet: &Facet, point_index: usize) -> f64 {
        let origin = facet.origin.clone();
        let pos = utils::dot(&facet.normal, &points[point_index]);

        pos - origin
    }
}

struct Plane {
    normal: Vec<f64>,
    origin: f64,
}

impl Plane {
    fn new(normal: Vec<f64>, origin: f64) -> Self {
        Self { normal, origin }
    }

    fn dist(&self, point: &[f64]) -> f64 {
        utils::dot(&self.normal, point) + self.origin
    }

    fn coord(&self) -> Vec<Vec<f64>> {
        let mut origin = vec![0.0, 0.0, 0.0];

        if self.origin != 0.0 {
            origin = self.intersect_plane(&vec![0.0, 0.0, 0.0], &self.normal);
        }

        let never = vec![2103.0, 1.0, 0.0];
        let xh = utils::cross(&self.normal, &never);
        let yh = utils::cross(&self.normal, &xh);

        vec![utils::normalize(&xh), utils::normalize(&yh), origin]
    }

    fn intersect_all(&self, facets: &BTreeMap<usize, Facet>, points: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let mut ret = Vec::new();

        for point in points.iter() {
            if self.dist(point) == 0.0 {
                ret.push(point.clone());
            }
        }

        for facet in facets.values() {
            let indices = &facet.indices;
            let a = &points[indices[0]];
            let b = &points[indices[1]];
            let c = &points[indices[2]];
            let plane_a = self.dist(a);
            let plane_b = self.dist(b);
            let plane_c = self.dist(c);

            if plane_a == 0.0 {
                ret.push(a.clone());
            }

            if plane_b == 0.0 {
                ret.push(b.clone());
            }

            if plane_c == 0.0 {
                ret.push(c.clone());
            }

            if plane_a * plane_b < 0.0 {
                ret.push(self.intersect_plane(a, &utils::sub(b, a)));
            }

            if plane_b * plane_c < 0.0 {
                ret.push(self.intersect_plane(b, &utils::sub(c, b)));
            }

            if plane_c * plane_a < 0.0 {
                ret.push(self.intersect_plane(c, &utils::sub(a, c)));
            }
        }

        ret.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ret.dedup();

        ret
    }

    fn intersect_plane(&self, facet: &Vec<f64>, direction: &Vec<f64>) -> Vec<f64> {
        let t = -(utils::dot(&self.normal, facet) + self.origin)
            / (utils::dot(&self.normal, direction));
        utils::add(facet, &utils::mul(&direction, t))
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
}

impl Point {
    fn new(x: f64, y: f64, dx: f64, dy: f64) -> Self {
        Self { x, y, dx, dy }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            dx: 0.0,
            dy: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
struct ConvexHull2 {
    points: Vec<Point>,
}

impl ConvexHull2 {
    fn try_new(mut points: Vec<Point>) -> Self {
        points.sort_by(|a, b| {
            if a.y != b.y {
                return a.y.partial_cmp(&b.y).unwrap();
            }

            a.x.partial_cmp(&b.x).unwrap()
        });

        for i in 1..points.len() {
            points[i].dx = points[i].x - points[0].x;
            points[i].dy = points[i].y - points[0].y;
        }

        let first_point = points.remove(0);
        points.sort_by(|a, b| {
            if a.dx * b.dy != a.dy * b.dx {
                return (a.dx * b.dy).partial_cmp(&(a.dy * b.dx)).unwrap().reverse();
            }

            if a.y != b.y {
                return a.y.partial_cmp(&b.y).unwrap();
            }

            a.x.partial_cmp(&b.x).unwrap()
        });
        points.insert(0, first_point);

        let mut stack = Vec::new();
        stack.push(points[0].clone());
        stack.push(points[1].clone());

        for i in 2..points.len() {
            while stack.len() >= 2
                && ConvexHull2::calculate_ccw(
                    stack.last().unwrap().clone(),
                    ConvexHull2::next_to_top(&mut stack),
                    points[i].clone(),
                ) >= 0
            {
                stack.pop();
            }

            stack.push(points[i].clone());
        }

        let mut convex_hull = vec![Point::new(0.0, 0.0, 0.0, 0.0); stack.len()];
        let mut index = stack.len();

        while !stack.is_empty() {
            index -= 1;
            convex_hull[index] = stack.pop().unwrap();
        }

        Self {
            points: convex_hull,
        }
    }

    fn area(&self) -> f64 {
        let mut ret = 0.0;
        let a = self.points[0];

        for i in 1..self.points.len() - 1 {
            let b = self.points[i];
            let c = self.points[i + 1];

            ret += ((a.x * b.y + b.x * c.y + c.x * a.y) - (a.y * b.x + b.y * c.x + c.y * a.x))
                .abs()
                / 2.0;
        }

        ret
    }

    fn calculate_ccw(p1: Point, p2: Point, p3: Point) -> i64 {
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);
        let (x3, y3) = (p3.x, p3.y);
        let ret = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);

        if ret > 0.0 {
            1
        } else if ret < 0.0 {
            -1
        } else {
            0
        }
    }

    fn next_to_top(stack: &mut Vec<Point>) -> Point {
        let top = stack.pop().unwrap();
        let next = stack.pop().unwrap();

        stack.push(next.clone());
        stack.push(top);

        next
    }
}

// Thanks for @seungwuk98 to help some parts of code! (Plane-related)
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut points = vec![Vec::new(); n];

    for i in 0..n {
        points[i] = vec![
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        ];
    }

    let convex_hull = ConvexHull3::try_new(&points, 0.0, None);

    let convert = |points: &Vec<Vec<f64>>, plane: &Plane| -> Vec<Vec<f64>> {
        let coords = plane.coord();
        let (xh, yh, origin) = (&coords[0], &coords[1], &coords[2]);
        let mut ret = Vec::new();

        for point in points {
            let x = utils::dot(&xh, &utils::sub(point, &origin));
            let y = utils::dot(&yh, &utils::sub(point, &origin));

            ret.push(vec![x, y]);
        }

        ret
    };

    for _ in 0..q {
        let (a, b, c, d) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );

        if convex_hull.is_err() {
            writeln!(out, "0").unwrap();
            continue;
        }

        let convex_hull = convex_hull.as_ref().unwrap();

        let plane = Plane::new(vec![a, b, c], d);
        let intersections = plane.intersect_all(&convex_hull.facets, &convex_hull.points);

        if intersections.len() < 3 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let points = convert(&intersections, &plane);
        let points = points
            .iter()
            .map(|point| Point::new(point[0], point[1], 0.0, 0.0))
            .collect::<Vec<_>>();
        let convex_hull = ConvexHull2::try_new(points.clone());
        let ret = convex_hull.area();

        writeln!(out, "{:.8}", ret).unwrap();
    }
}
