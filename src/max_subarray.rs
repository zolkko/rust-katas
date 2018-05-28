//! Given an array `array` return maximum sum of contiguous elements.
//! Maximum sum of an empty array must be 0.

use std::io::stdin;
use std::cmp::max;
use std::str::FromStr;

/// Naive implementation. It searches for the first positive integer, and then searches for the
/// bigger positive sum of elements.
///
/// If there is no positive elements in the input array, then it returns biggest negative number, or
/// zero if the array is empty.
#[cfg_attr(not(test), allow(dead_code))]
fn simple_search(array: &[i32]) -> i32 {
    let mut maybe_max_negative: Option<i32> = None;
    let mut iter = array.iter();

    while let Some(&value) = iter.next() {
        if value >= 0 {
            let mut acc = value;
            let mut sum = value;

            while let Some(&value) = iter.next() {
                if acc + value < 0 {
                    acc = 0;
                } else {
                    acc += value;
                }

                if sum < acc {
                    sum = acc;
                }
            }

            return sum
        }

        maybe_max_negative = match maybe_max_negative {
            None => Some(value),
            Some(v) if v < value => Some(value),
            Some(_) => maybe_max_negative
        };
    }

    maybe_max_negative.unwrap_or(0)
}

/// Now, there is nothing wrong with summing up negative integers, because the sum of two or more
/// negative integers going to be smaller than any element in the array, moreover one of these
/// integers will be greater or equal than the others.
///
/// We can rewrite `if sum < acc {}` statement as `sum = max(sum, acc)`
///
/// Further, by taking a dynamic programming approach and considering `array[a] -> 'a` and
/// `array[a,b] -> max(b, array[a] + b)` we will ends up with Kanade's algorithm.
fn kadane(array: &[i32]) -> i32 {
    if array.len() == 0 {
        0
    } else {
        let mut sum = array[0];
        let mut acc = array[0];
        for value in array.iter().skip(1) {
            acc = max(*value, acc + value);
            sum = max(sum, acc);
        }
        sum
    }
}


fn main() -> Result<(), std::io::Error> {
    let mut input = String::with_capacity(80);
    stdin().read_line(&mut input)?;

    let array = input.trim().split_whitespace()
        .flat_map(|x| FromStr::from_str(x)).collect::<Vec<i32>>();

    let simple_result = simple_search(&array);
    let kadane_result = kadane(&array);

    println!("Maximum subarray problem, Simple algorithm = {}", simple_result);
    println!("Maximum subarray problem, Kadane algorithm = {}", kadane_result);

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_zero_on_empty_array() {
        assert_eq!(simple_search(&[]), 0);
        assert_eq!(kadane(&[]), 0);
    }

    #[test]
    fn it_can_find_sum_in_general_case() {
        assert_eq!(simple_search(&[-2, -3, 4, -1, 5, -1, -2, 3, -3]), 8);
        assert_eq!(kadane(&[-2, -3, 4, -1, 5, -1, -2, 3, -3]), 8);
    }

    #[test]
    fn it_finds_sum_if_all_elements_are_negative() {
        assert_eq!(simple_search(&[-3, -2, -1, -2, -3]), -1);
        assert_eq!(kadane(&[-3, -2, -1, -2, -3]), -1);
    }

    #[test]
    fn it_finds_sum_if_all_elements_are_positive() {
        assert_eq!(simple_search(&[3, 2, 1, 2, 3]), 11);
        assert_eq!(kadane(&[3, 2, 1, 2, 3]), 11);

    }

    #[test]
    fn it_contains_single_positive_number() {
        assert_eq!(simple_search(&[5]), 5);
        assert_eq!(kadane(&[5]), 5);
    }

    #[test]
    fn it_contains_single_negative_number() {
        assert_eq!(simple_search(&[-10]), -10);
        assert_eq!(kadane(&[-10]), -10);
    }

    #[test]
    fn it_contains_only_zeros() {
        assert_eq!(simple_search(&[0, 0, 0]), 0);
        assert_eq!(kadane(&[0, 0, 0]), 0);
    }

    #[test]
    fn it_contains_single_zero() {
        assert_eq!(simple_search(&[0]), 0);
        assert_eq!(kadane(&[0]), 0);
    }
}
