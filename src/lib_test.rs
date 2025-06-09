#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_found() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let key = 11;
        let result = binary_search(&sorted_list, key);
        assert_eq!(result, Ok(4));
    }

    #[test]
    fn test_binary_search_not_found() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let key = 13;
        let result = binary_search(&sorted_list, key);
        assert_eq!(result, Err("Key not found"));
    }

    #[test]
    fn test_binary_search_empty_list() {
        let sorted_list: Vec<i32> = vec![];
        let key = 7;
        let result = binary_search(&sorted_list, key);
        assert_eq!(result, Err("Key not found"));
    }

    #[test]
    fn test_binary_search_first_element() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let key = 2;
        let result = binary_search(&sorted_list, key);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_binary_search_last_element() {
        let sorted_list = vec![2, 5, 7, 8, 11, 12];
        let key = 12;
        let result = binary_search(&sorted_list, key);
        assert_eq!(result, Ok(5));
    }

    #[test]
    fn test_binary_search_duplicate_elements() {
        let sorted_list = vec![2, 5, 7, 7, 11, 12];
        let key = 7;
        let result = binary_search(&sorted_list, key);
        assert_eq!(result, Ok(2));
    }

    // Add more test cases to cover other scenarios

    fn binary_search<T: Ord>(sorted_list: &[T], key: T) -> Result<usize, &'static str> {
        let mut low = 0;
        let mut high = sorted_list.len() as isize - 1;

        while low <= high as usize {
            let mid = (low + high as usize) / 2;
            match sorted_list[mid].cmp(&key) {
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid as isize - 1,
                std::cmp::Ordering::Equal => return Ok(mid),
            }
        }
        Err("Key not found")
    }
}
