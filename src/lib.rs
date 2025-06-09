pub fn binary_search<T: PartialOrd + Copy>(arr: &[T], target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2; // Prevent potential overflow

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, 5), None);
    }

    #[test]
    fn test_target_at_beginning() {
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search(&arr, 2), Some(0));
    }

    #[test]
    fn test_target_at_end() {
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search(&arr, 12), Some(5));
    }

    #[test]
    fn test_target_not_present() {
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search(&arr, 9), None);
    }

    #[test]
    fn test_target_in_middle() {
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search(&arr, 8), Some(3));
    }

    #[test]
    fn test_with_float() {
        let arr = [2.0, 5.0, 7.0, 8.0, 11.0, 12.0];
        assert_eq!(binary_search(&arr, 8.0), Some(3));
    }
}
