
pub fn read_test_input() -> Vec<Vec<char>> {
    // let input = include_str!("../test_input.txt");
    let input = include_str!("../puzzle_input.txt");

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

    fn complete_result(&self) -> u16 {
        if self.x && self.m && self.a && self.s { 1 }
        else { 0 }
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.x = false;
        self.m = false;
        self.a = false;
        self.s = false;
    }
}

fn find_to_the_right(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u16 {
    let mut xmas = Xmas::new();
    if col <= grid[row].len() - 4 {
        for idx in 0 .. 4 {
            if grid[row][col + idx] == xmas.next_token() {
                xmas.set_token(grid[row][col + idx]);
            } else {
                break;
            }
        }
    }
    xmas.complete_result()
}

fn find_to_the_left(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u16 {
    let mut xmas = Xmas::new();
    if col >= 3 {
        // look horizontal to the left side
        for idx in 0 .. 4 {
            if grid[row][col - idx] == xmas.next_token() {
                xmas.set_token(grid[row][col - idx]);
            } else {
                break;
            }
        }
    }
    xmas.complete_result()
}

fn find_top_to_bottom(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u16 {
    let mut xmas = Xmas::new();
    // look vertical down
    if row <= grid.len() - 4 {
        for idx in 0 .. 4 {
            if grid[row + idx][col] == xmas.next_token() {
                xmas.set_token(grid[row + idx][col]);
            } else {
                break;
            }
        }
    }
    xmas.complete_result()
}

fn find_bottom_to_top(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u16 {
    let mut xmas = Xmas::new();
    // look vertical up
    if row >= 3 {
        for idx in 0 .. 4 {
            if grid[row - idx][col ] == xmas.next_token() {
                xmas.set_token(grid[row - idx][col]);
            } else {
                break;
            }
        }
    }
    xmas.complete_result()
}

fn find_upper_left_to_bottom_right(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u16 {
    let mut xmas = Xmas::new();
    // diagonal lookup
    for idx in 0 .. 4 {
        if row + idx < grid.len() && col + idx < grid[row].len() {
            if grid[row + idx][col + idx] == xmas.next_token() {
                xmas.set_token(grid[row + idx][col + idx]);
            } else {
                break;
            }
        }
    }
    xmas.complete_result()
}

fn find_upper_left_to_bottom_left(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u16 {
    let mut xmas = Xmas::new();
    // diagonal lookup
    for idx in 0 .. 4 {
        if row + idx < grid.len() && idx <= col {
            if grid[row + idx][col - idx] == xmas.next_token() {
                xmas.set_token(grid[row + idx][col - idx]);
            } else {
                break;
            }
        }
    }
    xmas.complete_result()
}

fn find_bottom_left_to_upper_left(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u16 {
    let mut xmas = Xmas::new();
    // diagonal lookup
    for idx in 0 .. 4 {
        if idx <= row && idx <= col && row >= 3 && col >= 3 {
            if grid[row - idx][col - idx] == xmas.next_token() {
                xmas.set_token(grid[row - idx][col - idx]);
            } else {
                break;
            }
        }
    }
    xmas.complete_result()
}

fn find_bottom_left_to_upper_right(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u16 {
    let mut xmas = Xmas::new();
    // diagonal lookup
    for idx in 0 .. 4 {
        if idx <= row && col <= grid[row].len() - 4 {
            if grid[row - idx][col + idx] == xmas.next_token() {
                xmas.set_token(grid[row - idx][col + idx]);
            } else {
                break;
            }
        }
    }
    xmas.complete_result()
}

fn ceres_search_sub(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u16 {
    let mut result: u16 = 0;

    result += find_to_the_right(grid, row, col);
    result += find_to_the_left(grid, row, col);
    result += find_top_to_bottom(grid, row, col);
    result += find_bottom_to_top(grid, row, col);
    result += find_upper_left_to_bottom_right(grid, row, col);
    result += find_upper_left_to_bottom_left(grid, row, col);
    result += find_bottom_left_to_upper_left(grid, row, col);
    result += find_bottom_left_to_upper_right(grid, row, col);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_from_left_to_right() {
        {
            let input = vec![vec!['M','M','M','S','X','X','M','A','S','M']];

            assert_eq!(find_to_the_right(&input, 0, 4), 0);
            assert_eq!(find_to_the_right(&input, 0, 5), 1);
        }
        {
            let input = vec![vec!['M','M','M','S','X','M','X','M','A','S']];
            assert_eq!(find_to_the_right(&input, 0, 6), 1);
        }
        {
            let input = vec![vec!['X','M','A','S','M','M','M','S','X','M']];
            assert_eq!(find_to_the_right(&input, 0, 0), 1);
        }
    }

    #[test]
    fn test_read_from_right_to_left() {
        {
            let input = vec![vec!['M','M','M','S','X','X','M','A','S','M']];
            assert_eq!(find_to_the_left(&input, 0, 4), 0);
            assert_eq!(find_to_the_left(&input, 0, 5), 0);
        }
        {
            let input = vec![vec!['S','A','M','X','M','M','S','A','M','X']];
            assert_eq!(find_to_the_left(&input, 0, 3), 1);
            assert_eq!(find_to_the_left(&input, 0, 9), 1);
        }
        {
            let input = vec![vec!['M','M','S','S','A','M','X','A','M','X']];
            assert_eq!(find_to_the_left(&input, 0, 6), 1);
            assert_eq!(find_to_the_left(&input, 0, 9), 0);
        }
    }

    #[test]
    fn test_read_from_top_to_bottom() {
        {
            let input = vec![
                vec!['M','X','S'],
                vec!['X','M','A'],
                vec!['S','A','M'],
                vec!['A','S','S']
            ];

            assert_eq!(find_top_to_bottom(&input, 0, 0), 0);
            assert_eq!(find_top_to_bottom(&input, 0, 1), 1);
            assert_eq!(find_top_to_bottom(&input, 0, 2), 0);
        }
        {
            let input = vec![
                vec!['A','S','S'],
                vec!['X','M','A'],
                vec!['M','X','S'],
                vec!['X','M','A'],
                vec!['S','A','M'],
                vec!['A','S','S'],
                vec!['M','X','S']
            ];

            assert_eq!(find_top_to_bottom(&input, 0, 1), 0);
            assert_eq!(find_top_to_bottom(&input, 1, 1), 0);
            assert_eq!(find_top_to_bottom(&input, 2, 1), 1);
        }
    }

    #[test]
    fn test_read_from_bottom_to_top() {
        {
            let input = vec![
                vec!['M','S','S'],
                vec!['X','A','A'],
                vec!['S','M','M'],
                vec!['A','X','S']
            ];

            assert_eq!(find_bottom_to_top(&input, 3, 0), 0);
            assert_eq!(find_bottom_to_top(&input, 3, 1), 1);
            assert_eq!(find_bottom_to_top(&input, 3, 2), 0);
        }

        {
            let input = vec![
                vec!['A','S','S'],
                vec!['X','A','A'],
                vec!['M','M','S'],
                vec!['X','X','A'],
                vec!['S','A','M'],
                vec!['A','S','S'],
                vec!['M','X','S']
            ];

            assert_eq!(find_bottom_to_top(&input, 3, 0), 0);
            assert_eq!(find_bottom_to_top(&input, 6, 1), 0);
            assert_eq!(find_bottom_to_top(&input, 3, 1), 1);
        }
    }

    #[test]
    fn test_find_upper_left_to_bottom_right()
    {
        {
            let input = vec![
                vec!['X','S','S','M'],
                vec!['X','M','A','S'],
                vec!['S','M','A','X'],
                vec!['A','X','S','S']
            ];

            assert_eq!(find_upper_left_to_bottom_right(&input, 0, 0), 1);
            assert_eq!(find_upper_left_to_bottom_right(&input, 1, 0), 0);
        }
        {
            let input = vec![
                vec!['X','S','S','M','X','S','S','M'],
                vec!['X','M','A','S','X','M','A','S'],
                vec!['S','M','A','X','S','M','A','X'],
                vec!['X','S','S','X','X','S','S','M'], //<--
                vec!['X','M','A','S','M','M','A','S'],
                vec!['S','M','A','X','S','A','A','X'],
                vec!['A','X','S','S','A','X','S','S']
            ];

            assert_eq!(find_upper_left_to_bottom_right(&input, 3, 3), 1);
            assert_eq!(find_upper_left_to_bottom_right(&input, 0, 0), 0);
        }
    }

    #[test]
    fn test_find_upper_left_to_bottom_left()
    {
        let input = vec![
            vec!['X','S','S','M','X','S','S','X'],
            vec!['X','M','A','S','X','M','M','S'],
            vec!['S','M','A','X','S','A','A','X'],
            vec!['X','A','M','X','S','S','S','M'],
            vec!['X','A','M','S','M','A','A','S'],
            vec!['S','M','A','X','S','A','M','X'], // <--
            vec!['A','X','S','S','A','X','S','X']
        ];

        assert_eq!(find_upper_left_to_bottom_left(&input, 2, 3), 1);
        assert_eq!(find_upper_left_to_bottom_left(&input, 0, 7), 1);
    }

    #[test]
    fn test_find_bottom_left_to_upper_left()
    {
        let input = vec![
            vec!['X','S','S','M','X','S','S','S'],
            vec!['S','M','A','S','X','M','A','S'],
            vec!['S','A','A','X','S','M','A','X'],
            vec!['X','A','M','S','S','S','S','M'],
            vec!['X','A','A','X','M','A','A','S'],
            vec!['S','M','A','X','S','A','M','X'], // <--
            vec!['X','X','S','S','A','X','S','X']
        ];

        assert_eq!(find_bottom_left_to_upper_left(&input, 6, 7), 1);
        assert_eq!(find_bottom_left_to_upper_left(&input, 4, 3), 1);
    }

    #[test]
    fn test_find_bottom_left_to_upper_right()
    {
        let input = vec![
            vec!['X','S','S','M','X','S','S','S'],
            vec!['S','M','A','S','X','M','A','S'],
            vec!['S','A','A','X','S','M','A','X'],
            vec!['X','A','M','S','X','S','S','M'],
            vec!['X','A','A','X','M','A','A','S'],
            vec!['S','M','A','X','S','A','M','X'], // <--
            vec!['X','X','S','S','A','X','S','X']
        ];

        assert_eq!(find_bottom_left_to_upper_right(&input, 6, 0), 1);
        assert_eq!(find_bottom_left_to_upper_right(&input, 3, 4), 1);
    }
}
