use efficient_binary_search::binary_search;

#[test]
fn test_integration_found() {
    let data = vec![1, 3, 5, 7, 9];
    let target = 5;
    let result = binary_search(&data, &target);
    assert_eq!(result, Some(2));
}

#[test]
fn test_integration_not_found() {
    let data = vec![1, 3, 5, 7, 9];
    let target = 2;
    let result = binary_search(&data, &target);
    assert_eq!(result, None);
}
