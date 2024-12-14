use crate::days::day::Day;
use std::collections::HashSet;

type Points = HashSet<Point>;
type Guard = (Point, Direction);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    x: u8,
    y: u8,
}

impl std::hash::Hash for Point {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write(&[b'(', self.x, b',', self.y, b')']);
        state.finish();
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FancyPoint {
    x: u8,
    y: u8,
    d: Direction,
}

impl std::hash::Hash for FancyPoint {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        let d = match self.d {
            Direction::UP => 0,
            Direction::DOWN => 1,
            Direction::RIGHT => 2,
            Direction::LEFT => 3,
        };
        state.write(&[b'(', self.x, b',', self.y, b',', d, b')']);
        state.finish();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

pub struct Day6 {}

impl Day6 {
    fn parse_input(&self, input: &str) -> (Points, Guard, u8) {
        let mut obstacles: Points = HashSet::new();
        let mut guard = (Point { x: 0, y: 0 }, Direction::UP);

        let mut map_size = 0;
        for (y, row) in input.lines().enumerate() {
            if map_size == 0 {
                map_size = row.len() as u8;
            }
            for (x, spot) in row.chars().enumerate() {
                if spot == '^' {
                    guard = (
                        Point {
                            x: x as u8 + 1,
                            y: y as u8 + 1,
                        },
                        Direction::UP,
                    );
                }
                if spot == '#' {
                    obstacles.insert(Point {
                        x: x as u8 + 1,
                        y: y as u8 + 1,
                    });
                }
            }
        }

        (obstacles, guard, map_size)
    }

    fn step(&self, guard: &mut Guard, obstacles: &Points) {
        match guard.1 {
            Direction::UP => {
                if obstacles.contains(&Point {
                    x: guard.0.x,
                    y: guard.0.y - 1,
                }) {
                    guard.1 = Direction::RIGHT;
                } else {
                    guard.0.y -= 1;
                }
            }
            Direction::DOWN => {
                if obstacles.contains(&Point {
                    x: guard.0.x,
                    y: guard.0.y + 1,
                }) {
                    guard.1 = Direction::LEFT;
                } else {
                    guard.0.y += 1;
                }
            }
            Direction::LEFT => {
                if obstacles.contains(&Point {
                    x: guard.0.x - 1,
                    y: guard.0.y,
                }) {
                    guard.1 = Direction::UP;
                } else {
                    guard.0.x -= 1;
                }
            }
            Direction::RIGHT => {
                if obstacles.contains(&Point {
                    x: guard.0.x + 1,
                    y: guard.0.y,
                }) {
                    guard.1 = Direction::DOWN;
                } else {
                    guard.0.x += 1;
                }
            }
        }
    }
}

impl Day for Day6 {
    fn part1(&self, input: &str) {
        let (obstacles, mut guard, map_size) = self.parse_input(&input);
        let mut visited_points: Points = HashSet::new();

        loop {
            visited_points.insert(Point {
                x: guard.0.x,
                y: guard.0.y,
            });

            self.step(&mut guard, &obstacles);

            if guard.0.x == 0 || guard.0.y == 0 || guard.0.x > map_size || guard.0.y > map_size {
                break;
            }
        }

        println!("{}", visited_points.len());
    }

    fn part2(&self, input: &str) {
        let (mut obstacles, mut guard, map_size) = self.parse_input(&input);
        let mut visited_points: HashSet<FancyPoint> = HashSet::new();

        let mut new_obstacles: HashSet<Point> = HashSet::new();
        loop {
            visited_points.insert(FancyPoint {
                x: guard.0.x,
                y: guard.0.y,
                d: guard.1,
            });

            self.step(&mut guard, &obstacles);

            if guard.0.x == 0 || guard.0.y == 0 || guard.0.x > map_size || guard.0.y > map_size {
                break;
            }

            // If we've already visited this spot in ANY direction in the past,
            // we can't put an obstacle here now since it would have blocked us
            // earlier.
            //
            // Me in my infinite wisdom made no easy way to check for a past-visited
            // point independent of direction so we'll just try all four lol.
            if visited_points.contains(&FancyPoint {
                x: guard.0.x,
                y: guard.0.y,
                d: Direction::UP,
            }) || visited_points.contains(&FancyPoint {
                x: guard.0.x,
                y: guard.0.y,
                d: Direction::DOWN,
            }) || visited_points.contains(&FancyPoint {
                x: guard.0.x,
                y: guard.0.y,
                d: Direction::RIGHT,
            }) || visited_points.contains(&FancyPoint {
                x: guard.0.x,
                y: guard.0.y,
                d: Direction::LEFT,
            }) {
                continue;
            }

            if !new_obstacles.contains(&Point {
                x: guard.0.x,
                y: guard.0.y,
            }) {
                let point = guard.clone();
                let original_visited_points = visited_points.clone();

                // A candidate obstacle must satisfy two requirements:
                // 1. It must be put somewhere on the guard's original path (except starting point)
                // 2. It must cause the guard to go back to a point & direction they've been before
                //
                // As soon as we see #2, we know we've caused a loop since being put in the same
                // spot + direction they've been before would produce the same path.

                let new_obstacle = Point {
                    x: point.0.x,
                    y: point.0.y,
                };
                obstacles.insert(new_obstacle);

                let (new_x, new_y) = match point.1 {
                    Direction::UP => (point.0.x, point.0.y + 1),
                    Direction::DOWN => (point.0.x, point.0.y - 1),
                    Direction::RIGHT => (point.0.x - 1, point.0.y),
                    Direction::LEFT => (point.0.x + 1, point.0.y),
                };

                guard = (Point { x: new_x, y: new_y }, point.1);

                let mut valid = false;
                loop {
                    visited_points.insert(FancyPoint {
                        x: guard.0.x,
                        y: guard.0.y,
                        d: guard.1,
                    });

                    self.step(&mut guard, &obstacles);

                    if visited_points.contains(&FancyPoint {
                        x: guard.0.x,
                        y: guard.0.y,
                        d: guard.1,
                    }) {
                        valid = true;
                        break;
                    }
                    if guard.0.x == 0
                        || guard.0.y == 0
                        || guard.0.x > map_size
                        || guard.0.y > map_size
                    {
                        break;
                    }
                }

                if valid {
                    new_obstacles.insert(Point {
                        x: point.0.x,
                        y: point.0.y,
                    });
                }

                obstacles.remove(&new_obstacle);

                guard = point;
                visited_points = original_visited_points;
            }
        }

        println!("{}", new_obstacles.len());
    }
}
