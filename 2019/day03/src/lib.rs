// Copyright 2024 Jedrzej Stuczynski
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// legacy code
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use aoc_solution::Aoc;
use itertools::Itertools;
use std::cmp::{max, min};

#[derive(Aoc)]
pub struct Day03;

#[derive(Debug, PartialEq, Clone)]
enum PointAxisTranslation {
    Up(i64),
    Right(i64),
    Down(i64),
    Left(i64),
}

impl PointAxisTranslation {
    fn from_str(raw: &str) -> Option<Self> {
        let mut chars_iter = raw.chars();
        let direction = chars_iter.next()?;

        let value_str: String = chars_iter.collect();
        let value = value_str.parse::<i64>();
        match value {
            Err(_) => None,
            Ok(val) => match direction {
                'U' => Some(PointAxisTranslation::Up(val)),
                'R' => Some(PointAxisTranslation::Right(val)),
                'D' => Some(PointAxisTranslation::Down(val)),
                'L' => Some(PointAxisTranslation::Left(val)),
                _ => None,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    #[allow(dead_code)]
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn translate_on_axis(&self, translation: PointAxisTranslation) -> Self {
        match translation {
            PointAxisTranslation::Up(val) => Point {
                x: self.x,
                y: self.y + val,
            },
            PointAxisTranslation::Right(val) => Point {
                x: self.x + val,
                y: self.y,
            },
            PointAxisTranslation::Down(val) => Point {
                x: self.x,
                y: self.y - val,
            },
            PointAxisTranslation::Left(val) => Point {
                x: self.x - val,
                y: self.y,
            },
        }
    }

    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }

    fn manhattan_distance_to_origin(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    fn distance_to(&self, other: Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    // Note: this method assumes that we already determined the point is an actual intersection
    // so that it's guaranteed to be collinear
    fn is_on_segment(&self, segment: &WireSegment) -> bool {
        let r1 = min(segment.start.x, segment.end.x)..=max(segment.start.x, segment.end.x);
        let r2 = min(segment.start.y, segment.end.y)..=max(segment.start.y, segment.end.y);

        r1.contains(&self.x) && r2.contains(&self.y)
    }
}

#[derive(Debug, Clone)]
struct WireSegment {
    start: Point,
    end: Point,
}

impl WireSegment {
    fn new(start: Point, end: Point) -> Self {
        WireSegment { start, end }
    }

    fn len(&self) -> i64 {
        self.start.distance_to(self.end)
    }

    fn intersection(&self, other: &Self) -> Option<Point> {
        let a1 = self.end.y - self.start.y;
        let b1 = self.start.x - self.end.x;
        let c1 = a1 * self.start.x + b1 * self.start.y;

        let a2 = other.end.y - other.start.y;
        let b2 = other.start.x - other.end.x;
        let c2 = a2 * other.start.x + b2 * other.start.y;

        let delta = a1 * b2 - a2 * b1;
        match delta {
            0 => None,
            _ => {
                let potential_intersection = Point {
                    x: (b2 * c1 - b1 * c2) / delta,
                    y: (a1 * c2 - a2 * c1) / delta,
                };

                if potential_intersection.is_on_segment(self)
                    && potential_intersection.is_on_segment(other)
                {
                    Some(potential_intersection)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Wire {
    segments: Vec<WireSegment>,
}

impl Wire {
    pub fn new_from_raw(raw_str: &str) -> Self {
        let origin = Point::origin();
        let points: Vec<_> = vec![origin] // we need to start our sequence with the origin
            .into_iter()
            .chain(
                raw_str
                    .split(',')
                    .map(|s| PointAxisTranslation::from_str(s).unwrap()) // if it panics during unwrap it means we got invalid input so there's nothing sensible we can do anyway
                    .scan(origin, |curr_point, translation| {
                        let new_point = curr_point.translate_on_axis(translation);
                        *curr_point = new_point;
                        // TODO: unnecessary copy
                        Some(new_point)
                    }),
            )
            .collect();

        let segments = points
            .into_iter()
            .tuple_windows()
            .map(|(p1, p2)| WireSegment::new(p1, p2))
            .collect();

        Self { segments }
    }

    fn retrace_steps(&self, point: &Point) -> i64 {
        // while not on segment, add segment len
        // then if on segment, add distance from seg start to point
        let intersection_segment_index = self
            .segments
            .iter()
            .position(|seg| point.is_on_segment(seg))
            .unwrap(); // if it panics, something weird must have happened...

        let full_segments_distance: i64 = self
            .segments
            .iter()
            .take(intersection_segment_index)
            .map(|seg| seg.len())
            .sum();

        full_segments_distance + point.distance_to(self.segments[intersection_segment_index].start)
    }

    fn all_intersections(&self, other: &Self) -> Vec<Point> {
        self.segments
            .iter()
            .flat_map(|w1_seg| other.segments.iter().map(move |w2_seg| (w1_seg, w2_seg)))
            .filter_map(|(w1_seg, w2_seg)| w1_seg.intersection(w2_seg))
            .collect()
    }

    fn closest_intersection_to_origin(&self, other: &Self) -> Point {
        // all intersections
        let origin = Point::origin();
        self.all_intersections(other)
            .into_iter()
            .filter(|p| p != &origin) // we don't want origin itself
            .map(|p| (p, p.manhattan_distance_to_origin()))
            .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
            .unwrap()
            .0
        // we don't care about distance itself, only the coordinates
    }

    fn least_step_intersection_distance(&self, other: &Self) -> i64 {
        // all intersections
        let origin = Point::origin();
        self.all_intersections(other)
            .into_iter()
            .filter(|p| p != &origin) // we don't want origin itself
            .map(|p| self.retrace_steps(&p) + other.retrace_steps(&p))
            .min()
            .unwrap()
    }
}

pub fn do_part1(input_wires: Vec<Wire>) {
    assert_eq!(2, input_wires.len()); // as per specs
    let closest_intersection_dist = input_wires[0]
        .closest_intersection_to_origin(&input_wires[1])
        .manhattan_distance_to_origin();
    println!("Part 1 answer: {}", closest_intersection_dist);
}

pub fn do_part2(input_wires: Vec<Wire>) {
    assert_eq!(2, input_wires.len()); // as per specs
    let least_steps = input_wires[0].least_step_intersection_distance(&input_wires[1]);
    println!("Part 2 answer: {}", least_steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correctly_determines_closest_intersection_for_first_input() {
        let wire1 = Wire::new_from_raw("R8,U5,L5,D3");
        let wire2 = Wire::new_from_raw("U7,R6,D4,L4");

        assert_eq!(
            6,
            wire1
                .closest_intersection_to_origin(&wire2)
                .manhattan_distance_to_origin()
        )
    }

    #[test]
    fn it_correctly_determines_closest_intersection_for_second_input() {
        let wire1 = Wire::new_from_raw("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = Wire::new_from_raw("U62,R66,U55,R34,D71,R55,D58,R83");

        assert_eq!(
            159,
            wire1
                .closest_intersection_to_origin(&wire2)
                .manhattan_distance_to_origin()
        )
    }

    #[test]
    fn it_correctly_determines_closest_intersection_for_third_input() {
        let wire1 = Wire::new_from_raw("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = Wire::new_from_raw("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(
            135,
            wire1
                .closest_intersection_to_origin(&wire2)
                .manhattan_distance_to_origin()
        )
    }

    #[test]
    fn it_correctly_determines_closest_intersection_steps_for_first_input() {
        let wire1 = Wire::new_from_raw("R8,U5,L5,D3");
        let wire2 = Wire::new_from_raw("U7,R6,D4,L4");

        assert_eq!(30, wire1.least_step_intersection_distance(&wire2))
    }

    #[test]
    fn it_correctly_determines_closest_intersection_steps_for_second_input() {
        let wire1 = Wire::new_from_raw("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = Wire::new_from_raw("U62,R66,U55,R34,D71,R55,D58,R83");

        assert_eq!(610, wire1.least_step_intersection_distance(&wire2))
    }

    #[test]
    fn it_correctly_determines_closest_intersection_steps_for_third_input() {
        let wire1 = Wire::new_from_raw("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = Wire::new_from_raw("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(410, wire1.least_step_intersection_distance(&wire2))
    }

    #[cfg(test)]
    mod segment_intersection {
        use super::*;

        #[test]
        fn it_correctly_detects_intersection() {
            let l1 = WireSegment::new(Point::new(4, 0), Point::new(6, 10));
            let l2 = WireSegment::new(Point::new(0, 3), Point::new(10, 7));
            assert_eq!(Point::new(5, 5), l1.intersection(&l2).unwrap())
        }

        #[test]
        fn it_correctly_detects_no_intersection() {
            let l1 = WireSegment::new(Point::new(0, 0), Point::new(1, 1));
            let l2 = WireSegment::new(Point::new(1, 2), Point::new(4, 5));
            assert_eq!(None, l1.intersection(&l2))
        }

        #[test]
        fn it_correctly_detects_no_intersection_for_parallel_lines() {
            let l1 = WireSegment::new(Point::new(0, 0), Point::new(1, 1));
            let l2 = WireSegment::new(Point::new(0, 1), Point::new(1, 2));
            assert_eq!(None, l1.intersection(&l2))
        }

        #[test]
        fn it_correctly_detects_no_intersection_outside_segments_even_if_infinite_lines_would_have_intersected(
        ) {
            let l1 = WireSegment::new(Point::new(0, 0), Point::new(1, 1));
            let l2 = WireSegment::new(Point::new(2, 3), Point::new(3, 2));
            assert_eq!(None, l1.intersection(&l2))
        }
    }

    #[cfg(test)]
    mod point_axis_translation {
        use super::*;

        #[test]
        fn it_returns_valid_up_translation() {
            assert_eq!(
                PointAxisTranslation::Up(10),
                PointAxisTranslation::from_str("U10").unwrap()
            );
        }

        #[test]
        fn it_returns_valid_right_translation() {
            assert_eq!(
                PointAxisTranslation::Right(10),
                PointAxisTranslation::from_str("R10").unwrap()
            );
        }

        #[test]
        fn it_returns_valid_down_translation() {
            assert_eq!(
                PointAxisTranslation::Down(10),
                PointAxisTranslation::from_str("D10").unwrap()
            );
        }

        #[test]
        fn it_returns_valid_left_translation() {
            assert_eq!(
                PointAxisTranslation::Left(10),
                PointAxisTranslation::from_str("L10").unwrap()
            );
        }

        #[test]
        fn it_returns_none_for_invalid_translations() {
            if let Some(_) = PointAxisTranslation::from_str("Z10") {
                panic!("expected nothing!")
            }
            if let Some(_) = PointAxisTranslation::from_str("Z1Y0") {
                panic!("expected nothing!")
            }
        }
    }
}
