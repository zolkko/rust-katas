fn quick_sort_rec<T: Ord>(array: &mut [T], start: usize, end: usize) {
    if start > end {
        return;
    }
    let index = partition(array, start, end);
    if let Some(e) = index.checked_sub(1) {
        quick_sort_rec(array, start, e);
    }
    quick_sort_rec(array, index + 1, end);
}

fn partition<T: Ord>(array: &mut [T], start: usize, end: usize) -> usize {
    let mut pivot_index = start;

    // I must temporary split the array, because there is no other way to explain
    // Rust that array does not reallocate and array[end] element is not modified
    // inside the loop.
    let (rest, last) = array.split_at_mut(end);
    for i in start .. rest.len() {
        if rest[i] < last[0] {
            rest.swap(pivot_index, i);
            pivot_index += 1;
        }
    }

    // Because of NLL I can mutate the array over here.
    array.swap(pivot_index, end);

    pivot_index
}

trait QuickSortable {
    fn quick_sort(&mut self);
}

impl<T: Ord> QuickSortable for [T] {
    fn quick_sort(&mut self) {
        if let Some(e) = self.len().checked_sub(1) {
            quick_sort_rec(self, 0, e);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn sort_empty_vector() {
        let mut empty: Vec<isize> = Vec::new();
        empty.quick_sort();
        assert_eq!(empty, Vec::new());
    }

    #[test]
    fn sort_single_element_vector() {
        let mut array = vec![1];
        array.quick_sort();
        assert_eq!(array, vec![1]);
    }

    #[test]
    fn sort_reverse_vector() {
        let mut array = vec![3, 2, 1, 0];
        array.quick_sort();
        assert_eq!(array, vec![0, 1, 2, 3]);
    }

    #[test]
    fn sort_sorted_vector() {
        let mut array = vec![1, 2, 3];
        array.quick_sort();
        assert_eq!(array, vec![1, 2, 3])
    }

    #[test]
    fn sort_mixed_vector() {
        let mut array = vec![3, 2, 4, 5, 1, 7, 0, 9];
        array.quick_sort();
        assert_eq!(array, vec![0, 1, 2, 3, 4, 5, 7, 9]);
    }
}
