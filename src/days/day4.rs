use crate::days::day::Day;

type Grid = Vec<Vec<char>>;
type Point = (usize, usize);

pub struct Day4 {}

impl Day4 {
    fn get_grid(&self, input: &str) -> Grid {
        let mut grid = Vec::new();
        for row in input.lines() {
            let mut letters = Vec::new();
            for letter in row.chars() {
                letters.push(letter);
            }
            grid.push(letters);
        }

        grid
    }

    fn num_xmas(&self, origin: Point, grid: &Grid) -> u8 {
        let (x, y) = origin;
        if grid[y][x] != 'X' {
            return 0;
        }

        let mut matches = 0;

        // Case 0: Straight up
        if y as isize - 3 >= 0 {
            if grid[y - 1][x] == 'M' && grid[y - 2][x] == 'A' && grid[y - 3][x] == 'S' {
                matches += 1;
            }
        }

        // Case 1: Diagonal up-right
        if y as isize - 3 >= 0 && x + 4 <= grid[0].len() {
            if grid[y - 1][x + 1] == 'M' && grid[y - 2][x + 2] == 'A' && grid[y - 3][x + 3] == 'S' {
                matches += 1;
            }
        }

        // Case 2: Out right
        if x + 4 <= grid[0].len() {
            if grid[y][x + 1] == 'M' && grid[y][x + 2] == 'A' && grid[y][x + 3] == 'S' {
                matches += 1;
            }
        }

        // Case 3: Diagonal down-right
        if y + 4 <= grid.len() && x + 4 <= grid[0].len() {
            if grid[y + 1][x + 1] == 'M' && grid[y + 2][x + 2] == 'A' && grid[y + 3][x + 3] == 'S' {
                matches += 1;
            }
        }

        // Case 4: Stright Down
        if y + 4 <= grid.len() {
            if grid[y + 1][x] == 'M' && grid[y + 2][x] == 'A' && grid[y + 3][x] == 'S' {
                matches += 1;
            }
        }

        // Case 5: Diagonal down-left
        if y + 4 <= grid.len() && x as isize - 3 >= 0 {
            if grid[y + 1][x - 1] == 'M' && grid[y + 2][x - 2] == 'A' && grid[y + 3][x - 3] == 'S' {
                matches += 1;
            }
        }

        // Case 6: Out left
        if x as isize - 3 >= 0 {
            if grid[y][x - 1] == 'M' && grid[y][x - 2] == 'A' && grid[y][x - 3] == 'S' {
                matches += 1;
            }
        }

        // Case 7: Diagonal up-left
        if y as isize - 3 >= 0 && x as isize - 3 >= 0 {
            if grid[y - 1][x - 1] == 'M' && grid[y - 2][x - 2] == 'A' && grid[y - 3][x - 3] == 'S' {
                matches += 1;
            }
        }

        matches
    }

    fn has_x(&self, origin: Point, grid: &Grid) -> bool {
        let (x, y) = origin;
        if grid[y][x] != 'A' {
            return false;
        }

        if y < 1 || y + 1 == grid.len() {
            return false;
        }

        if x < 1 || x + 1 == grid[0].len() {
            return false;
        }

        let mut num_legs = 0;
        if grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S' {
            num_legs += 1;
        }
        if grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S' {
            num_legs += 1;
        }
        if grid[y + 1][x - 1] == 'M' && grid[y - 1][x + 1] == 'S' {
            num_legs += 1;
        }
        if grid[y + 1][x + 1] == 'M' && grid[y - 1][x - 1] == 'S' {
            num_legs += 1;
        }

        // It's not possible for there to be more than 2, but whatever
        num_legs == 2
    }
}

impl Day for Day4 {
    fn part1(&self, input: &str) {
        let grid = self.get_grid(input);

        let mut num_words: u16 = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                num_words += self.num_xmas((x, y), &grid) as u16;
            }
        }

        println!("{}", num_words);
    }

    fn part2(&self, input: &str) {
        let grid = self.get_grid(input);

        let mut num_xes: u16 = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if self.has_x((x, y), &grid) {
                    num_xes += 1;
                }
            }
        }

        println!("{}", num_xes);
    }
}
