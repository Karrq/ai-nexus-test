#[cfg(test)]
mod e2e {
    use efficient_binary_search::binary_search;

    #[test]
    fn test_binary_search_found() {
        let sorted_data = vec![2, 5, 7, 8, 11, 12];
        let target = 11;
        let result = binary_search(&sorted_data, &target);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_binary_search_not_found() {
        let sorted_data = vec![2, 5, 7, 8, 11, 12];
        let target = 13;
        let result = binary_search(&sorted_data, &target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_binary_search_empty_slice() {
        let sorted_data: Vec<i32> = vec![];
        let target = 5;
        let result = binary_search(&sorted_data, &target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_binary_search_first_element() {
        let sorted_data = vec![2, 5, 7, 8, 11, 12];
        let target = 2;
        let result = binary_search(&sorted_data, &target);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_binary_search_last_element() {
        let sorted_data = vec![2, 5, 7, 8, 11, 12];
        let target = 12;
        let result = binary_search(&sorted_data, &target);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_binary_search_string() {
        let sorted_data = vec!["apple", "banana", "cherry", "date"];
        let target = "banana";
        let result = binary_search(&sorted_data, &target);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_binary_search_char() {
        let sorted_data = vec!['a', 'b', 'c', 'd'];
        let target = 'c';
        let result = binary_search(&sorted_data, &target);
        assert_eq!(result, Some(2));
    }
}
