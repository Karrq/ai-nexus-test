#[cfg(test)]
mod tests {
    use efficient_search::binary_search;

    #[test]
    fn test_found() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let target = 11;
        let result = binary_search(&sorted_list, target);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_not_found() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let target = 13;
        let result = binary_search(&sorted_list, target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_empty_list() {
        let sorted_list: Vec<i32> = vec![];
        let target = 7;
        let result = binary_search(&sorted_list, target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_first_element() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let target = 2;
        let result = binary_search(&sorted_list, target);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_last_element() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let target = 12;
        let result = binary_search(&sorted_list, target);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_duplicate_elements() {
        let sorted_list = vec![2, 5, 7, 7, 11, 12];
        let target = 7;
        let result = binary_search(&sorted_list, target);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_string_list() {
        let sorted_list = vec!["apple", "banana", "cherry"];
        let target = "banana";
        let result = binary_search(&sorted_list, target);
        assert_eq!(result, Some(1));
    }
}
