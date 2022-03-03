use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

struct Instruction {
    on: bool,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Instruction {
    fn to_cuboid(&self) -> Cuboid {
        Cuboid {
            x: (self.x.0, self.x.1 + 1),
            y: (self.y.0, self.y.1 + 1),
            z: (self.z.0, self.z.1 + 1),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let lines = input
        .split('\n')
        .map(|s| s.to_string());
    let re = Regex::new("-?[0-9]+").unwrap();
    lines
        .into_iter()
        .map(|line| {
            let on = &(line.split(' ').next().unwrap().to_string()) == "on";
            let number_part = line.split(' ').nth(1).unwrap().to_string();
            let x_part = number_part.split(',').next().unwrap().to_string();
            let y_part = number_part.split(',').nth(1).unwrap().to_string();
            let z_part = number_part.split(',').nth(2).unwrap().to_string();
            Instruction {
                on,
                x: (
                    re.find_iter(&x_part)
                        .next()
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap(),
                    re.find_iter(&x_part)
                        .nth(1)
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap(),
                ),
                y: (
                    re.find_iter(&y_part)
                        .next()
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap(),
                    re.find_iter(&y_part)
                        .nth(1)
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap(),
                ),
                z: (
                    re.find_iter(&z_part)
                        .next()
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap(),
                    re.find_iter(&z_part)
                        .nth(1)
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap(),
                ),
            }
        })
        .collect::<Vec<Instruction>>()
}

fn instruction_is_relevant_for_part_1(instruction: &Instruction) -> bool {
    instruction.x.0 >= -50
        && instruction.y.0 >= -50
        && instruction.z.0 >= -50
        && instruction.x.1 <= 50
        && instruction.y.1 <= 50
        && instruction.z.1 <= 50
}

fn part_1(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut on_positions = HashMap::new();
    for instruction in instructions {
        if !instruction_is_relevant_for_part_1(&instruction) {
            continue;
        }
        for x in instruction.x.0..=instruction.x.1 {
            for y in instruction.y.0..=instruction.y.1 {
                for z in instruction.z.0..=instruction.z.1 {
                    match instruction.on {
                        true => {
                            on_positions.insert((x, y, z), true);
                        }
                        false => {
                            on_positions.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
    }
    on_positions.len()
}
#[derive(Clone, Copy)]
struct Cuboid {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Cuboid {
    fn volume(&self) -> u64 {
        (i64::abs(self.x.0 - self.x.1)
            * i64::abs(self.y.0 - self.y.1)
            * i64::abs(self.z.0 - self.z.1)) as u64
    }

    fn is_empty(&self) -> bool {
        self.volume() == 0 || self.y.0 >= self.y.1 || self.x.0 >= self.x.1 || self.z.0 >= self.z.1
    }

    fn intersects(&self, other: &Cuboid) -> bool {
        !(other.x.0 > self.x.1 - 1
            || other.x.1 - 1 < self.x.0
            || other.y.0 > self.y.1 - 1
            || other.y.1 - 1 < self.y.0
            || other.z.0 > self.z.1 - 1
            || other.z.1 - 1 < self.z.0)
    }

    fn subtract(&self, other: &Cuboid) -> Vec<Cuboid> {
        if !self.intersects(other) {
            vec![*self]
        } else {
            [
                Cuboid {
                    x: (self.x.0, other.x.0),
                    y: (self.y.0, self.y.1),
                    z: (self.z.0, self.z.1),
                },
                Cuboid {
                    x: (other.x.1, self.x.1),
                    y: (self.y.0, self.y.1),
                    z: (self.z.0, self.z.1),
                },
                Cuboid {
                    x: (i64::max(self.x.0,other.x.0), i64::min(self.x.1,other.x.1)),
                    y: (self.y.0, other.y.0),
                    z: (self.z.0, self.z.1),
                },
                Cuboid {
                    x: (i64::max(self.x.0,other.x.0), i64::min(self.x.1,other.x.1)),
                    y: (other.y.1, self.y.1),
                    z: (self.z.0, self.z.1),
                },
                Cuboid {
                    x: (i64::max(self.x.0,other.x.0), i64::min(self.x.1,other.x.1)),
                    y: (i64::max(self.y.0,other.y.0), i64::min(self.y.1,other.y.1)),
                    z: (self.z.0, other.z.0),
                },
                Cuboid {
                    x: (i64::max(self.x.0,other.x.0), i64::min(self.x.1,other.x.1)),
                    y: (i64::max(self.y.0,other.y.0), i64::min(self.y.1,other.y.1)),
                    z: (other.z.1, self.z.1),
                },
            ]
            .iter()
            .filter(|c| !c.is_empty())
            .map(|c| *c)
            .collect::<Vec<Cuboid>>()
        }
    }
}

fn part_2(input: &str) -> u64 {
    let instructions = parse_input(input);
    let mut cuboids: Vec<Cuboid> = vec![];
    for instruction in instructions {
        let new_cuboid = instruction.to_cuboid();
        match instruction.on {
            true => {
                let mut cuts = vec![new_cuboid];

                for cuboid in cuboids.iter() {
                    let mut new_cuts = vec!();

                    for cut in cuts.iter() {
                        new_cuts.extend(cut.subtract(cuboid));
                    }
                    cuts = new_cuts;
                }
                cuboids.extend(cuts);
            }
            false => {
                let mut cuts = vec!();
                for cuboid in cuboids.into_iter() {
                    cuts.extend(cuboid.subtract(&new_cuboid));
                }
                cuboids = cuts;
            }
        }
    }
    cuboids.iter().fold(0, |acc, cuboid| acc + cuboid.volume())
}

#[test]
fn test_example() {
    assert_eq!(39, part_1(include_str!("../small_example.txt")));
    assert_eq!(590784, part_1(include_str!("../example.txt")));
    assert_eq!(
        2758514936282235,
        part_2(include_str!("../large_example.txt"))
    );
}
