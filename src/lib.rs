/// Performs a binary search on a sorted slice of i32.
///
/// # Arguments
///
/// * `list` - A sorted slice of i32.
/// * `target` - The i32 to search for.
///
/// # Returns
///
/// An `Option<usize>` representing the index of the target if found, or `None` if not found.
///
/// # Examples
///
/// ```
/// let list = [2, 5, 7, 8, 11, 12];
/// let target = 13;
/// let result = efficient_search::binary_search(&list, target);
/// assert_eq!(result, None);
///
/// let list = [2, 5, 7, 8, 11, 12];
/// let target = 12;
/// let result = efficient_search::binary_search(&list, target);
/// assert_eq!(result, Some(5));
/// ```
pub fn binary_search(list: &[i32], target: i32) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = list.len();

    if low >= high {
        return None;
    }

    while low < high {
        let mid = low + (high - low) / 2;
        let value = list[mid];

        if value == target {
            return Some(mid);
        } else if value < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    None
}
