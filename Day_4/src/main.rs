use Day_4::{ceres_x_search, ceres_search, read_test_input};

fn main() {
    println!("Result {}", ceres_search(&read_test_input()));
    println!("Result {}", ceres_x_search(&read_test_input()));
}
