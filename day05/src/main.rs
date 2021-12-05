use core::str::Split;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::fmt;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input.split("\n")));
    println!("{}", part_2(input.split("\n")));
}

struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn from_string(s: String) -> Line {
        let values: Vec<i32> = s
            .split("->")
            .map(|s| {
                s.to_string()
                    .split(",")
                    .map(|s2| s2.to_string())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>()
            .iter()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        Line {
            start: (values[0], values[1]),
            end: (values[2], values[3]),
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_diagonal(&self) -> bool {
        !(self.is_horizontal() || self.is_vertical())
    }

    fn get_points(&self, consider_diagonal: bool) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = vec![];
        if self.is_horizontal() {
            let forward = self.end.0 > self.start.0;
            if forward {
                for i in 0..=self.end.0 - self.start.0 {
                    points.push(((self.start.0 + i), (self.start.1)));
                }
            } else {
                for i in 0..=self.start.0 - self.end.0 {
                    points.push(((self.end.0 + i), (self.start.1)));
                }
            }
        }
        if self.is_vertical() {
            let forward = self.end.1 > self.start.1;
            if forward {
                for i in 0..=self.end.1 - self.start.1 {
                    points.push(((self.start.0), (self.start.1 + i)));
                }
            } else {
                for i in 0..=self.start.1 - self.end.1 {
                    points.push(((self.start.0), (self.end.1 + i)));
                }
            }
        }
        if self.is_diagonal() && consider_diagonal {
            let min_x_value = min(self.start.0, self.end.0);
            let min_y_value = min(self.start.1, self.end.1);
            let max_x_value = max(self.start.0, self.end.0);
            let max_y_value = max(self.start.1, self.end.1);
            let declining_line = (self.start.0 == min_x_value && self.start.1 == min_y_value)
                || (self.end.0 == min_x_value && self.end.1 == min_y_value);
            // will result in a line that goes from upper left to lower right
            let point_count = max_x_value - min_x_value;
            for i in 0..=point_count {
                if declining_line {
                    points.push(((min_x_value + i), (min_y_value + i)));
                } else {
                    points.push(((min_x_value + i), (max_y_value - i)));
                }
            }
        }
        points
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}) -> ({}, {})",
            self.start.0, self.start.1, self.end.0, self.end.1
        )
    }
}

fn part_1(input: Split<&str>) -> i32 {
    count_intersections(input, false)
}

fn part_2(input: Split<&str>) -> i32 {
    count_intersections(input, true)
}

fn count_intersections(input: Split<&str>, consider_diagonal: bool) -> i32 {
    let lines: Vec<Line> = input
        .map(|s| Line::from_string(s.to_string().trim().to_string()))
        .collect();
    let points: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| line.get_points(consider_diagonal))
        .flatten()
        .collect();
    let mut point_map: HashMap<(i32, i32), i32> = HashMap::new();
    for point in points {
        let count: i32 = if point_map.contains_key(&point) {
            *point_map.get(&point).unwrap() + 1
        } else {
            1
        };
        point_map.insert(point, count);
    }
    let mut cross_point_count = 0;
    for (_, val) in point_map.iter() {
        if *val > 1 {
            cross_point_count += 1;
        }
    }
    cross_point_count
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(5, part_1(input.split("\n")));
    assert_eq!(12, part_2(input.split("\n")));
}
