#[cfg(test)]
mod tests {
    use crate::binary_search;

    #[test]
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        let key = 5;
        let result = binary_search(&arr, &key);
        assert_eq!(result, None);
    }

    #[test]
    fn test_key_found() {
        let arr = [2, 5, 7, 8, 11, 12];
        let key = 11;
        let result = binary_search(&arr, &key);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_key_not_found() {
        let arr = [2, 5, 7, 8, 11, 12];
        let key = 13;
        let result = binary_search(&arr, &key);
        assert_eq!(result, None);
    }

    #[test]
    fn test_first_element() {
        let arr = [2, 5, 7, 8, 11, 12];
        let key = 2;
        let result = binary_search(&arr, &key);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_last_element() {
        let arr = [2, 5, 7, 8, 11, 12];
        let key = 12;
        let result = binary_search(&arr, &key);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_duplicate_elements() {
        let arr = [2, 5, 5, 8, 11, 12];
        let key = 5;
        let result = binary_search(&arr, &key);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_custom_comparison() {
        let arr = [2, 5, 7, 8, 11, 12];
        let key = 11;
        let result = binary_search(&arr, &key);
        assert_eq!(result, Some(4));
    }
}