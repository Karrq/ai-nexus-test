#[test]
fn test_binary_search_found() {
    let sorted_list = vec![2, 5, 7, 8, 11, 12];
    let target = 11;
    let result = efficient_search::binary_search(&sorted_list, target);
    assert_eq!(result, Some(4));
}

#[test]
fn test_binary_search_not_found() {
    let sorted_list = vec![2, 5, 7, 8, 11, 12];
    let target = 13;
    let result = efficient_search::binary_search(&sorted_list, target);
    assert_eq!(result, None);
}

#[test]
fn test_binary_search_empty_list() {
    let sorted_list: Vec<i32> = vec![];
    let target = 7;
    let result = efficient_search::binary_search(&sorted_list, target);
    assert_eq!(result, None);
}

#[test]
fn test_binary_search_single_element_found() {
    let sorted_list = vec![7];
    let target = 7;
    let result = efficient_search::binary_search(&sorted_list, target);
    assert_eq!(result, Some(0));
}

#[test]
fn test_binary_search_single_element_not_found() {
    let sorted_list = vec![7];
    let target = 8;
    let result = efficient_search::binary_search(&sorted_list, target);
    assert_eq!(result, None);
}

#[test]
fn test_binary_search_first_element() {
    let sorted_list = vec![2, 5, 7, 8, 11, 12];
    let target = 2;
    let result = efficient_search::binary_search(&sorted_list, target);
    assert_eq!(result, Some(0));
}

#[test]
fn test_binary_search_last_element() {
    let sorted_list = vec![2, 5, 7, 8, 11, 12];
    let target = 12;
    let result = efficient_search::binary_search(&sorted_list, target);
    assert_eq!(result, Some(5));
}
