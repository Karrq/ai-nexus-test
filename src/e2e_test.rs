#[cfg(test)]
mod tests {
    use binary_search_rs::binary_search;

    #[test]
    fn test_binary_search_found() {
        let arr = [2, 5, 7, 8, 11, 12];
        let target = 11;
        let result = binary_search(&arr, target);
        assert_eq!(result, Ok(4));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = [2, 5, 7, 8, 11, 12];
        let target = 13;
        let result = binary_search(&arr, target);
        assert_eq!(result, Err("Not found"));
    }

    #[test]
    fn test_binary_search_empty_array() {
        let arr: [i32; 0] = [];
        let target = 5;
        let result = binary_search(&arr, target);
        assert_eq!(result, Err("Not found"));
    }

    #[test]
    fn test_binary_search_first_element() {
        let arr = [2, 5, 7, 8, 11, 12];
        let target = 2;
        let result = binary_search(&arr, target);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_binary_search_last_element() {
        let arr = [2, 5, 7, 8, 11, 12];
        let target = 12;
        let result = binary_search(&arr, target);
        assert_eq!(result, Ok(5));
    }
}
