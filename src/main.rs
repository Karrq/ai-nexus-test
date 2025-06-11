use binary_search_library::binary_search;

fn main() {
    let sorted_vec = vec![2, 5, 7, 8, 11, 12];

    let target = 13;
    match binary_search(&sorted_vec, &target) {
        Some(index) => println!("Element {} found at index {}", target, index),
        None => println!("Element {} not found", target),
    }

    let target = 5;
    match binary_search(&sorted_vec, &target) {
        Some(index) => println!("Element {} found at index {}", target, index),
        None => println!("Element {} not found", target),
    }
}
