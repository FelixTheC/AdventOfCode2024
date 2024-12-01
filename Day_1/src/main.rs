use crate::puzzle_input_const::{LEFT_LIST_2, RIGHT_LIST_2};

mod puzzle_input_const;

fn part1() {
    let mut sorted_left_list = puzzle_input_const::LEFT_LIST;
    let mut sorted_right_list = puzzle_input_const::RIGHT_LIST;

    sorted_left_list.sort();
    sorted_right_list.sort();

    let mut result: u128 = 0;

    for (left, right) in sorted_left_list.iter().zip(sorted_right_list.iter()) {
        let difference = if left > right {
            *left - *right
        } else {
            *right - *left
        } as u128;

        result += difference;
    }

    println!("Total distance: {}", result);
}

fn get_occurrences(val: u32, list: &[u32]) -> u32 {
    let mut count = 0;

    list.iter().for_each(|x| {
        if *x == val {
            count += 1;
        }
    });

    count
}

fn part2() {
    let mut result: u128 = 0;

    for val in LEFT_LIST_2 {
        let left_count = get_occurrences(val, &RIGHT_LIST_2);
        result += (val * left_count) as u128;
    }

    println!("similarity score: {}", result);
}

fn main() {
    part1();
    part2();
}
