extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use std::fs::File;
use ndarray::prelude::*;
use ndarray_csv::{Array2Reader};
use csv::ReaderBuilder;
use ndarray::Array2;

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

    let mut g: Vec<f64> = Vec::new();

    for ((i, j), value) in matrix.indexed_iter() {
        if i == j {
            println!("{}", value.sqrt());
            g.push(value.sqrt());
            continue;
        }
        if j > i {
            continue;
        }
        println!("{}", value);
        g.push((*value - g.iter().map(|el| el.powi(2)).sum::<f64>()));
    }
    println!("{:?}", g);
}
