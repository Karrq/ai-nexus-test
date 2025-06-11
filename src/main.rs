use efficient_search::binary_search;

fn main() {
    let sorted_vec = vec![2, 4, 6, 8, 10];
    let target = 6;
    let result = binary_search(&sorted_vec, &target);
    println!("Index of {} is: {:?}", target, result);

    let target = 7;
    let result = binary_search(&sorted_vec, &target);
    println!("Index of {} is: {:?}", target, result);

    let sorted_strings = vec!["apple", "banana", "cherry", "date"];
    let target_string = "banana";
    let result_string = binary_search(&sorted_strings, &target_string);
    println!("Index of {} is: {:?}", target_string, result_string);

    let empty_vec: Vec<i32> = Vec::new();
    let target_empty = 5;
    let result_empty = binary_search(&empty_vec, &target_empty);
    println!("Index of {} in empty vec is: {:?}", target_empty, result_empty);

    let vec_with_duplicates = vec![1, 2, 2, 3, 4, 4, 5];
    let target_duplicate = 2;
    let result_duplicate = binary_search(&vec_with_duplicates, &target_duplicate);
    println!("Index of {} in vec with duplicates is: {:?}", target_duplicate, result_duplicate);
}
