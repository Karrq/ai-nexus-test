pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;

        match arr[mid].cmp(target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
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
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, &1), None);
    }
}