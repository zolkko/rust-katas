#[macro_use]
extern crate ndarray;


fn main() {
    let mut line = String::with_capacity(10);
    std::io::stdin().read_line(&mut line).expect("expect the line of text");

    let input = line.trim_right();

    let sum = input.split(' ')
        .flat_map(|x| x.parse::<i32>().ok())
        .fold(0i32, |a, b| a + b);

    println!("{}", sum);
}

#[cfg(test)]
mod tests {

    use ndarray::prelude::*;
    use ndarray::ShapeBuilder;


    #[test]
    fn demo() {

        let mut a = ArrayD::<f64>::zeros(IxDyn(&[3, 2, 2]));

        a[[0, 0, 0]] = 2.0;
        a[[1, 1, 0]] = 3.0;
        a[[2, 0, 0]] = 7.0;
        a[[2, 1, 1]] = 7.0;

        for mut sub_levels in a.axis_iter_mut(Axis(0)) {
            let level_sum = sub_levels.sum();
            println!("{:?}\nsum:{:?}\n\n", sub_levels, level_sum);
        }






        println!("+++++++++++");

        panic!()
    }

}