/// Performs an iterative binary search on a sorted slice.
///
/// # Arguments
///
/// * `slice` - The sorted slice to search.
/// * `target` - The element to search for.
///
/// # Returns
///
/// An `Option<usize>` containing the index of the target element if found, or `None` if not found.
///
/// # Examples
///
/// ```
/// let sorted_vec = vec![2, 4, 6, 8, 10];
/// let target = 6;
/// let result = efficient_search::binary_search(&sorted_vec, &target);
/// assert_eq!(result, Some(2));
///
/// let target = 7;
/// let result = efficient_search::binary_search(&sorted_vec, &target);
/// assert_eq!(result, None);
/// ```
pub fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = slice.len();

    while low < high {
        let mid = low + (high - low) / 2;

        match slice[mid].cmp(target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    None
}
