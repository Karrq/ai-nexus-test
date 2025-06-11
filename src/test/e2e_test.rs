#[cfg(test)]
mod tests {
    use rust_binary_search_library_2::binary_search;

    #[test]
    fn test_binary_search_found() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let target = 11;
        let result = binary_search(&sorted_list, &target);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_binary_search_not_found() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let target = 13;
        let result = binary_search(&sorted_list, &target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_binary_search_empty_list() {
        let sorted_list: Vec<i32> = Vec::new();
        let target = 13;
        let result = binary_search(&sorted_list, &target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_binary_search_first_element() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let target = 2;
        let result = binary_search(&sorted_list, &target);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_binary_search_last_element() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let target = 12;
        let result = binary_search(&sorted_list, &target);
        assert_eq!(result, Some(5));
    }
}
