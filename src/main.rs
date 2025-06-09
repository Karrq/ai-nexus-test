mod lib;

fn main() {
    let sorted_array = vec![2, 5, 7, 8, 11, 12];
    let target = 13;

    match lib::binary_search(&sorted_array, &target) {
        Some(index) => println!("Target {} found at index {}", target, index),
        None => println!("Target {} not found in the array", target),
    }
}
