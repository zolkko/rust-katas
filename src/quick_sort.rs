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
    let (rest, last) = array.split_at_mut(end);
    for i in start .. rest.len() {
        if rest[i] < last[0] {
            rest.swap(pivot_index, i);
            pivot_index += 1;
        }
    }
    array.swap(pivot_index, end);
    pivot_index
}

trait QuickSortable {
    fn quick_sort(&mut self);
}

impl<T: Ord> QuickSortable for [T] {
    fn quick_sort(&mut self) {
        if let Some(e) = self.len().checked_sub(1) {
            quick_sort_rec(&mut self, 0, e);
        }
    }
}