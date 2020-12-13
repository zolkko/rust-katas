extern crate csv;
extern crate num;
extern crate rand;
extern crate nalgebra;
extern crate gnuplot;

use std::io;
use std::path::Path;
use csv::Reader;
use csv::Writer;
use rand::distributions::{Range, IndependentSample};
use nalgebra::{DMat};
use nalgebra::{Transpose, Iterable};

use gnuplot::{Figure, Caption, PointSymbol, Color, PointSize};

fn read_csv_file(file_name: &str) -> io::Result<(Vec<f64>, Vec<f64>)> {
    let csv_file_path = Path::new(file_name);
    if csv_file_path.exists() {
        let mut csv_reader = Reader::from_file(csv_file_path).unwrap().has_headers(false);
        let mut vec_x: Vec<f64> = Vec::new();
        let mut vec_y: Vec<f64> = Vec::new();
        for record in csv_reader.decode() {
            let (x, y): (f64, f64) = record.unwrap();
            vec_x.push(x);
            vec_y.push(y);
        }
        Ok((vec_x, vec_y))
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "file does not eixts"))
    }
}

fn write_cvs_file(file_name: &str, vec_x: &[f64], vec_y: &[f64]) -> io::Result<()> {
    let csv_file_path = Path::new(file_name);
    let mut csv_writer = match Writer::from_file(csv_file_path) {
        Ok(x) => x,
        Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "failed to write the file"))
    };
    for record in vec_x.iter().zip(vec_y) {
        match csv_writer.encode(record) {
            Ok(_) => (),
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "failed to encode data"))
        }
    }
    Ok(())
}

fn linear_least_squares(x_vec: &[f64], y_vec: &[f64]) -> (f64, f64) {
    let len = x_vec.len() as f64;
    let sum_x = x_vec.iter().fold(0.0f64, |acc, &x| acc + x);
    let sum_y = y_vec.iter().fold(0.0f64, |acc, &y| acc + y);
    let sum_x2 = x_vec.iter().fold(0.0f64, |acc, &x| x.powi(2) + acc);
    let sum_xy = x_vec.iter().zip(y_vec.iter()).map(|(x, y)| x * y).fold(0.0f64, |acc, i| acc + i);

    let a = ((len * sum_xy) - (sum_x * sum_y)) / (len * sum_x2 - sum_x.powi(2));
    let b = (sum_y - (a * sum_x)) / len;

    (a, b)
}

fn min(vec: &[f64]) -> Result<f64, ()> {
    let result = vec.iter().cloned().fold(f64::NAN, f64::min);
    if result.is_nan() {
        Err(())
    } else {
        Ok(result)
    }
}

fn max(vec: &[f64]) -> Result<f64, ()> {
    let result = vec.iter().cloned().fold(f64::NAN, f64::max);
    if result.is_nan() {
        Err(())
    } else {
        Ok(result)
    }
}

fn generate_data(n: usize, bias: f64, variance: f64) -> (Vec<f64>, Vec<f64>) {
    let mut x: Vec<f64> = Vec::with_capacity(n);
    let mut y: Vec<f64> = Vec::with_capacity(n);

    let between = Range::new(-1f64, 1f64);
    let mut rnd = rand::thread_rng();

    for i in 0..n {
        let j = i as f64;
        x.push(j);
        y.push((j + bias) +  between.ind_sample(&mut rnd) * variance);
    }

    (x, y)
}

#[allow(dead_code)]
fn batch_gradient_descent(theta_vals: &[f64], vec_x: &[f64], vec_y: &[f64], data_len: usize, step: f64, iters: usize) -> Vec<f64>
{
    let m = theta_vals.len();
    // (rows, columns)
    let mut x: DMat<f64> = DMat::new_zeros(data_len, m);
    let mut y: DMat<f64> = DMat::new_zeros(data_len, 1);

    for i in 0..data_len {
        y[(i, 0)] = vec_y[i];
        x[(i, 0)] = 1f64;
        let sample_x = vec_x[i];
        for j in 1..m {
            x[(i, j)] = sample_x.powi(j as i32);
        }
    }

    let xt: DMat<f64> = x.transpose();
    let mut theta: DMat<f64> = DMat::from_row_vec(m, 1, &theta_vals);

    let coef = step * (1f64 / (data_len as f64));

    for _ in 0..iters {
        let hypothesis = x.clone() * theta.clone();
        let loss = hypothesis - y.clone();
        theta = theta.clone() - (xt.clone() * loss) * coef;
    }

    theta.as_vec().iter().cloned().collect::<Vec<f64>>()
}

fn poly_func(x: f64) -> f64 {
    // -0.08f64 * x.powi(2) + 3f64 * x - 1f64
    0.1f64 * x.powi(2)
}

fn generate_poly_dots<F>(start_x: f64, end_x: f64, steps: usize, variance: f64, mut func: F) -> Option<(Vec<f64>, Vec<f64>)>
    where F: FnMut(f64) -> f64
{
    let between = Range::new(-1f64, 1f64);
    let mut rnd = rand::thread_rng();

    let step: f64 = (end_x - start_x) / (steps as f64);
    let mut x: f64 = start_x;
    let mut vec_x: Vec<f64> = Vec::with_capacity(steps);
    let mut vec_y: Vec<f64> = Vec::with_capacity(steps);
    for _ in 0..steps {
        vec_x.push(x);
        vec_y.push(func(x) + between.ind_sample(&mut rnd) * variance);
        x += step;
    }
    Some((vec_x, vec_y))
}

#[allow(dead_code)]
fn draw_poly(coef: &[f64], from: f64, to: f64, steps: usize) -> Option<(Vec<f64>, Vec<f64>)>
{
    if steps <= 1 || coef.len() == 0 {
        None
    } else {
        let mut vec_x: Vec<f64> = Vec::with_capacity(steps);
        let mut vec_y: Vec<f64> = Vec::with_capacity(steps);
        let step = (to - from) / ((steps - 1) as f64);
        let mut x = from;
        for _ in 0..steps {
            vec_x.push(x);
            vec_y.push(coef.iter().enumerate().fold(0.0f64, |acc, (i, w)| acc + (w * x.powi(i as i32))));
            x += step;
        }
        Some((vec_x, vec_y))
    }
}

static DATA_FILE: &'static str = "/Users/zolkko/projects/rsample/data.txt";

fn main() {
    let (data_x, data_y) = match read_csv_file(DATA_FILE) {
        Ok(x) => x,
        Err(_) => {
            let data = generate_data(50, 0.1f64, 8.0f64);
            write_cvs_file(DATA_FILE, &data.0, &data.1).expect("Failed to write a file");
            data
        }
    };

    let min_x = min(&data_x).unwrap();
    let max_x = max(&data_x).unwrap();

    let (ls_a, ls_b) = linear_least_squares(&data_x, &data_y);
    let (ls_x, ls_y) = draw_poly(&[ls_b, ls_a], min_x, max_x, 2).unwrap();

    let theta = batch_gradient_descent(&[0f64, 0f64], &data_x, &data_y, data_x.len(), 0.0005, 1000);
    let (grad_x, grad_y) = draw_poly(&theta[..], min_x, max_x, 2).unwrap();

    let (poly_x, poly_y) = generate_poly_dots(0.0, 30.0, 200, 4.0, poly_func).unwrap();
    let pmin_x = min(&poly_x).unwrap();
    let pmax_x = max(&poly_x).unwrap();

    let theta2 = batch_gradient_descent(&[0f64, 0f64, 0f64, 0f64], &poly_x, &poly_y, poly_x.len(), 0.000000005, 1000);
    let (grad2_x, grad2_y) = draw_poly(&theta2[..], pmin_x, pmax_x, 100).unwrap();

    let mut fg = Figure::new();
    fg.set_terminal("qt", "").clear_axes();
    fg.axes2d()
        .points(&data_x[..], &data_y[..],
            &[Caption("Data Points"), PointSymbol('o'), Color("#0000ff"), PointSize(1.5)])
        .lines(&ls_x[..], &ls_y[..],
            &[Caption("Linear Least Squares Fitting"), Color("#ff0000")])
        .lines(&grad_x[..], &grad_y[..],
            &[Caption("Batch Gradient Descent Fitting"), Color("#0000ff")])
        .points(&poly_x[..], &poly_y[..],
            &[Caption("Experimental Poly Function"), Color("#000000"), PointSize(0.8), PointSymbol('o')])
        .lines(&grad2_x[..], &grad2_y[..],
                &[Caption("Batch Gradient Descent Fitting"), Color("#00ff00")])
                ;
    fg.show();
}
