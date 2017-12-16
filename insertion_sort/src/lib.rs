#[cfg(test)]
mod tests {
    use super::insertion_sort;
    #[test]
    fn test_insertion_sort() {
        let mut arr = [5, 4, 3, 2, 9, 8];
        insertion_sort(&mut arr);
        assert_eq!([2, 3, 4, 5, 8, 9], arr);
    }

    #[test]
    fn test_insert_sort_single_value() {
        let mut arr = [8];
        insertion_sort(&mut arr);
        assert_eq!([8], arr);
    }
}

/// Sort an array in-place given, worst case (O^2)
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let (mut i, len) = (1, arr.len());
    while i < len { // One loop (n)
        let mut j = i;
        while j > 0 && arr[j-1] > arr[j] { // 2nd loop (n)
            arr.swap(j, j-1);
            j -= 1;
        }
        i += 1;
    }
    // n * n = n^2
}