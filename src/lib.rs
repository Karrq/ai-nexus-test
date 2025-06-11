pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match arr[mid].cmp(target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search(&arr, &13), None);
        assert_eq!(binary_search(&arr, &12), Some(5));
        assert_eq!(binary_search(&arr, &0), None);
        assert_eq!(binary_search(&arr, &2), Some(0));
        assert_eq!(binary_search(&arr, &5), Some(1));
    }

    #[test]
    fn test_binary_search_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, &1), None);
    }

    #[test]
    fn test_binary_search_one_element() {
        let arr = [5];
        assert_eq!(binary_search(&arr, &5), Some(0));
        assert_eq!(binary_search(&arr, &1), None);
    }
}
