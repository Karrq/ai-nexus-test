/// Performs a binary search on a sorted slice to find the index of a target element.
///
/// # Arguments
///
/// * `arr` - A sorted slice of elements that implement the `Ord` trait.
/// * `target` - The element to search for.
///
/// # Returns
///
/// An `Option<usize>` representing the index of the target element if found, or `None` if not found.
///
/// # Examples
///
/// ```
/// let arr = [2, 5, 7, 8, 11, 12];
/// let target = 13;
/// let result = efficient_search::binary_search(&arr, &target);
/// assert_eq!(result, None);
/// ```
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        match arr[mid].cmp(target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid - 1,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn test_binary_search_found() {
        let arr = [2, 5, 7, 8, 11, 12];
        let target = 11;
        let result = binary_search(&arr, &target);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = [2, 5, 7, 8, 11, 12];
        let target = 13;
        let result = binary_search(&arr, &target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_binary_search_empty_array() {
        let arr: [i32; 0] = [];
        let target = 7;
        let result = binary_search(&arr, &target);
        assert_eq!(result, None);
    }
}
