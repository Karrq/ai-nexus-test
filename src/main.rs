use binary_search::binary_search;

fn main() {
    let arr = vec![2, 5, 7, 8, 11, 12];
    let target = 11;

    match binary_search(&arr, target) {
        Some(index) => println!("Target {} found at index {}", target, index),
        None => println!("Target {} not found in the array", target),
    }
}
