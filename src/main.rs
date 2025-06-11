se efficient_search::binary_search;

fn main() {
    let list = [2, 5, 7, 8, 11, 12];
    let target = 13;
    let result = binary_search(&list, target);
    println!("Target {} found at index: {:?}", target, result);

    let target = 12;
    let result = binary_search(&list, target);
    println!("Target {} found at index: {:?}", target, result);

    let list = [];
    let target = 12;
    let result = binary_search(&list, target);
    println!("Target {} found at index: {:?}", target, result);
}
