use std::io::BufRead;
use std::error::Error;
use std::collections::HashSet as Set;
use std::ops::Sub;
use std::hash::Hash;
use std::borrow::Borrow;


/// The function returns true if there are two numbers in the `numbers` slice
/// which sum is equal to `sum`.
fn find_two_number<T>(numbers: &[T], sum: T) -> bool
where
    T: Sub + Hash + Copy + Eq,
    // <T as Sub>::Output: T,
    for<'a> &'a T: Borrow<<T as Sub>::Output>,
    <T as Sub>::Output: Hash + Copy + Eq,
{
    let mut seen = Set::new();
    for number in numbers {
        let diff = sum - *number;
        if seen.contains(&diff) {
            return true;
        }
        seen.insert(number);
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut search = String::new();
    let mut numbers = String::new();

    let stdin = std::io::stdin();
    let mut guard = stdin.lock();
    guard.read_line(&mut search)?;
    guard.read_line(&mut numbers)?;
    drop(guard);

    let s = search.trim().parse::<isize>()?;
    let nums = numbers.trim().split(' ').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();

    if nums.is_empty() {
        println!("non empty list of numbers expected");
    } else {
        if find_two_number(&nums, s) {
            println!("found the pair of number");
        } else {
            println!("the pair of number is not in the list");
        }
    }

    Ok(())
}