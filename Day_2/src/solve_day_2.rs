use crate::read_puzzle_input;

const MAX_LEVEL_DIFFERENCE: i8 = 3;

#[allow(dead_code)]
fn problem_dampener(current_level: i32, next_level: i32, is_decreasing: bool) -> bool {
    let diff = current_level - next_level;
    (current_level > next_level) != is_decreasing ||
        diff.abs() < 1 ||
        diff.abs() > MAX_LEVEL_DIFFERENCE as i32
}

pub fn solve_day_2(enable_problem_dampener: bool) -> u64 {
    let mut safe_reports: u64 = 0;
    let reports = read_puzzle_input::get_puzzle_input();

    for report in reports {
        let mut is_safe = true;
        let mut problem_dumper_used = false;
        let is_decreasing = report.levels[0] > report.levels[1];

        let mut next_idx: usize = 1;
        for level in &report.levels[0 .. report.levels.len() - 1] {
            let next_level = report.levels[next_idx];
            let diff = level - next_level;
            if (*level > next_level) != is_decreasing ||
                diff.abs() < 1 ||
                diff.abs() > MAX_LEVEL_DIFFERENCE as i32 {

                if enable_problem_dampener && !problem_dumper_used && next_idx > 1 {
                    let previous_idx = next_idx - 2;
                    if !problem_dampener(report.levels[previous_idx], report.levels[next_idx], is_decreasing) {
                        is_safe = false;
                        break;
                    } else {
                        problem_dumper_used = true;
                    }
                } else {
                    is_safe = false;
                    break;
                }

            }
            next_idx += 1;
        }

        if is_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}
