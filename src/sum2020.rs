const TARGET: usize = 2020;


fn find_numbers(data: Vec<usize>) -> Option<(usize, usize)> {
    let mut sorted_data: Vec<_> = data.into_iter().filter(|&x| x < 2020).collect();
    sorted_data.sort();

    for i in 0..(sorted_data.len() - 1) {
        for j in (i + 1)..sorted_data.len() {
            let res = sorted_data[i] + sorted_data[j];
            if res == TARGET {
                return Some((sorted_data[i], sorted_data[j]))
            } else if res > TARGET {
                return None;
            }
        }
    }
    None
}

fn main() {
    let data = vec![1721, 979, 366, 299, 675, 1456];
    if let Some((a, b)) = find_numbers(data) {
        println!("Found numbers: {} and {}", a, b);
    } else {
        println!("There is no numbers that sums up to {}", TARGET);
    }
}