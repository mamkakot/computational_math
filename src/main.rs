extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use std::fs::File;
use ndarray::prelude::*;
use ndarray_csv::{Array2Reader};
use csv::ReaderBuilder;
use ndarray::Array2;
use num::complex::Complex;

fn main() {
    let file = File::open("input.csv").unwrap();
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<f64> = reader.deserialize_array2_dynamic().unwrap();

    println!("{}", array_read);

    // получение последнего столбца в отдельный массив
    let last_column_index = array_read.ncols() - 1;

    let (matrix, last_column) = array_read.view().split_at(Axis(1), last_column_index);

    let last_column = last_column.to_shape(3).unwrap();
    println!("last column {}", last_column);

    // проверка на квадратность
    if !matrix.is_square() {
        println!("Matrix should be squared!");
        return;
    }

    // проверка матрицы на симметричность
    if matrix != matrix.t() {
        println!("Matrix should be symmetric!");
        return;
    }

    let mut g: Array2<Complex<f64>> = Array2::zeros((matrix.len(), matrix.len()));

    for ((i, j), value) in matrix.indexed_iter() {
        if i == j {
            if (i == 0) {
                let g_1_1 = if (*value > 0.0) { Complex::new(value.sqrt(), 0.0) } else { Complex::new(0.0, value.sqrt()) };
                g[[i, j]] = g_1_1;
                continue;
            }
            let g_i_i = (value - squared_sum_under_main_diagonal(matrix)).sqrt();
            g[[i, i]] = g_i_i;
            continue;
        }
        if j > i {
            g[[i, j]] = Complex::new(0.0, 0.0);
            continue;
        }
        println!("{}", value);
        g[[i, j]] = Complex::new(*value - g.iter().map(|el| el.powi(2)).sum::<f64>(), 0.0);
    }
    println!("{:?}", g);
}

fn squared_sum_under_main_diagonal(matrix: ArrayView<f64, Ix2>) -> Complex<f64> {
    let mut sum = Complex::new(0.0, 0.0);
    for i in 0..(matrix.dim().0) {
        for j in 0..i {
            println!("{}", i);
            println!("{}", j);
            sum += matrix[[i, j]].powi(2);
        }
    }
    sum
}