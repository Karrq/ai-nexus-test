/// Performs an iterative binary search on a sorted slice.
///
/// This function searches for the `target` element within the given `slice`.
/// The `slice` must be sorted in ascending order according to the `Ord` trait.
///
/// # Arguments
///
/// * `slice` - A sorted slice of type `T`.
/// * `target` - The element to search for.
///
/// # Returns
///
/// A `Result` containing:
/// * `Ok(index)` - If the `target` is found, returns the index of the element in the slice.
/// * `Err(())` - If the `target` is not found in the slice.
///
/// # Examples
///
/// ```
/// let sorted_numbers = [2, 4, 6, 8, 10];
/// assert_eq!(binary_search(&sorted_numbers, &6), Ok(2));
/// assert_eq!(binary_search(&sorted_numbers, &5), Err(()));
/// ```
pub fn binary_search<T: Ord>(slice: &[T], target: &T) -> Result<usize, ()> {
    let mut low = 0;
    let mut high = slice.len();

    while low < high {
        let mid = low + (high - low) / 2;

        match slice[mid].cmp(target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Ok(mid),
        }
    }

    Err(())
}
