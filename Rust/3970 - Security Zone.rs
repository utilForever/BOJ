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

#[derive(Default, Copy, Clone, PartialEq)]
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

#[derive(Default)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
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

struct Disk {
    id: usize,
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

struct InputForFindHull {
    disks: Vec<usize>,
    pre_apex_disk: usize,
    post_apex_disk: usize,
    hull_point_of_pre_apex_disk: Point,
    hull_point_of_post_apex_disk: Point,
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
        let mut initial_expanded_non_positive_disks_d_left = Vec::new();
        let mut oriented_line_segment_base_line_pq =
            Line::new(high_left_extreme_point_p, low_right_extreme_point_q);

        self.divide_input_disks_into_two_initial_subsets(
            &mut oriented_line_segment_base_line_pq,
            &mut initial_expanded_non_positive_disks_d_right,
            &mut initial_expanded_non_positive_disks_d_left,
        );

        let mut stack_for_finding_hull = Vec::new();

        self.prepare_and_insert_input_data_for_finding_hull_to_stack(
            &initial_expanded_non_positive_disks_d_left,
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

        for (idx, disk) in self.disks.iter().skip(1).enumerate() {
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

        for (idx, disk) in self.disks.iter().skip(1).enumerate() {
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
            } else if self.this_disk_is_a_member_of_expanded_non_negative_set_wrt_line(
                disk,
                oriented_line_segment_base_line_pq,
                true,
            ) {
                initial_expanded_non_positive_disks_d_left.push(idx);
            }
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
                >= -candidate_disk.radius - 1e-6
        } else {
            oriented_line_segment.signed_distance(candidate_disk.center)
                > -candidate_disk.radius - 1e-6
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
                <= candidate_disk.radius + 1e-6
        } else {
            oriented_line_segment.signed_distance(candidate_disk.center)
                < candidate_disk.radius + 1e-6
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
            let (
                mut disks_d,
                mut pre_apex_disk_dp,
                mut post_apex_disk_dq,
                mut hull_point_p,
                mut hull_point_q,
            ) = (
                &mut input_of_current_step.disks,
                &mut input_of_current_step.pre_apex_disk,
                &mut input_of_current_step.post_apex_disk,
                &mut input_of_current_step.hull_point_of_pre_apex_disk,
                &mut input_of_current_step.hull_point_of_post_apex_disk,
            );

            let num_of_disk_in_d = disks_d.len();

            if num_of_disk_in_d == 1 {
                hull_disks.push(*pre_apex_disk_dp);
            } else if num_of_disk_in_d == 2 && pre_apex_disk_dp != post_apex_disk_dq {
                hull_disks.push(*pre_apex_disk_dp);
                hull_disks.push(*post_apex_disk_dq);
            } else {
                let mut disks_d_front_edge = Vec::new();
                let mut disks_d_back_edge = Vec::new();

                let mut triangle_apex_x = Point::default();
                let mut apex_disk_dx = 0;

                let oriented_line_segment_base_edge_pq = Line::new(*hull_point_p, *hull_point_q);
                let mut candidate_apex_n_disk_pairs = Vec::new();

                self.find_the_highest_triangle_apex_and_the_apex_disk_wrt_this_oriented_line_segment(&disks_d, &oriented_line_segment_base_edge_pq, &mut candidate_apex_n_disk_pairs, false, None, None);

                if candidate_apex_n_disk_pairs.len() == 1 {
                    triangle_apex_x = candidate_apex_n_disk_pairs[0].0;
                    apex_disk_dx = candidate_apex_n_disk_pairs[0].1;
                } else {
                    self.pick_one_as_triangle_apex_and_apex_disk_among_disks_with_identical_height_and_remove_disks_contained_in_others_from_input_disks_if_exist(&candidate_apex_n_disk_pairs, &mut triangle_apex_x, &mut apex_disk_dx, &mut disks_d);
                }

                let oriented_line_segment_front_edge_px = Line::new(*hull_point_p, triangle_apex_x);
                let oriented_line_segment_back_edge_xq = Line::new(triangle_apex_x, *hull_point_q);

                self.find_expanded_non_positive_disks_wrt_oriented_line_segment(
                    &disks_d,
                    &oriented_line_segment_front_edge_px,
                    &pre_apex_disk_dp,
                    &apex_disk_dx,
                    &mut disks_d_front_edge,
                    true,
                );
                self.find_expanded_non_positive_disks_wrt_oriented_line_segment(
                    &disks_d,
                    &oriented_line_segment_back_edge_xq,
                    &apex_disk_dx,
                    &post_apex_disk_dq,
                    &mut disks_d_back_edge,
                    true,
                );

                if self.triangle_filter_is_sliver(
                    disks_d.len(),
                    disks_d_front_edge.len(),
                    disks_d_back_edge.len(),
                    &pre_apex_disk_dp,
                    &post_apex_disk_dq,
                ) {
                    disks_d_front_edge.clear();
                    disks_d_back_edge.clear();

                    self.regularize_sliver_triangle_and_repivot_disks();
                }

                self.prepare_and_insert_input_data_for_finding_hull_to_stack(
                    &disks_d_back_edge,
                    &mut apex_disk_dx,
                    &mut post_apex_disk_dq,
                    &mut triangle_apex_x,
                    &mut hull_point_q,
                    stack_for_finding_hull,
                );
                self.prepare_and_insert_input_data_for_finding_hull_to_stack(
                    &disks_d_front_edge,
                    &mut pre_apex_disk_dp,
                    &mut apex_disk_dx,
                    &mut hull_point_p,
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
        oriented_line_segment_base_edge_pq: &Line,
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
                - oriented_line_segment_base_edge_pq.signed_distance(self.disks[*disk].center);

            if max_perpendicular_distance_from_line_to_boundary_of_cur_disk
                > largest_max_perpendicular_distance_from_line_to_boundary_of_disk_among_disks
                    + 1e-6
            {
                largest_max_perpendicular_distance_from_line_to_boundary_of_disk_among_disks =
                    max_perpendicular_distance_from_line_to_boundary_of_cur_disk;

                apex_n_disk_pairs.clear();
                let farthest_point_on_cur_disk_touching_tangent_line = self
                    .find_the_fartest_point_of_disk_from_this_line(
                        disk,
                        oriented_line_segment_base_edge_pq,
                    );
                apex_n_disk_pairs.push((farthest_point_on_cur_disk_touching_tangent_line, *disk));
            } else if (max_perpendicular_distance_from_line_to_boundary_of_cur_disk
                - largest_max_perpendicular_distance_from_line_to_boundary_of_disk_among_disks)
                .abs()
                <= 1e-6
            {
                let farthest_point_on_cur_disk_touching_tangent_line = self
                    .find_the_fartest_point_of_disk_from_this_line(
                        disk,
                        oriented_line_segment_base_edge_pq,
                    );
                apex_n_disk_pairs.push((farthest_point_on_cur_disk_touching_tangent_line, *disk));
            }
        }

        if oriented_line_is_not_negative_support_of_dp_n_dq
            && largest_max_perpendicular_distance_from_line_to_boundary_of_disk_among_disks.abs()
                <= 1e-6
        {
            if pre_apex_disk_dp.is_some() {
                apex_n_disk_pairs.push((
                    oriented_line_segment_base_edge_pq.start,
                    pre_apex_disk_dp.unwrap(),
                ));
            }

            if post_apex_disk_dq.is_some() {
                apex_n_disk_pairs.push((
                    oriented_line_segment_base_edge_pq.end,
                    post_apex_disk_dq.unwrap(),
                ));
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
            let index = disks_d.iter().position(|&x| x == *contained_disk).unwrap();
            disks_d.remove(index);
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
                <= -self.disks[*candidate_disk].radius - 1e-6
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
            return false;
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
        input_for_find_hull: &InputForFindHull,
        disks_d_front_edge: &mut Vec<usize>,
        disks_d_back_edge: &mut Vec<usize>,
        pivot_disk_dx: &mut usize,
        pivot_point_x: &mut Point,
    ) {
        let (mut disks_d, mut pre_apex_disk_dp, mut post_apex_disk_dq, hull_point_p, hull_point_q) = (
            &mut input_for_find_hull.disks,
            &mut input_for_find_hull.pre_apex_disk,
            &mut input_for_find_hull.post_apex_disk,
            input_for_find_hull.hull_point_of_pre_apex_disk,
            input_for_find_hull.hull_point_of_post_apex_disk,
        );

        
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    // let c = scan.token::<i64>();
    let c = 1;

    for _ in 0..c {
        let n = scan.token::<usize>();
        let mut disks = Vec::new();

        for i in 1..=n {
            let (x, y, r) = (
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>() + 10.0,
            );

            disks.push(Disk {
                id: i,
                center: Point { x, y },
                radius: r,
            });
        }

        let mut quickhull_disk_algorithm = QuickhullDisk::new(disks);
        let hull_disks = quickhull_disk_algorithm.find_hull_disks();
    }
}
