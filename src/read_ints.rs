fn main() {
    let mut line = String::with_capacity(10);
    std::io::stdin().read_line(&mut line).expect("expect the line of text");

    let input = line.trim_right();

    let sum = input.split(' ')
        .flat_map(|x| x.parse::<i32>().ok())
        .fold(0i32, |a, b| a + b);

    println!("{}", sum);
}