use binary_search::binary_search;

fn main() {
    let sorted_numbers = [2, 4, 6, 8, 10];

    // Test case 1: Target found
    match binary_search(&sorted_numbers, &6) {
        Ok(index) => println!("Target 6 found at index: {}", index),
        Err(_) => println!("Target 6 not found"),
    }

    // Test case 2: Target not found
    match binary_search(&sorted_numbers, &5) {
        Ok(index) => println!("Target 5 found at index: {}", index),
        Err(_) => println!("Target 5 not found"),
    }

    // Test case 3: Empty slice
    let empty_slice: [i32; 0] = [];
    match binary_search(&empty_slice, &5) {
        Ok(index) => println!("Target 5 found at index: {}", index),
        Err(_) => println!("Target 5 not found"),
    }

    // Test case 4: Strings
    let sorted_strings = ["apple", "banana", "cherry", "date"];
    match binary_search(&sorted_strings, &"banana") {
        Ok(index) => println!("Target 'banana' found at index: {}", index),
        Err(_) => println!("Target 'banana' not found"),
    }

    // Test case 5: Strings not found
    match binary_search(&sorted_strings, &"grape") {
        Ok(index) => println!("Target 'grape' found at index: {}", index),
        Err(_) => println!("Target 'grape' not found"),
    }
}
