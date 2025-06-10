/// Performs a binary search on a sorted slice.
///
/// # Examples
///
/// ```
/// let sorted_array = [2, 5, 7, 8, 11, 12];
/// let index = binary_search::binary_search(&sorted_array, &11).unwrap();
/// assert_eq!(index, 4);
/// ```
///
/// # Errors
///
/// Returns `Err(())` if the element is not found in the slice.
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Result<usize, ()> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;

        match arr[mid].cmp(target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Ok(mid),
        }
    }

    Err(())
}
