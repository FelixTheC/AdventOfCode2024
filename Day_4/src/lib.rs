
pub fn read_test_input() -> Vec<Vec<char>> {
    let input = include_str!("../test_input.txt");

    let mut result: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {

        let mut line_result: Vec<char> = Vec::new();

        for chr in line.chars() {
            line_result.push(chr);
        }

        result.push(line_result);
    }

    result
}


struct Xmas {
    x: bool,
    m: bool,
    a: bool,
    s: bool,
}

impl Xmas {
    fn new() -> Xmas {
        Xmas{x: false, m: false, a: false, s: false}
    }

    fn next_token(&self) -> char {
        if !self.x {
            'X'
        } else if self.x && !self.m {
            'M'
        } else if self.x && self.m && !self.a {
            'A'
        } else {
            'S'
        }
    }

    fn set_token(&mut self, token: char) {
        match token {
            'X' => self.x = true,
            'M' => self.m = true,
            'A' => self.a = true,
            'S' => self.s = true,
            _ => ()
        }
    }

    fn is_complete(&self) -> bool {
        self.x && self.m && self.a && self.s
    }

    fn reset(&mut self) {
        self.x = false;
        self.m = false;
        self.a = false;
        self.s = false;
    }
}

fn ceres_search_sub(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u16 {
    let mut result: u16 = 0;
    let mut xmas = Xmas::new();
    xmas.set_token('X');

    // look horizontal to the right side
    let max_col = grid[row].len() - 4;
    if col <= max_col {
        for idx in 0 .. 4 {
            if grid[row][col + idx] == xmas.next_token() {
                xmas.set_token(grid[row][col + idx]);
            }
        }
    }

    if xmas.is_complete() {
        result += 1;
    }

    xmas.reset();

    if col >= 4 {
        // look horizontal to the left side
        for idx in 0 .. 4 {
            if grid[row][col - idx] == xmas.next_token() {
                xmas.set_token(grid[row][col - idx]);
            }
        }
    }

    if xmas.is_complete() {
        result += 1;
    }

    xmas.reset();

    // look vertical down
    if row <= grid.len() - 4 {
        for idx in 0 .. 4 {
            if grid[row + idx][col] == xmas.next_token() {
                xmas.set_token(grid[row + idx][col]);
            }
        }
    }

    if xmas.is_complete() {
        result += 1;
    }

    xmas.reset();

    // look vertical up
    if row >= 4 {
        for idx in 0 .. 4 {
            if grid[row - idx][col ] == xmas.next_token() {
                xmas.set_token(grid[row - idx][col]);
            }
        }
    }

    if xmas.is_complete() {
        result += 1;
    }

    xmas.reset();

    // diagonal lookup
    for idx in 0 .. 4 {
        if row + idx < grid.len() && col + idx < grid[row].len() {
            if grid[row + idx][col + idx] == xmas.next_token() {
                xmas.set_token(grid[row + idx][col + idx]);
            }
        }
    }

    if xmas.is_complete() {
        result += 1;
    }

    xmas.reset();

    // diagonal lookup
    for idx in 0 .. 4 {
        if row + idx < grid.len() && idx <= col && col - idx > 0 {
            if grid[row + idx][col - idx] == xmas.next_token() {
                xmas.set_token(grid[row + idx][col - idx]);
            }
        }
    }

    if xmas.is_complete() {
        result += 1;
    }

    xmas.reset();

    // diagonal lookup
    for idx in 0 .. 4 {
        if idx <= row && idx <= col && row >= 4 && col >= 4 {
            if grid[row - idx][col - idx] == xmas.next_token() {
                xmas.set_token(grid[row - idx][col - idx]);
            }
        }
    }

    if xmas.is_complete() {
        result += 1;
    }

    xmas.reset();

    // diagonal lookup
    for idx in 0 .. 4 {
        if idx <= row && idx <= col && row >= 4 && col <= grid[row].len() - 4 {
            if grid[row - idx][col + idx] == xmas.next_token() {
                xmas.set_token(grid[row - idx][col + idx]);
            }
        }
    }

    if xmas.is_complete() {
        result += 1;
    }

    result
}


pub fn ceres_search(grid: &Vec<Vec<char>>) -> u16 {
    let mut result: u16 = 0;

    for (row_nr, row) in grid.iter().enumerate() {
        for (col_nr, chr) in row.iter().enumerate() {
            if chr == &'X' {
                result += ceres_search_sub(grid, row_nr, col_nr);
            }
        }
    }

    result
}
