pub(crate) struct  Report {
    pub(crate) levels: Vec<i32>
}

impl Report {
    pub(crate) fn new() -> Report {
        Report {
            levels: Vec::new()
        }
    }
}

fn read_puzzle_input(use_test_input: bool) -> Vec<Report> {
    let input = if use_test_input {
        include_str!("../test_input.txt")
    } else {
        include_str!("../puzzle_input.txt")
    };

    let mut result: Vec<Report> = Vec::new();

    for line in input.lines() {
        let mut report= Report::new();
        for num in line.split(" ") {
            report.levels.push(num.parse().unwrap());
        }
        result.push(report);
    }

    result
}

#[allow(dead_code)]
pub(crate) fn get_puzzle_input() -> Vec<Report> {
    read_puzzle_input(false)
}

#[allow(dead_code)]
pub(crate) fn get_test_puzzle_input() -> Vec<Report> {
    read_puzzle_input(true)
}