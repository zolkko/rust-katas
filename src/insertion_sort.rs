//! This is the simplest form of exchange sorting.
//!
//! In the best case it's complexity is O(n).
//! Because it performs (see inner while loop) pairs' comparision.
//! Consider vector `[a, b, c, d]` where `(a < b)`, `(b < c)`, `(c < d)`.


fn insertion_sort<T: Ord>(array: &mut [T]) {
    for i in 1..array.len() {
        // the index variable cannot be less than zero, because i starts with one and thus subtraction is always the valid operation here.
        let mut index = i - 1;

        while array[index] > array[index + 1] {
            array.swap(index, index + 1);

            if index != 0 {
                index -= 1;
            } else {
                break;
            }
        }
    }
}

trait InsertionSort {
    fn insertion_sort(&mut self);
}

impl<T: Ord> InsertionSort for [T] {
    fn insertion_sort(&mut self) {
        insertion_sort(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn sort_empty_vector() {
        let mut v = Vec::<isize>::new();
        v.insertion_sort();
        assert_eq!(v, vec![]);
    }

    #[test]
    fn sort_single_element_vector() {
        let mut v = vec![1];
        v.insertion_sort();
        assert_eq!(v, vec![1]);
    }

    #[test]
    fn sort_sorted_vector() {
        let mut v = vec![1, 2, 3, 4];
        v.insertion_sort();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn sort_reverse_vector() {
        let mut v = vec![4, 3, 2, 1];
        v.insertion_sort();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn sort_mixed_vector() {
        let mut v = vec![4, 1, 3, 2];
        v.insertion_sort();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }
}
