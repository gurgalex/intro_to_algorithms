#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_descending_insertion_sort() {
        let mut arr = [3, 2, 9, 0, 3, 1];
        insertion_sort_descending(&mut arr);
        assert_eq!([9, 3, 3, 2, 1, 0], arr);
    }

    #[test]
    fn test_insert_sort_descending_single() {
        let mut arr = [-2];
        insertion_sort(&mut arr);
        assert_eq!([-2], arr);
    }
}

/// Sort an array in-place, worst case (O^2)
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let (mut i, len) = (1, arr.len());
    while i < len {
        // Outer loop (n)
        let mut j = i; // The sorted elements behind position i
        // Two elements are not present on the first iteration, skip swap
        while j > 0 && arr[j - 1] > arr[j] {
            // inner loop of first (n * n)
            arr.swap(j, j - 1);
            j -= 1;
        }
        i += 1; // Advance index of array
    }
    // n * n = n^2
}

pub fn insertion_sort_descending<T: Ord>(arr: &mut [T]) {
    let (mut i, len) = (1, arr.len());
    while i < len {
        // Outer loop (n)
        let mut j = i; // The sorted elements behind position i
        // Two elements are not present on the first iteration, skip swap
        while j > 0 && arr[j - 1] < arr[j] {
            // inner loop of first (n * n)
            arr.swap(j, j - 1);
            j -= 1;
        }
        i += 1; // Advance index of array
    }
    // n * n = n^2
}
