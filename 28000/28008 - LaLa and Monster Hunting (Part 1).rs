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

extern "C" {
    fn rand() -> u32;
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }

    fn set_point(&mut self, px: f64, py: f64) {
        self.x = px;
        self.y = py;
    }

    fn set_x(&mut self, px: f64) {
        self.x = px;
    }

    fn set_y(&mut self, py: f64) {
        self.y = py;
    }

    fn magnitude(&self) -> f64 {
        self.x.hypot(self.y)
    }

    fn get_unit_vector(&self) -> Point {
        let magnitude = self.magnitude();

        Point {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }

    fn angle_from_vec1_to_vec2(&self, other: Point) -> f64 {
        let vec1 = self.get_unit_vector();
        let vec2 = other.get_unit_vector();
        let point_to_point = Point::new(vec1 - vec2);
        let length = point_to_point.magnitude();
        let cosine = ((2.0 - length * length) / 2.0).clamp(-1.0, 1.0);

        if vec1 * vec2 > 0.0 {
            cosine.acos()
        } else {
            2.0 * std::f64::consts::PI - cosine.acos()
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul for Point {
    type Output = f64;

    fn mul(self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn get_length(&self) -> f64 {
        let length = Point::new(self.end - self.start);

        length.magnitude()
    }

    fn signed_distance(&self, point: Point) -> f64 {
        let point_vector = Point::new(point - self.start);
        let line_vector = Point::new(self.end - self.start);

        line_vector * point_vector / line_vector.magnitude()
    }

    fn evaluate_vector(&self) -> Point {
        self.end - self.start
    }

    fn get_normal_vector(&self) -> Point {
        let dir_vector = self.evaluate_vector();

        Point {
            x: -dir_vector.y,
            y: dir_vector.x,
        }
    }

    fn make_perpendicular_line(&self, passing_point: &Point) -> Line {
        let dir_vector = self.get_normal_vector();

        Line::new(*passing_point, *passing_point + dir_vector)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Disk {
    center: Point,
    radius: f64,
}

impl Disk {
    fn contain(&self, disk: &Disk, tolerance: f64) -> bool {
        let center_to_center = Point::new(disk.center - self.center);
        let distance = center_to_center.magnitude();

        self.radius >= (disk.radius + distance) - tolerance
    }
}

#[derive(Debug)]
struct Arc {
    disk: Disk,
    start: Point,
    end: Point,
}

impl Arc {
    fn new(disk: Disk) -> Self {
        Self {
            disk,
            start: Point {
                x: f64::MAX,
                y: f64::MAX,
            },
            end: Point {
                x: f64::MAX,
                y: f64::MAX,
            },
        }
    }

    fn new_with_points(disk: Disk, start: Point, end: Point) -> Self {
        Self { disk, start, end }
    }

    fn angle_btw_start_and_end_pts(&self) -> f64 {
        if self.start.x == f64::MAX || self.start.y == f64::MAX {
            2.0 * std::f64::consts::PI
        } else if self.start == self.end {
            0.0
        } else {
            let center = self.disk.center;
            let vec_center_to_start = self.start - center;
            let vec_center_to_end = self.end - center;

            vec_center_to_start.angle_from_vec1_to_vec2(vec_center_to_end)
        }
    }

    fn calculate_arc_length(&self) -> f64 {
        self.angle_btw_start_and_end_pts() * self.disk.radius
    }
}

#[derive(Default)]
struct LineImplicitEquation {
    coefficient: [f64; 3],
}

impl LineImplicitEquation {
    fn evaluate_implicit_equation(&self, value_for_x: f64, value_for_y: f64) -> f64 {
        self.coefficient[0] * value_for_x + self.coefficient[1] * value_for_y + self.coefficient[2]
    }
}

#[derive(Debug)]
struct InputForFindHull {
    disks: Vec<usize>,
    pre_apex_disk: usize,
    post_apex_disk: usize,
    hull_point_of_pre_apex_disk: Point,
    hull_point_of_post_apex_disk: Point,
}

#[derive(Debug)]
enum SliverConfiguration {
    SliverCaseA,
    SliverCaseB,
    SliverCaseC1,
    SliverCaseC2,
}

#[derive(Default)]
struct QuickhullDisk {
    disks: Vec<Disk>,
}

impl QuickhullDisk {
    pub fn new(disks: Vec<Disk>) -> Self {
        Self { disks }
    }

    pub fn find_hull_disks(&mut self) -> Vec<usize> {
        if self.disks.len() == 1 {
            return vec![0];
        }

        if self
            .disks
            .iter()
            .all(|disk| disk.radius == self.disks[0].radius)
        {
            let mut ret = self.merge(0, self.disks.len() - 1);
            ret.push(ret[0]);

            return ret;
        }

        let mut high_left_extreme_point_p = Point::default();
        let mut low_right_extreme_point_q = Point::default();
        let mut disk_having_high_left_point_p = 0;
        let mut disk_having_low_right_point_q = 0;

        self.find_high_left_n_low_right_extreme_points_and_their_disks_to_divide_disk_set(
            &mut high_left_extreme_point_p,
            &mut low_right_extreme_point_q,
            &mut disk_having_high_left_point_p,
            &mut disk_having_low_right_point_q,
        );

        let mut initial_expanded_non_positive_disks_d_right = Vec::new();
        let mut initial_expanded_non_negative_disks_d_left = Vec::new();
        let mut oriented_line_segment_base_line_pq =
            Line::new(high_left_extreme_point_p, low_right_extreme_point_q);

        self.divide_input_disks_into_two_initial_subsets(
            &mut oriented_line_segment_base_line_pq,
            &mut initial_expanded_non_positive_disks_d_right,
            &mut initial_expanded_non_negative_disks_d_left,
        );

        let mut stack_for_finding_hull = Vec::new();

        self.prepare_and_insert_input_data_for_finding_hull_to_stack(
            &initial_expanded_non_negative_disks_d_left,
            &mut disk_having_low_right_point_q,
            &mut disk_having_high_left_point_p,
            &mut low_right_extreme_point_q,
            &mut high_left_extreme_point_p,
            &mut stack_for_finding_hull,
        );
        self.prepare_and_insert_input_data_for_finding_hull_to_stack(
            &initial_expanded_non_positive_disks_d_right,
            &mut disk_having_high_left_point_p,
            &mut disk_having_low_right_point_q,
            &mut high_left_extreme_point_p,
            &mut low_right_extreme_point_q,
            &mut stack_for_finding_hull,
        );

        let mut hull_disks = Vec::new();

        self.find_hull_disks_by_iteration(&mut stack_for_finding_hull, &mut hull_disks);

        hull_disks
    }

    fn merge(&mut self, left: usize, right: usize) -> Vec<usize> {
        if left == right {
            return vec![left];
        }

        let mid = (left + right) / 2;
        let p = self.merge(left, mid);
        let q = self.merge(mid + 1, right);

        self.merge_internal(p, q)
    }

    fn merge_internal(&mut self, p: Vec<usize>, q: Vec<usize>) -> Vec<usize> {
        let mut values = Vec::new();

        for val in p {
            values.push(((self.disks[val].center.x, self.disks[val].center.y), val));
        }

        for val in q {
            values.push(((self.disks[val].center.x, self.disks[val].center.y), val));
        }

        values.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut h1 = Vec::new();
        let mut h2 = Vec::new();
        let mut ret = Vec::new();

        let ccw = |a: (f64, f64), b: (f64, f64), c: (f64, f64)| {
            let val = (a.0 * b.1 + b.0 * c.1 + c.0 * a.1) - (a.1 * b.0 + b.1 * c.0 + c.1 * a.0);

            if val == 0.0 {
                0
            } else if val > 0.0 {
                1
            } else {
                -1
            }
        };

        if values.len() <= 2 {
            for val in values {
                ret.push(val.1);
            }
        } else {
            h1.push(values[0]);
            h1.push(values[1]);

            for i in 2..values.len() {
                loop {
                    let len_h1 = h1.len();

                    if len_h1 < 2 || ccw(h1[len_h1 - 2].0, h1[len_h1 - 1].0, values[i].0) > 0 {
                        break;
                    }

                    h1.pop();
                }

                h1.push(values[i]);
            }

            h2.push(values[values.len() - 1]);
            h2.push(values[values.len() - 2]);

            for i in (0..=values.len() - 3).rev() {
                loop {
                    let len_h2 = h2.len();

                    if len_h2 < 2 || ccw(h2[len_h2 - 2].0, h2[len_h2 - 1].0, values[i].0) > 0 {
                        break;
                    }

                    h2.pop();
                }

                h2.push(values[i]);
            }

            for val in h1 {
                ret.push(val.1);
            }

            for val in h2 {
                if *ret.last().unwrap() == val.1 {
                    continue;
                }

                ret.push(val.1);
            }

            ret.pop();
        }

        ret
    }

    fn find_high_left_n_low_right_extreme_points_and_their_disks_to_divide_disk_set(
        &mut self,
        high_left_extreme_point_p: &mut Point,
        low_right_extreme_point_q: &mut Point,
        disk_having_high_left_point_p: &mut usize,
        disk_having_low_right_point_q: &mut usize,
    ) {
        self.find_high_left_extreme_point_n_its_disk(
            high_left_extreme_point_p,
            disk_having_high_left_point_p,
        );
        self.find_low_right_extreme_point_n_its_disk(
            low_right_extreme_point_q,
            disk_having_low_right_point_q,
        );
    }

    fn find_high_left_extreme_point_n_its_disk(
        &mut self,
        high_left_extreme_point_p: &mut Point,
        disk_having_high_left_point_p: &mut usize,
    ) {
        *disk_having_high_left_point_p = 0;
        let mut left_most_x = self.disks[*disk_having_high_left_point_p].center.x
            - self.disks[*disk_having_high_left_point_p].radius;

        for (idx, disk) in self.disks.iter().enumerate() {
            if idx == 0 {
                continue;
            }

            let left_most_x_of_cur_disk = disk.center.x - disk.radius;

            if left_most_x_of_cur_disk < left_most_x {
                left_most_x = left_most_x_of_cur_disk;
                *disk_having_high_left_point_p = idx;
            } else if left_most_x_of_cur_disk == left_most_x {
                if disk.center.y > self.disks[*disk_having_high_left_point_p].center.y {
                    *disk_having_high_left_point_p = idx;
                } else if disk.center.y == self.disks[*disk_having_high_left_point_p].center.y {
                    if disk.radius > self.disks[*disk_having_high_left_point_p].radius {
                        *disk_having_high_left_point_p = idx;
                    }
                }
            }
        }

        *high_left_extreme_point_p = Point {
            x: left_most_x,
            y: self.disks[*disk_having_high_left_point_p].center.y,
        };
    }

    fn find_low_right_extreme_point_n_its_disk(
        &mut self,
        low_right_extreme_point_q: &mut Point,
        disk_having_low_right_point_q: &mut usize,
    ) {
        *disk_having_low_right_point_q = 0;
        let mut right_most_x = self.disks[*disk_having_low_right_point_q].center.x
            + self.disks[*disk_having_low_right_point_q].radius;

        for (idx, disk) in self.disks.iter().enumerate() {
            if idx == 0 {
                continue;
            }

            let right_most_x_of_cur_disk = disk.center.x + disk.radius;

            if right_most_x_of_cur_disk > right_most_x {
                right_most_x = right_most_x_of_cur_disk;
                *disk_having_low_right_point_q = idx;
            } else if right_most_x_of_cur_disk == right_most_x {
                if disk.center.y < self.disks[*disk_having_low_right_point_q].center.y {
                    *disk_having_low_right_point_q = idx;
                } else if disk.center.y == self.disks[*disk_having_low_right_point_q].center.y {
                    if disk.radius > self.disks[*disk_having_low_right_point_q].radius {
                        *disk_having_low_right_point_q = idx;
                    }
                }
            }
        }

        *low_right_extreme_point_q = Point {
            x: right_most_x,
            y: self.disks[*disk_having_low_right_point_q].center.y,
        };
    }

    fn divide_input_disks_into_two_initial_subsets(
        &mut self,
        oriented_line_segment_base_line_pq: &mut Line,
        initial_expanded_non_positive_disks_d_right: &mut Vec<usize>,
        initial_expanded_non_positive_disks_d_left: &mut Vec<usize>,
    ) {
        for (idx, disk) in self.disks.iter().enumerate() {
            if self.this_disk_is_a_member_of_expanded_non_positive_set_wrt_line(
                disk,
                oriented_line_segment_base_line_pq,
                true,
            ) {
                initial_expanded_non_positive_disks_d_right.push(idx);
            }

            if self.this_disk_is_a_member_of_expanded_non_negative_set_wrt_line(
                disk,
                oriented_line_segment_base_line_pq,
                true,
            ) {
                initial_expanded_non_positive_disks_d_left.push(idx);
            }
        }
    }

    fn this_disk_is_a_member_of_expanded_non_negative_set_wrt_line(
        &self,
        candidate_disk: &Disk,
        oriented_line_segment: &mut Line,
        including_on_negative: bool,
    ) -> bool {
        if including_on_negative {
            oriented_line_segment.signed_distance(candidate_disk.center)
                >= -candidate_disk.radius - 1e-6
        } else {
            oriented_line_segment.signed_distance(candidate_disk.center)
                > -candidate_disk.radius + 1e-6
        }
    }

    fn this_disk_is_a_member_of_expanded_non_positive_set_wrt_line(
        &self,
        candidate_disk: &Disk,
        oriented_line_segment: &mut Line,
        including_on_positive: bool,
    ) -> bool {
        if including_on_positive {
            oriented_line_segment.signed_distance(candidate_disk.center)
                <= candidate_disk.radius + 1e-6
        } else {
            oriented_line_segment.signed_distance(candidate_disk.center)
                < candidate_disk.radius - 1e-6
        }
    }

    fn prepare_and_insert_input_data_for_finding_hull_to_stack(
        &mut self,
        disks: &Vec<usize>,
        pre_apex_disk: &mut usize,
        post_apex_disk: &mut usize,
        hull_point_of_pre_apex_disk: &mut Point,
        hull_point_of_post_apex_disk: &mut Point,
        stack_for_finding_hull: &mut Vec<InputForFindHull>,
    ) {
        let input_for_find_hull = InputForFindHull {
            disks: disks.clone(),
            pre_apex_disk: *pre_apex_disk,
            post_apex_disk: *post_apex_disk,
            hull_point_of_pre_apex_disk: *hull_point_of_pre_apex_disk,
            hull_point_of_post_apex_disk: *hull_point_of_post_apex_disk,
        };

        stack_for_finding_hull.push(input_for_find_hull);
    }

    fn find_hull_disks_by_iteration(
        &mut self,
        stack_for_finding_hull: &mut Vec<InputForFindHull>,
        hull_disks: &mut Vec<usize>,
    ) {
        while !stack_for_finding_hull.is_empty() {
            let mut input_of_current_step = stack_for_finding_hull.pop().unwrap();
            let num_of_disk_in_d = input_of_current_step.disks.len();

            if num_of_disk_in_d == 1 {
                hull_disks.push(input_of_current_step.pre_apex_disk);
            } else if num_of_disk_in_d == 2
                && input_of_current_step.pre_apex_disk != input_of_current_step.post_apex_disk
            {
                hull_disks.push(input_of_current_step.pre_apex_disk);
                hull_disks.push(input_of_current_step.post_apex_disk);
            } else {
                let mut disks_d_front_edge = Vec::new();
                let mut disks_d_back_edge = Vec::new();

                let mut triangle_apex_x = Point::default();
                let mut apex_disk_dx = 0;

                let oriented_line_segment_base_edge_pq = Line::new(
                    input_of_current_step.hull_point_of_pre_apex_disk,
                    input_of_current_step.hull_point_of_post_apex_disk,
                );
                let mut candidate_apex_n_disk_pairs = Vec::new();

                self.find_the_highest_triangle_apex_and_the_apex_disk_wrt_this_oriented_line_segment(&input_of_current_step.disks, &oriented_line_segment_base_edge_pq, &mut candidate_apex_n_disk_pairs, false, None, None);

                if candidate_apex_n_disk_pairs.len() == 1 {
                    triangle_apex_x = candidate_apex_n_disk_pairs[0].0;
                    apex_disk_dx = candidate_apex_n_disk_pairs[0].1;
                } else {
                    self.pick_one_as_triangle_apex_and_apex_disk_among_disks_with_identical_height_and_remove_disks_contained_in_others_from_input_disks_if_exist(&candidate_apex_n_disk_pairs, &mut triangle_apex_x, &mut apex_disk_dx, &mut input_of_current_step.disks);
                }

                let oriented_line_segment_front_edge_px = Line::new(
                    input_of_current_step.hull_point_of_pre_apex_disk,
                    triangle_apex_x,
                );
                let oriented_line_segment_back_edge_xq = Line::new(
                    triangle_apex_x,
                    input_of_current_step.hull_point_of_post_apex_disk,
                );

                self.find_expanded_non_positive_disks_wrt_oriented_line_segment(
                    &input_of_current_step.disks,
                    &oriented_line_segment_front_edge_px,
                    &input_of_current_step.pre_apex_disk,
                    &apex_disk_dx,
                    &mut disks_d_front_edge,
                    true,
                );
                self.find_expanded_non_positive_disks_wrt_oriented_line_segment(
                    &input_of_current_step.disks,
                    &oriented_line_segment_back_edge_xq,
                    &apex_disk_dx,
                    &input_of_current_step.post_apex_disk,
                    &mut disks_d_back_edge,
                    true,
                );

                if self.triangle_filter_is_sliver(
                    input_of_current_step.disks.len(),
                    disks_d_front_edge.len(),
                    disks_d_back_edge.len(),
                    &input_of_current_step.pre_apex_disk,
                    &input_of_current_step.post_apex_disk,
                ) {
                    disks_d_front_edge.clear();
                    disks_d_back_edge.clear();

                    self.regularize_sliver_triangle_and_repivot_disks(
                        &mut input_of_current_step,
                        &mut disks_d_front_edge,
                        &mut disks_d_back_edge,
                        &mut apex_disk_dx,
                        &mut triangle_apex_x,
                    );
                }

                self.prepare_and_insert_input_data_for_finding_hull_to_stack(
                    &disks_d_back_edge,
                    &mut apex_disk_dx,
                    &mut input_of_current_step.post_apex_disk,
                    &mut triangle_apex_x,
                    &mut input_of_current_step.hull_point_of_post_apex_disk,
                    stack_for_finding_hull,
                );
                self.prepare_and_insert_input_data_for_finding_hull_to_stack(
                    &disks_d_front_edge,
                    &mut input_of_current_step.pre_apex_disk,
                    &mut apex_disk_dx,
                    &mut input_of_current_step.hull_point_of_pre_apex_disk,
                    &mut triangle_apex_x,
                    stack_for_finding_hull,
                );
            }
        }

        hull_disks.dedup();
    }

    fn find_the_highest_triangle_apex_and_the_apex_disk_wrt_this_oriented_line_segment(
        &mut self,
        disks_d: &Vec<usize>,
        oriented_line_segment_pq: &Line,
        apex_n_disk_pairs: &mut Vec<(Point, usize)>,
        oriented_line_is_not_negative_support_of_dp_n_dq: bool,
        pre_apex_disk_dp: Option<usize>,
        post_apex_disk_dq: Option<usize>,
    ) {
        let mut largest_max_perpendicular_distance_from_line_to_boundary_of_disk_among_disks = 0.0;

        for disk in disks_d.iter() {
            if oriented_line_is_not_negative_support_of_dp_n_dq
                && pre_apex_disk_dp.is_some()
                && ((pre_apex_disk_dp.is_some() && pre_apex_disk_dp.unwrap() == *disk)
                    || (post_apex_disk_dq.is_some() && post_apex_disk_dq.unwrap() == *disk))
            {
                continue;
            }

            let max_perpendicular_distance_from_line_to_boundary_of_cur_disk = self.disks[*disk]
                .radius
                - oriented_line_segment_pq.signed_distance(self.disks[*disk].center);

            if max_perpendicular_distance_from_line_to_boundary_of_cur_disk
                > largest_max_perpendicular_distance_from_line_to_boundary_of_disk_among_disks
                    + 1e-6
            {
                largest_max_perpendicular_distance_from_line_to_boundary_of_disk_among_disks =
                    max_perpendicular_distance_from_line_to_boundary_of_cur_disk;

                apex_n_disk_pairs.clear();
                let farthest_point_on_cur_disk_touching_tangent_line = self
                    .find_the_fartest_point_of_disk_from_this_line(disk, oriented_line_segment_pq);
                apex_n_disk_pairs.push((farthest_point_on_cur_disk_touching_tangent_line, *disk));
            } else if (max_perpendicular_distance_from_line_to_boundary_of_cur_disk
                - largest_max_perpendicular_distance_from_line_to_boundary_of_disk_among_disks)
                .abs()
                <= 1e-6
            {
                let farthest_point_on_cur_disk_touching_tangent_line = self
                    .find_the_fartest_point_of_disk_from_this_line(disk, oriented_line_segment_pq);
                apex_n_disk_pairs.push((farthest_point_on_cur_disk_touching_tangent_line, *disk));
            }
        }

        if oriented_line_is_not_negative_support_of_dp_n_dq
            && largest_max_perpendicular_distance_from_line_to_boundary_of_disk_among_disks.abs()
                <= 1e-6
        {
            if pre_apex_disk_dp.is_some() {
                apex_n_disk_pairs.push((oriented_line_segment_pq.start, pre_apex_disk_dp.unwrap()));
            }

            if post_apex_disk_dq.is_some() {
                apex_n_disk_pairs.push((oriented_line_segment_pq.end, post_apex_disk_dq.unwrap()));
            }
        }
    }

    fn find_the_fartest_point_of_disk_from_this_line(
        &mut self,
        disk: &usize,
        oriented_line: &Line,
    ) -> Point {
        let negative_direction = -oriented_line.get_normal_vector();
        let unit_vector = negative_direction.get_unit_vector();
        let farthest_point = Point {
            x: unit_vector.x * self.disks[*disk].radius,
            y: unit_vector.y * self.disks[*disk].radius,
        } + self.disks[*disk].center;

        farthest_point
    }

    fn pick_one_as_triangle_apex_and_apex_disk_among_disks_with_identical_height_and_remove_disks_contained_in_others_from_input_disks_if_exist(
        &mut self,
        apex_n_disk_pairs: &Vec<(Point, usize)>,
        triangle_apex_x: &mut Point,
        apex_disk_dx: &mut usize,
        disks_d: &mut Vec<usize>,
    ) {
        let mut candidate_of_triangle_apex = Point::default();
        let mut candidate_of_apex_disk = 0;

        self.pick_one_as_triangle_apex_and_apex_disk(
            apex_n_disk_pairs,
            &mut candidate_of_triangle_apex,
            &mut candidate_of_apex_disk,
        );

        let mut contained_disk_in_others = Vec::new();
        *triangle_apex_x = candidate_of_triangle_apex;
        *apex_disk_dx = self
            .find_largest_apex_disk_containing_this_apex_disk_selected_from_candidates(
                apex_n_disk_pairs,
                &candidate_of_apex_disk,
                &mut contained_disk_in_others,
            );

        if !contained_disk_in_others.is_empty() {
            self.remove_contained_disks_from_input_disks(&contained_disk_in_others, disks_d);
        }
    }

    fn pick_one_as_triangle_apex_and_apex_disk(
        &mut self,
        apex_n_disk_pairs: &Vec<(Point, usize)>,
        triangle_apex_x: &mut Point,
        apex_disk_dx: &mut usize,
    ) {
        if apex_n_disk_pairs.len() >= 2 {
            let generated_number = unsafe { rand() as usize % apex_n_disk_pairs.len() };
            *triangle_apex_x = apex_n_disk_pairs[generated_number].0;
            *apex_disk_dx = apex_n_disk_pairs[generated_number].1;
        } else {
            *triangle_apex_x = apex_n_disk_pairs[0].0;
            *apex_disk_dx = apex_n_disk_pairs[0].1;
        }
    }

    fn find_largest_apex_disk_containing_this_apex_disk_selected_from_candidates(
        &mut self,
        candidate_triangle_apex_n_disk_pairs: &Vec<(Point, usize)>,
        selected_apex_disk: &usize,
        contained_disks_in_others: &mut Vec<usize>,
    ) -> usize {
        let mut largest_apex_disk_containing_candidate_apex_disk = *selected_apex_disk;

        for (_, cur_apex_disk) in candidate_triangle_apex_n_disk_pairs.iter() {
            if *cur_apex_disk == largest_apex_disk_containing_candidate_apex_disk {
                continue;
            }

            if self.disks[*cur_apex_disk].contain(
                &self.disks[largest_apex_disk_containing_candidate_apex_disk],
                1e-6,
            ) {
                contained_disks_in_others.push(largest_apex_disk_containing_candidate_apex_disk);
                largest_apex_disk_containing_candidate_apex_disk = *cur_apex_disk;
            } else if self.disks[largest_apex_disk_containing_candidate_apex_disk]
                .contain(&self.disks[*cur_apex_disk], 1e-6)
            {
                contained_disks_in_others.push(*cur_apex_disk);
            }
        }

        largest_apex_disk_containing_candidate_apex_disk
    }

    fn remove_contained_disks_from_input_disks(
        &mut self,
        contained_disks_in_others: &Vec<usize>,
        disks_d: &mut Vec<usize>,
    ) {
        for contained_disk in contained_disks_in_others.iter() {
            disks_d.retain(|&x| x != *contained_disk);
        }
    }

    fn find_expanded_non_positive_disks_wrt_oriented_line_segment(
        &mut self,
        disks: &Vec<usize>,
        oriented_line_segment_of_two_points_on_d1_n_d2: &Line,
        disk_d1: &usize,
        disk_d2: &usize,
        output_disks: &mut Vec<usize>,
        including_on_positive: bool,
    ) {
        if oriented_line_segment_of_two_points_on_d1_n_d2.start
            != oriented_line_segment_of_two_points_on_d1_n_d2.end
        {
            for disk in disks.iter() {
                if *disk == *disk_d1 || *disk == *disk_d2 {
                    continue;
                } else {
                    if self.this_disk_is_a_member_of_expanded_non_positive_set_wrt_line_segment(
                        disk,
                        oriented_line_segment_of_two_points_on_d1_n_d2,
                        including_on_positive,
                    ) {
                        output_disks.push(*disk);
                    }
                }
            }
        }

        if *disk_d1 == *disk_d2 {
            output_disks.push(*disk_d1);
        } else {
            output_disks.push(*disk_d1);
            output_disks.push(*disk_d2);
        }
    }

    fn this_disk_is_a_member_of_expanded_non_positive_set_wrt_line_segment(
        &self,
        candidate_disk: &usize,
        oriented_line_segment: &Line,
        including_on_positive: bool,
    ) -> bool {
        let oriented_line = oriented_line_segment;
        let orthogonal_line_at_start_point =
            oriented_line.make_perpendicular_line(&oriented_line_segment.start);
        let orthogonal_line_at_end_point =
            oriented_line.make_perpendicular_line(&oriented_line_segment.end);

        let signed_distance_from_oriented_line_disk_center_point =
            oriented_line.signed_distance(self.disks[*candidate_disk].center);

        if signed_distance_from_oriented_line_disk_center_point
            <= -self.disks[*candidate_disk].radius + 1e-6
        {
            if orthogonal_line_at_start_point.signed_distance(self.disks[*candidate_disk].center)
                < -self.disks[*candidate_disk].radius - 1e-6
                && orthogonal_line_at_end_point.signed_distance(self.disks[*candidate_disk].center)
                    > self.disks[*candidate_disk].radius + 1e-6
            {
                true
            } else {
                false
            }
        } else if (signed_distance_from_oriented_line_disk_center_point
            > -self.disks[*candidate_disk].radius + 1e-6)
            && (signed_distance_from_oriented_line_disk_center_point
                < self.disks[*candidate_disk].radius - 1e-6)
        {
            if orthogonal_line_at_start_point.signed_distance(self.disks[*candidate_disk].center)
                < -1e-6
                && orthogonal_line_at_end_point.signed_distance(self.disks[*candidate_disk].center)
                    > 1e-6
            {
                true
            } else {
                false
            }
        } else if including_on_positive
            && (signed_distance_from_oriented_line_disk_center_point
                - self.disks[*candidate_disk].radius)
                .abs()
                <= 1e-6
        {
            if orthogonal_line_at_start_point.signed_distance(self.disks[*candidate_disk].center)
                < -1e-6
                && orthogonal_line_at_end_point.signed_distance(self.disks[*candidate_disk].center)
                    > 1e-6
            {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn triangle_filter_is_sliver(
        &self,
        num_of_input_disks: usize,
        num_of_disks_on_front_edge: usize,
        num_of_disks_on_back_edge: usize,
        pre_apex_disk_dp: &usize,
        post_apex_disk_dq: &usize,
    ) -> bool {
        pre_apex_disk_dp != post_apex_disk_dq
            && ((num_of_disks_on_front_edge == num_of_input_disks
                && num_of_disks_on_back_edge == 1)
                || (num_of_disks_on_front_edge == 1
                    && num_of_disks_on_back_edge == num_of_input_disks))
    }

    fn regularize_sliver_triangle_and_repivot_disks(
        &mut self,
        input_for_find_hull: &mut InputForFindHull,
        disks_d_front_edge: &mut Vec<usize>,
        disks_d_back_edge: &mut Vec<usize>,
        pivot_disk_dx: &mut usize,
        pivot_point_x: &mut Point,
    ) {
        let non_negative_supporting_tangent_line_segment_from_dp_to_dq = self
            .compute_a_ccw_oriented_tangent_line_from_disk_d1_to_d2(
                input_for_find_hull.pre_apex_disk,
                input_for_find_hull.post_apex_disk,
            );

        let mut triangle_apex_n_disk_pairs_of_expanded_non_positive = Vec::new();
        let mut triangle_apex_n_disk_pairs_of_on_positive_except_dp_n_dq = Vec::new();

        let sliver_configuration = self.find_sliver_triangle_configuration_and_apex_disks(
            &mut input_for_find_hull.disks,
            &non_negative_supporting_tangent_line_segment_from_dp_to_dq,
            &mut input_for_find_hull.pre_apex_disk,
            &mut input_for_find_hull.post_apex_disk,
            &mut triangle_apex_n_disk_pairs_of_expanded_non_positive,
            &mut triangle_apex_n_disk_pairs_of_on_positive_except_dp_n_dq,
        );

        match sliver_configuration {
            SliverConfiguration::SliverCaseA => {
                let mut triangle_apex_n_disk_pairs_of_dp_n_dq = Vec::new();
                triangle_apex_n_disk_pairs_of_dp_n_dq.push((
                    non_negative_supporting_tangent_line_segment_from_dp_to_dq.start,
                    input_for_find_hull.pre_apex_disk,
                ));
                triangle_apex_n_disk_pairs_of_dp_n_dq.push((
                    non_negative_supporting_tangent_line_segment_from_dp_to_dq.end,
                    input_for_find_hull.post_apex_disk,
                ));

                self.pick_one_as_triangle_apex_and_apex_disk(
                    &triangle_apex_n_disk_pairs_of_dp_n_dq,
                    pivot_point_x,
                    pivot_disk_dx,
                );
            }
            SliverConfiguration::SliverCaseB => {
                if triangle_apex_n_disk_pairs_of_on_positive_except_dp_n_dq.len() == 1 {
                    *pivot_point_x = triangle_apex_n_disk_pairs_of_on_positive_except_dp_n_dq[0].0;
                    *pivot_disk_dx = triangle_apex_n_disk_pairs_of_on_positive_except_dp_n_dq[0].1;
                } else {
                    self.pick_one_as_triangle_apex_and_apex_disk_among_disks_with_identical_height_and_remove_disks_contained_in_others_from_input_disks_if_exist(&triangle_apex_n_disk_pairs_of_on_positive_except_dp_n_dq, pivot_point_x, pivot_disk_dx, &mut input_for_find_hull.disks);
                }
            }
            SliverConfiguration::SliverCaseC1 => {
                *pivot_point_x = triangle_apex_n_disk_pairs_of_expanded_non_positive[0].0;
                *pivot_disk_dx = triangle_apex_n_disk_pairs_of_expanded_non_positive[0].1;
            }
            SliverConfiguration::SliverCaseC2 => {
                self.pick_one_as_triangle_apex_and_apex_disk_among_disks_with_identical_height_and_remove_disks_contained_in_others_from_input_disks_if_exist(&triangle_apex_n_disk_pairs_of_expanded_non_positive, pivot_point_x, pivot_disk_dx, &mut input_for_find_hull.disks);
            }
        }

        let oriented_line_segment_front_edge_px = Line::new(
            input_for_find_hull.hull_point_of_pre_apex_disk,
            *pivot_point_x,
        );
        self.find_expanded_non_positive_disks_wrt_oriented_line_segment(
            &input_for_find_hull.disks,
            &oriented_line_segment_front_edge_px,
            &input_for_find_hull.pre_apex_disk,
            pivot_disk_dx,
            disks_d_front_edge,
            true,
        );

        let oriented_line_segment_back_edge_xq = Line::new(
            *pivot_point_x,
            input_for_find_hull.hull_point_of_post_apex_disk,
        );
        self.find_expanded_non_positive_disks_wrt_oriented_line_segment(
            &input_for_find_hull.disks,
            &oriented_line_segment_back_edge_xq,
            pivot_disk_dx,
            &input_for_find_hull.post_apex_disk,
            disks_d_back_edge,
            true,
        );
    }

    fn compute_a_ccw_oriented_tangent_line_from_disk_d1_to_d2(
        &self,
        disk1: usize,
        disk2: usize,
    ) -> Line {
        let disk1 = &self.disks[disk1];
        let disk2 = &self.disks[disk2];

        if disk1.radius == 0.0 && disk2.radius == 0.0 {
            return Line::new(disk1.center, disk2.center);
        } else {
            let mut two_tangent_oriented_line_segment = vec![Line::default(); 2];
            self.compute_two_oriented_tangent_line_segments_from_circle_c1_to_c2(
                disk1,
                disk2,
                &mut two_tangent_oriented_line_segment,
            );

            let sum_of_signed_distance = two_tangent_oriented_line_segment[0]
                .signed_distance(disk1.center)
                + two_tangent_oriented_line_segment[0].signed_distance(disk2.center);

            if sum_of_signed_distance > 0.0 {
                two_tangent_oriented_line_segment[0].clone()
            } else {
                two_tangent_oriented_line_segment[1].clone()
            }
        }
    }

    fn compute_two_oriented_tangent_line_segments_from_circle_c1_to_c2(
        &self,
        disk1: &Disk,
        disk2: &Disk,
        two_tangent_oriented_line_segments: &mut Vec<Line>,
    ) {
        let mut implicit_equation_of_tangent_line1 = LineImplicitEquation::default();
        let mut implicit_equation_of_tangent_line2 = LineImplicitEquation::default();

        self.make_exterior_tangent_lines_of_two_circles(
            disk1,
            disk2,
            &mut implicit_equation_of_tangent_line1,
            &mut implicit_equation_of_tangent_line2,
        );

        let mut tangent_point_on_disk1 = [Point::default(); 2];
        let mut tangent_point_on_disk2 = [Point::default(); 2];

        if disk1.radius != 0.0 {
            tangent_point_on_disk1[0] = self.compute_tangent_point_between_line_and_circle(
                disk1,
                &implicit_equation_of_tangent_line1,
            );
            tangent_point_on_disk1[1] = self.compute_tangent_point_between_line_and_circle(
                disk1,
                &implicit_equation_of_tangent_line2,
            );
        } else {
            tangent_point_on_disk1[0] = disk1.center;
            tangent_point_on_disk1[1] = disk1.center;
        }

        if disk2.radius != 0.0 {
            tangent_point_on_disk2[0] = self.compute_tangent_point_between_line_and_circle(
                disk2,
                &implicit_equation_of_tangent_line1,
            );
            tangent_point_on_disk2[1] = self.compute_tangent_point_between_line_and_circle(
                disk2,
                &implicit_equation_of_tangent_line2,
            );
        } else {
            tangent_point_on_disk2[0] = disk2.center;
            tangent_point_on_disk2[1] = disk2.center;
        }

        two_tangent_oriented_line_segments[0] =
            Line::new(tangent_point_on_disk1[0], tangent_point_on_disk2[0]);
        two_tangent_oriented_line_segments[1] =
            Line::new(tangent_point_on_disk1[1], tangent_point_on_disk2[1]);
    }

    fn make_exterior_tangent_lines_of_two_circles(
        &self,
        disk1: &Disk,
        disk2: &Disk,
        result1: &mut LineImplicitEquation,
        result2: &mut LineImplicitEquation,
    ) {
        let w1 = if disk1.radius < disk2.radius {
            disk2
        } else {
            disk1
        };
        let w2 = if disk1.radius < disk2.radius {
            disk1
        } else {
            disk2
        };

        let c2c_vector = w1.center - w2.center;
        let r = w1.radius - w2.radius;
        let length = c2c_vector.magnitude();
        let sine = r / length;
        let cosine = (length * length - r * r).sqrt() / length;

        let mut normal1 = Point {
            x: c2c_vector.x * cosine - c2c_vector.y * sine,
            y: c2c_vector.x * sine + c2c_vector.y * cosine,
        };
        let mut normal2 = Point {
            x: c2c_vector.x * cosine + c2c_vector.y * sine,
            y: c2c_vector.y * cosine - c2c_vector.x * sine,
        };
        normal1 = normal1.get_unit_vector();
        normal2 = normal2.get_unit_vector();

        normal1.set_point(normal1.y, -1.0 * normal1.x);
        normal2.set_point(-1.0 * normal2.y, normal2.x);

        result1.coefficient[0] = normal1.x;
        result1.coefficient[1] = normal1.y;
        result1.coefficient[2] =
            -1.0 * normal1.x * w2.center.x - normal1.y * w2.center.y + w2.radius;

        result2.coefficient[0] = normal2.x;
        result2.coefficient[1] = normal2.y;
        result2.coefficient[2] =
            -1.0 * normal2.x * w2.center.x - normal2.y * w2.center.y + w2.radius;
    }

    fn compute_tangent_point_between_line_and_circle(
        &self,
        disk: &Disk,
        line_implicit_equation: &LineImplicitEquation,
    ) -> Point {
        let mut tangent_point = Point::default();

        let x = disk.center.x;
        let y = disk.center.y;
        let a = line_implicit_equation.coefficient[0];
        let b = line_implicit_equation.coefficient[1];
        let a2_b2 = a.powi(2) + b.powi(2);

        if (a2_b2 - 1.0).abs() <= 1e-6 {
            tangent_point.set_x(-a * line_implicit_equation.evaluate_implicit_equation(x, y) + x);
            tangent_point.set_y(-b * line_implicit_equation.evaluate_implicit_equation(x, y) + y);
        } else {
            tangent_point
                .set_x(-a * line_implicit_equation.evaluate_implicit_equation(x, y) / a2_b2 + x);
            tangent_point
                .set_y(-b * line_implicit_equation.evaluate_implicit_equation(x, y) / a2_b2 + y);
        }

        tangent_point
    }

    fn find_sliver_triangle_configuration_and_apex_disks(
        &mut self,
        disks_d: &mut Vec<usize>,
        non_negative_supporting_tangent_line_segment_from_dp_to_dq: &Line,
        pre_apex_disk_dp: &mut usize,
        post_apex_disk_dq: &mut usize,
        triangle_apex_n_disk_pairs_of_expanded_non_positive: &mut Vec<(Point, usize)>,
        triangle_apex_n_disk_pairs_of_on_positive_except_dp_n_dq: &mut Vec<(Point, usize)>,
    ) -> SliverConfiguration {
        let mut sliver_configuration = SliverConfiguration::SliverCaseA;

        let mut triangle_apex_n_disk_pairs = Vec::new();
        self.find_the_highest_triangle_apex_and_the_apex_disk_wrt_this_oriented_line_segment(
            disks_d,
            non_negative_supporting_tangent_line_segment_from_dp_to_dq,
            &mut triangle_apex_n_disk_pairs,
            true,
            Some(*pre_apex_disk_dp),
            Some(*post_apex_disk_dq),
        );

        let candidate_apex_disk = triangle_apex_n_disk_pairs[0].1;
        let height_of_triangle_apex = self.disks[candidate_apex_disk].radius
            - non_negative_supporting_tangent_line_segment_from_dp_to_dq
                .signed_distance(self.disks[candidate_apex_disk].center);

        if height_of_triangle_apex.abs() <= 1e-6 {
            let mut contained_disks_in_pre_or_post_apex_disk = Vec::new();
            self.remove_pre_and_post_apex_disks_and_contained_disks_in_one_of_the_two_from_candidate_apex_disks_if_exist(pre_apex_disk_dp, post_apex_disk_dq, &mut triangle_apex_n_disk_pairs, &mut contained_disks_in_pre_or_post_apex_disk);

            if !contained_disks_in_pre_or_post_apex_disk.is_empty() {
                self.remove_contained_disks_from_input_disks(
                    &contained_disks_in_pre_or_post_apex_disk,
                    disks_d,
                );
            }

            if triangle_apex_n_disk_pairs.is_empty() {
                sliver_configuration = SliverConfiguration::SliverCaseA;
            } else {
                sliver_configuration = SliverConfiguration::SliverCaseB;
                *triangle_apex_n_disk_pairs_of_on_positive_except_dp_n_dq =
                    triangle_apex_n_disk_pairs;
            }
        } else {
            if triangle_apex_n_disk_pairs.len() == 1 {
                sliver_configuration = SliverConfiguration::SliverCaseC1;
                triangle_apex_n_disk_pairs_of_expanded_non_positive
                    .push(triangle_apex_n_disk_pairs[0])
            } else {
                sliver_configuration = SliverConfiguration::SliverCaseC2;
                *triangle_apex_n_disk_pairs_of_expanded_non_positive = triangle_apex_n_disk_pairs;
            }
        }

        sliver_configuration
    }

    fn remove_pre_and_post_apex_disks_and_contained_disks_in_one_of_the_two_from_candidate_apex_disks_if_exist(
        &self,
        pre_apex_disk_dp: &mut usize,
        post_apex_disk_dq: &mut usize,
        candidate_triangle_apex_n_disk_pairs: &mut Vec<(Point, usize)>,
        contained_disk_in_pre_or_post_apex_disk: &mut Vec<usize>,
    ) {
        let mut idx = 0;

        while idx < candidate_triangle_apex_n_disk_pairs.len() {
            let candidate_apex_disk = candidate_triangle_apex_n_disk_pairs[idx].1;

            if candidate_apex_disk == *pre_apex_disk_dp || candidate_apex_disk == *post_apex_disk_dq
            {
                candidate_triangle_apex_n_disk_pairs.remove(idx);
                idx = idx.saturating_sub(1);
                continue;
            }

            if self.disks[*pre_apex_disk_dp].contain(&self.disks[candidate_apex_disk], 1e-6)
                || self.disks[*post_apex_disk_dq].contain(&self.disks[candidate_apex_disk], 1e-6)
            {
                contained_disk_in_pre_or_post_apex_disk.push(candidate_apex_disk);
                candidate_triangle_apex_n_disk_pairs.remove(idx);
                idx = idx.saturating_sub(1);
            } else {
                idx += 1;
            }
        }
    }

    fn extract_convex_hull_boundary(&self, hull_disks: &Vec<usize>) -> Vec<(Arc, Line)> {
        if hull_disks.is_empty() {
            return Vec::new();
        } else if hull_disks.len() == 1 {
            let arc = Arc::new(self.disks[hull_disks[0]].clone());
            let tangent_line = Line::new(
                Point {
                    x: f64::MAX,
                    y: f64::MAX,
                },
                Point {
                    x: f64::MAX,
                    y: f64::MAX,
                },
            );
            return vec![(arc, tangent_line)];
        }

        let num_of_tangent_lines = hull_disks.len() - 1;
        let mut tangent_line_segments = vec![Line::default(); num_of_tangent_lines];
        let mut geometry_of_hull_disks = vec![0; num_of_tangent_lines];

        let mut idx_disk = 0;
        let idx_last_disk_in_extreme_disks = hull_disks.len() - 1;
        let mut idx = 0;

        while idx_disk < idx_last_disk_in_extreme_disks {
            let cur_disk = hull_disks[idx_disk];
            geometry_of_hull_disks[idx] = cur_disk;

            idx_disk += 1;
            let next_disk = hull_disks[idx_disk];
            let oriented_tangent_line_segment =
                self.compute_a_ccw_oriented_tangent_line_from_disk_d1_to_d2(cur_disk, next_disk);
            tangent_line_segments[idx] = oriented_tangent_line_segment;

            idx += 1;
        }

        for i in 0..num_of_tangent_lines {
            if (tangent_line_segments[i % num_of_tangent_lines].end.x
                - tangent_line_segments[(i + 1) % num_of_tangent_lines]
                    .start
                    .x)
                .abs()
                <= 1e-5
                && (tangent_line_segments[i % num_of_tangent_lines].end.y
                    - tangent_line_segments[(i + 1) % num_of_tangent_lines]
                        .start
                        .y)
                    .abs()
                    <= 1e-5
            {
                tangent_line_segments[(i + 1) % num_of_tangent_lines].start =
                    tangent_line_segments[i % num_of_tangent_lines].end;
            }
        }

        let mut boundary_arc_n_lines = Vec::new();

        for i in 0..num_of_tangent_lines {
            let start_point_of_arc =
                tangent_line_segments[(i + num_of_tangent_lines - 1) % num_of_tangent_lines].end;
            let end_point_of_arc = tangent_line_segments[i].start;

            let cur_arc = Arc::new_with_points(
                self.disks[geometry_of_hull_disks[i]].clone(),
                start_point_of_arc,
                end_point_of_arc,
            );
            let tangent_line_segment_next_to_cur_arc = tangent_line_segments[i].clone();
            boundary_arc_n_lines.push((cur_arc, tangent_line_segment_next_to_cur_arc));
        }

        boundary_arc_n_lines
    }
}

fn is_point_inside_convex_hull(boundaries: Vec<(Arc, Line)>) -> bool {
    if boundaries.len() == 1 {
        let center = boundaries[0].0.disk.center;
        let d = center.x.hypot(center.y);

        return d <= boundaries[0].0.disk.radius;
    }

    let origin = Point { x: 0.0, y: 0.0 };
    let is_left = |a: Point, b: Point, c: Point| -> bool {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x) >= 0.0
    };

    for boundary in boundaries {
        let a = boundary.0.start;
        let b = boundary.0.end;

        if !is_left(a, b, origin) {
            return false;
        }

        let a = boundary.1.start;
        let b = boundary.1.end;

        if !is_left(a, b, origin) {
            return false;
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut disks = Vec::new();

    for _ in 1..=n {
        let (x, y, r) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );

        disks.push(Disk {
            center: Point { x, y },
            radius: r,
        });
    }

    disks.sort_by(|a, b| {
        if a.radius < b.radius {
            std::cmp::Ordering::Less
        } else if a.radius > b.radius {
            std::cmp::Ordering::Greater
        } else {
            if a.center.x < b.center.x {
                std::cmp::Ordering::Less
            } else if a.center.x > b.center.x {
                std::cmp::Ordering::Greater
            } else {
                if a.center.y < b.center.y {
                    std::cmp::Ordering::Less
                } else if a.center.y > b.center.y {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        }
    });

    disks.dedup();

    let mut quickhull_disk_algorithm = QuickhullDisk::new(disks);
    let hull_disks = quickhull_disk_algorithm.find_hull_disks();
    let boundary_arc_n_lines = quickhull_disk_algorithm.extract_convex_hull_boundary(&hull_disks);

    writeln!(
        out,
        "{}",
        if is_point_inside_convex_hull(boundary_arc_n_lines) {
            "Yes"
        } else {
            "No"
        }
    )
    .unwrap();
}
